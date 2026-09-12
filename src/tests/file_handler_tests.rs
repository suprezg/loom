/*
File Name: file_handler_tests.rs
Purpose: Integration tests for the FileHandler component verifying file payload extraction, path mapping, and error handling.
*/

#![allow(non_snake_case)]

use loom::helpers::file_handler::{giveFilePayload, mergeFilePayloads, IngestedPayload};

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

/*
Tests mergeFilePayloads with multiple valid payloads merging thread and fabric content and span offsets correctly.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testMergePayloadSuccess() -> ()
{
    let p1 = giveFilePayload("../examples/valid/authentication.thread").expect("Failed to ingest authentication.thread");
    let p2 = giveFilePayload("../examples/valid/auth_service.thread").expect("Failed to ingest auth_service.thread");

    let result = mergeFilePayloads(vec![p1, p2]);
    assert!(result.is_ok());

    let merged = result.unwrap();
    assert_eq!(merged.threadFileMapping.len(), 2);
    assert_eq!(merged.threadFileMapping[0].startOffset, 0);
    assert!(merged.threadFileMapping[1].startOffset > 0);
    assert_eq!(merged.threadFileMapping[1].startOffset, merged.threadFileMapping[0].endOffset);
}

/*
Tests mergeFilePayloads failure with empty payload list or duplicate files.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testMergePayloadFailure() -> ()
{
    let emptyResult = mergeFilePayloads(vec![]);
    assert!(emptyResult.is_err());

    let p1 = giveFilePayload("../examples/valid/authentication.thread").expect("Failed to ingest authentication.thread");
    let duplicateResult = mergeFilePayloads(vec![p1.clone(), p1]);
    assert!(duplicateResult.is_err());
}
