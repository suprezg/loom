/*
File Name: path_resolver_models.rs
Purpose: Data models for the path resolver component of the Loom compiler.
*/

#![allow(non_snake_case)]

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
