/*
File Name: path_resolver.rs
Purpose: Implementation of the PathResolver helper component and path resolver models for the Loom compiler.
*/

#![allow(non_snake_case)]

use std::path::Path;

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
Enumeration of possible path resolution errors.
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResolverError
{
    PathNotFound,
    CanonicalizationFailed,
    InvalidPathString,
    NotAFile,
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
	Result<bool, ResolverError>: Ok(true) if valid, or appropriate ResolverError on failure.
*/
fn validatePath(baseDir: &str, relativeTarget: Option<&str>) -> Result<bool, ResolverError>
{
    if isInvalidPathString(baseDir) {
        return Err(ResolverError::InvalidPathString);
    }

    if let Some(target) = relativeTarget {
        if isInvalidPathString(target) {
            return Err(ResolverError::InvalidPathString);
        }
    } else {
        let path = Path::new(baseDir);
        if !path.exists() {
            return Err(ResolverError::PathNotFound);
        }
        if !path.is_file() {
            return Err(ResolverError::NotAFile);
        }
    }

    Ok(true)
}

/*
Resolves baseDir and relativeTarget, performing canonicalization and checks.

Takes:
	baseDir (&str): The base directory.
	relativeTarget (&str): The relative target path.

Gives:
	Result<LoomPath, ResolverError>: The resolved LoomPath struct or ResolverError.
*/
pub fn resolvePath(baseDir: &str, relativeTarget: &str) -> Result<LoomPath, ResolverError>
{
    validatePath(baseDir, Some(relativeTarget))?;

    let basePath = Path::new(baseDir);
    let joined = basePath.join(relativeTarget);

    match joined.canonicalize() {
        Ok(canonical) => {
            if !canonical.is_file() {
                return Err(ResolverError::NotAFile);
            }
            let absoluteStr = canonical.to_string_lossy().into_owned();
            Ok(LoomPath {
                absolute: normalizedPathSeparators(&absoluteStr),
                relative: normalizedPathSeparators(relativeTarget),
            })
        }
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                Err(ResolverError::PathNotFound)
            } else {
                Err(ResolverError::CanonicalizationFailed)
            }
        }
    }
}

/*
Returns the parent directory containing the target file, falling back to '.' if empty.

Takes:
	filePath (&str): The file path to get the parent of.

Gives:
	Result<String, ResolverError>: The parent directory string or ResolverError.
*/
pub fn getParentDir(filePath: &str) -> Result<String, ResolverError>
{
    validatePath(filePath, None)?;

    let path = Path::new(filePath);
    match path.parent() {
        Some(p) => {
            let pStr = p.to_string_lossy().trim().to_string();
            let normalized = normalizedPathSeparators(&pStr);
            if normalized.is_empty() {
                Ok(String::from("."))
            } else {
                Ok(normalized)
            }
        }
        None => {
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
        assert_eq!(result, Err(ResolverError::PathNotFound));
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
        assert_eq!(result, Err(ResolverError::InvalidPathString));
    }

    /*
    Tests validatePath with a directory path (should return NotAFile error when target is None).

    Takes:
    	None.

    Gives:
    	(): Unit type.
    */
    #[test]
    fn testValidatePathWithDirectory() -> ()
    {
        let result = validatePath("helpers", None);
        assert_eq!(result, Err(ResolverError::NotAFile));
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
