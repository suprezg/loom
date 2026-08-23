/*
File Name: file_ingester_tests.rs
Purpose: Integration tests for the FileIngester component verifying file payload extraction and error handling.
*/

#![allow(non_snake_case)]

use loom::helpers::file_ingester::{giveFilePayload, FilePayload, IngestError};

/*
Tests giveFilePayload with a valid file path returning a correct FilePayload struct.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testGiveFilePayloadSuccess() -> ()
{
    let path = "helpers/path_resolver.rs";
    let result = giveFilePayload(path);
    assert!(result.is_ok());

    let payload: FilePayload = result.unwrap();
    assert_eq!(payload.path, "helpers/path_resolver.rs");
    assert!(!payload.content.is_empty());
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
    assert_eq!(result, Err(IngestError::IoError));
}
