/*
File Name: path_resolver.rs
Purpose: Implementation of the PathResolver helper component and path resolver models for the Loom compiler.
*/

#![allow(non_snake_case)]

use std::path::Path;
use crate::helpers::diagnostics::{logMessage, LoomMessage};

/*
Represents a resolved path containing both raw relative and canonical absolute formats.
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoomPath
{
    pub absolute: String,
    pub relative: String,
}

/*
Checks if a path string is invalid or contains junk characters.

Takes:
	path (&str): The path string to check.

Gives:
	bool: True if the path is invalid or junk, false otherwise.
*/
fn isInvalidPathString(path: &str) -> bool
{
    path.is_empty() 
        || path.contains('\0') 
        || path.chars().any(|c| c.is_control() || "*?<>|\"".contains(c))
}

/*
Normalizes path separators to standard forward slashes and trims whitespace.

Takes:
	path (&str): The path string to normalize.

Gives:
	String: The normalized path string.
*/
fn normalizedPathSeparators(path: &str) -> String
{
    path.trim().replace('\\', "/")
}

/*
Validates whether path strings are well-formed and checks single file existence and type when relativeTarget is None.

Takes:
	baseDir (&str): The base directory or single target file path.
	relativeTarget (Option<&str>): Optional relative target path when validating directory and target together.

Gives:
	Result<bool, String>: Ok(true) if valid, or Error string on failure.
*/
fn validatePath(baseDir: &str, relativeTarget: Option<&str>) -> Result<bool, String>
{
    logMessage(&LoomMessage::new(
        format!("Validating path structure for baseDir '{}'", baseDir),
        miette::Severity::Advice,
    ));

    if isInvalidPathString(baseDir) {
        let err = format!("Invalid path string: '{}' contains null bytes or control characters", baseDir);
        logMessage(&LoomMessage::new(&err, miette::Severity::Error));
        return Err(err);
    }

    if let Some(target) = relativeTarget {
        logMessage(&LoomMessage::new(
            format!("Validating relative target path string '{}'", target),
            miette::Severity::Advice,
        ));
        if isInvalidPathString(target) {
            let err = format!("Invalid relative target path string: '{}' contains invalid characters", target);
            logMessage(&LoomMessage::new(&err, miette::Severity::Error));
            return Err(err);
        }
    } else {
        let path = Path::new(baseDir);
        if !path.exists() {
            let err = format!("Path not found: '{}' does not exist", baseDir);
            logMessage(&LoomMessage::new(&err, miette::Severity::Error));
            return Err(err);
        }
        if !path.is_file() {
            let err = format!("Target path '{}' is a directory, expected a regular file", baseDir);
            logMessage(&LoomMessage::new(&err, miette::Severity::Error));
            return Err(err);
        }
    }

    logMessage(&LoomMessage::new(
        format!("Path validation successful for '{}'", baseDir),
        miette::Severity::Advice,
    ));

    Ok(true)
}

/*
Resolves baseDir and relativeTarget, performing canonicalization and checks.

Takes:
	baseDir (&str): The base directory.
	relativeTarget (&str): The relative target path.

Gives:
	Result<LoomPath, String>: The resolved LoomPath struct or Error string.
*/
pub fn resolvePath(baseDir: &str, relativeTarget: &str) -> Result<LoomPath, String>
{
    logMessage(&LoomMessage::new(
        format!("Resolving relative target path '{}' against base directory '{}'", relativeTarget, baseDir),
        miette::Severity::Advice,
    ));

    validatePath(baseDir, Some(relativeTarget))?;

    let basePath = Path::new(baseDir);
    let joined = basePath.join(relativeTarget);

    logMessage(&LoomMessage::new(
        format!("Attempting path canonicalization for joined path '{}'", joined.display()),
        miette::Severity::Advice,
    ));

    match joined.canonicalize() {
        Ok(canonical) => {
            if !canonical.is_file() {
                let err = format!("Canonicalized path '{}' is a directory, expected a regular file", canonical.display());
                logMessage(&LoomMessage::new(&err, miette::Severity::Error));
                return Err(err);
            }
            let absoluteStr = canonical.to_string_lossy().into_owned();
            let loomPath = LoomPath {
                absolute: normalizedPathSeparators(&absoluteStr),
                relative: normalizedPathSeparators(relativeTarget),
            };

            logMessage(&LoomMessage::new(
                format!("Successfully resolved canonical path: '{}'", loomPath.absolute),
                miette::Severity::Advice,
            ));

            Ok(loomPath)
        }
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                let errMsg = format!("Path resolution failed: Target path '{}' not found", joined.display());
                logMessage(&LoomMessage::new(&errMsg, miette::Severity::Error));
                Err(errMsg)
            } else {
                let errMsg = format!("Canonicalization failed for path '{}': {}", joined.display(), err);
                logMessage(&LoomMessage::new(&errMsg, miette::Severity::Error));
                Err(errMsg)
            }
        }
    }
}

/*
Returns the parent directory containing the target file, falling back to '.' if empty.

Takes:
	filePath (&str): The file path to get the parent of.

Gives:
	Result<String, String>: The parent directory string or Error string.
*/
pub fn getParentDir(filePath: &str) -> Result<String, String>
{
    logMessage(&LoomMessage::new(
        format!("Extracting parent directory for file path '{}'", filePath),
        miette::Severity::Advice,
    ));

    validatePath(filePath, None)?;

    let path = Path::new(filePath);
    match path.parent() {
        Some(p) => {
            let pStr = p.to_string_lossy().trim().to_string();
            let normalized = normalizedPathSeparators(&pStr);
            let parentDir = if normalized.is_empty() {
                String::from(".")
            } else {
                normalized
            };

            logMessage(&LoomMessage::new(
                format!("Extracted parent directory '{}' for file '{}'", parentDir, filePath),
                miette::Severity::Advice,
            ));

            Ok(parentDir)
        }
        None => {
            logMessage(&LoomMessage::new(
                format!("No explicit parent directory found for '{}', defaulting to '.'", filePath),
                miette::Severity::Advice,
            ));
            Ok(String::from("."))
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    /*
    Tests isInvalidPathString returning false for a valid path string.

    Takes:
    	None.

    Gives:
    	(): Unit type.
    */
    #[test]
    fn testIsInvalidPathStringReturnsFalse() -> ()
    {
        let result = isInvalidPathString("valid/path.rs");
        assert_eq!(result, false);
    }

    /*
    Tests isInvalidPathString returning true for an invalid path string.

    Takes:
    	None.

    Gives:
    	(): Unit type.
    */
    #[test]
    fn testIsInvalidPathStringReturnsTrue() -> ()
    {
        let result = isInvalidPathString("invalid\0path");
        assert_eq!(result, true);
    }

    /*
    Tests normalizedPathSeparators returning a properly normalized path string.

    Takes:
    	None.

    Gives:
    	(): Unit type.
    */
    #[test]
    fn testNormalizedPathSeparators() -> ()
    {
        let result = normalizedPathSeparators("  helpers\\path_resolver.rs  ");
        assert_eq!(result, "helpers/path_resolver.rs");
    }

    /*
    Tests validatePath with a valid existing file.

    Takes:
    	None.

    Gives:
    	(): Unit type.
    */
    #[test]
    fn testValidatePathWithValidFile() -> ()
    {
        let result = validatePath("Cargo.toml", None);
        assert_eq!(result, Ok(true));
    }

    /*
    Tests validatePath with a valid path to a non-existent file.

    Takes:
    	None.

    Gives:
    	(): Unit type.
    */
    #[test]
    fn testValidatePathWithNonExistentFile() -> ()
    {
        let result = validatePath("non_existent_file.txt", None);
        assert!(result.is_err());
    }

    /*
    Tests validatePath with an invalid/junk path string.

    Takes:
    	None.

    Gives:
    	(): Unit type.
    */
    #[test]
    fn testValidatePathWithJunkText() -> ()
    {
        let result = validatePath("invalid\0path", None);
        assert!(result.is_err());
    }

    /*
    Tests validatePath with a directory path (should return Error when target is None).

    Takes:
    	None.

    Gives:
    	(): Unit type.
    */
    #[test]
    fn testValidatePathWithDirectory() -> ()
    {
        let result = validatePath("helpers", None);
        assert!(result.is_err());
    }

    /*
    Tests validatePath with baseDir and target together.

    Takes:
    	None.

    Gives:
    	(): Unit type.
    */
    #[test]
    fn testValidatePathWithBaseDirAndTarget() -> ()
    {
        let result = validatePath("helpers", Some("path_resolver.rs"));
        assert_eq!(result, Ok(true));
    }
}
