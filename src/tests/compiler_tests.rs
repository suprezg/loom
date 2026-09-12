/*
File Name: compiler_tests.rs
Purpose: Integration tests verifying the high-level build and validate services for valid and invalid specification suites.
*/

#![allow(non_snake_case)]

use std::fs;
use loom::helpers::diagnostics::{initLogLevel, LogLevel};
use loom::services::compiler::{build, validate};

/*
Tests that validate succeeds on the valid examples directory.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testValidateWithValidExamples() -> ()
{
    initLogLevel(LogLevel::Quiet);
    let paths = vec!["../examples/valid".to_string()];
    let result = validate(&paths);
    assert!(result.is_ok(), "Expected validate to succeed on valid examples");
}

/*
Tests that validate succeeds on multiple input specification files.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testValidateWithMultipleInputFiles() -> ()
{
    initLogLevel(LogLevel::Quiet);
    let paths = vec![
        "../examples/valid/app_storage.thread".to_string(),
        "../examples/valid/auth_protocol.thread".to_string(),
        "../examples/valid/auth_service.thread".to_string(),
        "../examples/valid/authentication.thread".to_string(),
    ];
    let result = validate(&paths);
    assert!(result.is_ok(), "Expected validate to succeed on multiple valid files");
}

/*
Tests that validate fails on an invalid example file.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testValidateWithInvalidExample() -> ()
{
    initLogLevel(LogLevel::Quiet);
    let paths = vec!["../examples/invalid/lm2001_unresolved_reference.thread".to_string()];
    let result = validate(&paths);
    assert!(result.is_err(), "Expected validate to fail on invalid example");
}

/*
Tests that build succeeds on the valid examples directory and generates output JSON AST files.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testBuildWithValidExamples() -> ()
{
    initLogLevel(LogLevel::Quiet);
    let outputDir = "../data/test_build_output";
    let _ = fs::remove_dir_all(outputDir);

    let paths = vec!["../examples/valid".to_string()];
    let result = build(&paths, outputDir);
    assert!(result.is_ok(), "Expected build to succeed on valid examples");

    let threadAstPath = format!("{}/thread_ast.json", outputDir);
    let fabricAstPath = format!("{}/fabric_ast.json", outputDir);

    assert!(fs::metadata(&threadAstPath).is_ok(), "thread_ast.json should be generated");
    assert!(fs::metadata(&fabricAstPath).is_ok(), "fabric_ast.json should be generated");

    let _ = fs::remove_dir_all(outputDir);
}

/*
Tests that build succeeds with multiple input files and directories.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testBuildWithMultipleInputFiles() -> ()
{
    initLogLevel(LogLevel::Quiet);
    let outputDir = "../data/test_multiple_build_output";
    let _ = fs::remove_dir_all(outputDir);

    let paths = vec![
        "../examples/valid/app_storage.thread".to_string(),
        "../examples/valid/auth_protocol.thread".to_string(),
        "../examples/valid/auth_service.thread".to_string(),
        "../examples/valid/authentication.thread".to_string(),
        "../examples/valid/system.fabric".to_string(),
    ];
    let result = build(&paths, outputDir);
    assert!(result.is_ok(), "Expected build to succeed on multiple valid files");

    let threadAstPath = format!("{}/thread_ast.json", outputDir);
    let fabricAstPath = format!("{}/fabric_ast.json", outputDir);

    assert!(fs::metadata(&threadAstPath).is_ok(), "thread_ast.json should be generated");
    assert!(fs::metadata(&fabricAstPath).is_ok(), "fabric_ast.json should be generated");

    let _ = fs::remove_dir_all(outputDir);
}

/*
Tests that build fails on an invalid example file.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testBuildWithInvalidExample() -> ()
{
    initLogLevel(LogLevel::Quiet);
    let outputDir = "../data/test_invalid_build_output";
    let paths = vec!["../examples/invalid/lm2002_duplicate_declaration.thread".to_string()];
    let result = build(&paths, outputDir);
    assert!(result.is_err(), "Expected build to fail on invalid example");
}
