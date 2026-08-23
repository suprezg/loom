/*
File Name: path_resolver.rs
Purpose: Implementation of the PathResolver helper component and path resolver models for the Loom compiler.
*/

#![allow(non_snake_case)]

use std::path::Path;

/**
 * Represents a resolved path containing both raw relative and canonical absolute formats.
 *
 * Members:
 * 	absolute: The fully canonicalized absolute path string.
 * 	relative: The original relative path string.
 */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoomPath
{
    pub absolute: String,
    pub relative: String,
}

/**
 * Enumeration of possible path resolution errors.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResolverError
{
    PathNotFound,
    CanonicalizationFailed,
    InvalidPathString,
    NotAFile,
}

/**
 * Checks if a path string is invalid or contains junk characters.
 *
 * Takes:
 * 	path: &str - The path string to check.
 *
 * Gives:
 * 	bool - True if the path is invalid or junk, false otherwise.
 */
fn isInvalidPathString(path: &str) -> bool
{
    path.is_empty() 
        || path.contains('\0') 
        || path.chars().any(|c| c.is_control() || "*?<>|\"".contains(c))
}

/**
 * Normalizes path separators to standard forward slashes and trims whitespace.
 *
 * Takes:
 * 	path: &str - The path string to normalize.
 *
 * Gives:
 * 	String - The normalized path string.
 */
fn normalizedPathSeparators(path: &str) -> String
{
    path.trim().replace('\\', "/")
}

/**
 * Validates whether a target path is well-formed, exists on disk, and is a file.
 *
 * Takes:
 * 	target: &str - The target file path.
 *
 * Gives:
 * 	Result<bool, ResolverError> - Ok(true) if exists and is a file, Ok(false) otherwise.
 */
pub fn validatePath(target: &str) -> Result<bool, ResolverError>
{
    if isInvalidPathString(target)
    {
        return Err(ResolverError::InvalidPathString);
    }
    
    let path = Path::new(target);
    if !path.exists()
    {
        return Err(ResolverError::PathNotFound);
    }
    if !path.is_file()
    {
        return Err(ResolverError::NotAFile);
    }
    Ok(true)
}

/**
 * Resolves baseDir and relativeTarget, performing canonicalization and checks.
 *
 * Takes:
 * 	baseDir: &str - The base directory.
 * 	relativeTarget: &str - The relative target path.
 *
 * Gives:
 * 	Result<LoomPath, ResolverError> - The resolved LoomPath struct or ResolverError.
 */
pub fn resolvePath(baseDir: &str, relativeTarget: &str) -> Result<LoomPath, ResolverError>
{
    if isInvalidPathString(baseDir) || isInvalidPathString(relativeTarget)
    {
        return Err(ResolverError::InvalidPathString);
    }
    
    let base_path = Path::new(baseDir);
    let joined = base_path.join(relativeTarget);
    
    match joined.canonicalize() {
        Ok(canonical) =>
        {
            if !canonical.is_file()
            {
                return Err(ResolverError::NotAFile);
            }
            let absolute_str = canonical.to_string_lossy().into_owned();
            Ok(LoomPath {
                absolute: normalizedPathSeparators(&absolute_str),
                relative: normalizedPathSeparators(relativeTarget),
            })
        }
        Err(err) =>
        {
            if err.kind() == std::io::ErrorKind::NotFound
            {
                Err(ResolverError::PathNotFound)
            }
            else
            {
                Err(ResolverError::CanonicalizationFailed)
            }
        }
    }
}

/**
 * Returns the parent directory containing the target file, falling back to '.' if empty.
 *
 * Takes:
 * 	filePath: &str - The file path to get the parent of.
 *
 * Gives:
 * 	Result<String, ResolverError> - The parent directory string or ResolverError.
 */
pub fn getParentDir(filePath: &str) -> Result<String, ResolverError>
{
    if isInvalidPathString(filePath)
    {
        return Err(ResolverError::InvalidPathString);
    }
    
    let path = Path::new(filePath);
    if !path.exists()
    {
        return Err(ResolverError::PathNotFound);
    }
    if !path.is_file()
    {
        return Err(ResolverError::NotAFile);
    }
    match path.parent() {
        Some(p) =>
        {
            let p_str = p.to_string_lossy().trim().to_string();
            let normalized = normalizedPathSeparators(&p_str);
            if normalized.is_empty()
            {
                Ok(String::from("."))
            }
            else
            {
                Ok(normalized)
            }
        }
        None =>
        {
            Ok(String::from("."))
        }
    }
}
