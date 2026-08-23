/*
File Name: path_resolver_tests.rs
Purpose: Integration tests for the PathResolver component verifying path resolution and parent directory extraction.
*/

#![allow(non_snake_case)]

use loom::helpers::path_resolver::{
    getParentDir, resolvePath, LoomPath, ResolverError,
};

/*
Tests resolvePath with correct baseDir and correct relativeTarget.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testResolvePathSuccess() -> ()
{
    let result = resolvePath("helpers", "path_resolver.rs");
    assert!(result.is_ok());

    let loomPath: LoomPath = result.unwrap();
    assert_eq!(loomPath.relative, "path_resolver.rs");
    assert!(loomPath.absolute.ends_with("helpers/path_resolver.rs"));
}

/*
Tests resolvePath with correct baseDir and wrong relativeTarget.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testResolvePathWrongRelativeTarget() -> ()
{
    let result = resolvePath("helpers", "non_existent.rs");
    assert_eq!(result, Err(ResolverError::PathNotFound));
}

/*
Tests resolvePath with wrong baseDir and right relativeTarget.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testResolvePathWrongBaseDir() -> ()
{
    let result = resolvePath("non_existent_dir", "path_resolver.rs");
    assert_eq!(result, Err(ResolverError::PathNotFound));
}

/*
Tests resolvePath with wrong baseDir and wrong relativeTarget.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testResolvePathWrongBaseAndWrongTarget() -> ()
{
    let result = resolvePath("non_existent_dir", "non_existent.rs");
    assert_eq!(result, Err(ResolverError::PathNotFound));
}

/*
Tests resolvePath failure that results in CanonicalizationFailed.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testResolvePathCanonicalizationFailed() -> ()
{
    let result = resolvePath("Cargo.toml", "some_subpath");
    assert_eq!(result, Err(ResolverError::CanonicalizationFailed));
}

/*
Tests resolvePath with junk text.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testResolvePathWithJunkText() -> ()
{
    let result = resolvePath("invalid\0path", "path_resolver.rs");
    assert_eq!(result, Err(ResolverError::InvalidPathString));

    let result2 = resolvePath("helpers", "invalid\npath");
    assert_eq!(result2, Err(ResolverError::InvalidPathString));
}

/*
Tests resolvePath with a directory path as target (should return NotAFile error).

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testResolvePathWithDirectory() -> ()
{
    let result = resolvePath(".", "helpers");
    assert_eq!(result, Err(ResolverError::NotAFile));
}

/*
Tests getParentDir with a correct file path.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testGetParentDirSuccess() -> ()
{
    let result = getParentDir("helpers/path_resolver.rs");
    assert_eq!(result, Ok(String::from("helpers")));

    let result2 = getParentDir("Cargo.toml");
    assert_eq!(result2, Ok(String::from(".")));
}

/*
Tests getParentDir with an invalid/junk path string.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testGetParentDirWithJunkText() -> ()
{
    let result = getParentDir("invalid\0path");
    assert_eq!(result, Err(ResolverError::InvalidPathString));

    let result2 = getParentDir("");
    assert_eq!(result2, Err(ResolverError::InvalidPathString));
}

/*
Tests getParentDir with a directory path (should return NotAFile error).

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testGetParentDirWithDirectory() -> ()
{
    let result = getParentDir("helpers");
    assert_eq!(result, Err(ResolverError::NotAFile));
}
