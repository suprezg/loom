/*
File Name: file_handler_tests.rs
Purpose: Integration tests for the FileHandler component verifying file payload extraction, path mapping, and error handling.
*/

#![allow(non_snake_case)]

use loom::helpers::file_handler::{giveFilePayload, IngestedPayload};

/*
Tests giveFilePayload with a valid thread specification file returning a correct IngestedPayload struct with absolute file path mapping.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testGiveFilePayloadSuccess() -> ()
{
    let path = "../examples/valid/authentication.thread";
    let result = giveFilePayload(path);
    assert!(result.is_ok());

    let payload: IngestedPayload = result.unwrap();
    assert!(!payload.threadContent.is_empty());
    assert_eq!(payload.threadFileMapping.len(), 1);
    assert!(payload.threadFileMapping[0].filePath.ends_with("examples/valid/authentication.thread"));
}

/*
Tests giveFilePayload with an invalid or non-existent file path returning an IngestError.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testGiveFilePayloadFailure() -> ()
{
    let path = "non_existent_file.txt";
    let result = giveFilePayload(path);
    assert!(result.is_err());
}
