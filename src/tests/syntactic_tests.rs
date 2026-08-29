/*
File Name: syntactic_tests.rs
Purpose: Integration tests verifying that parseThread and parseFabric correctly parse valid examples and map Tier 1 parser errors into domain LoomDiagnostics (LM0001-LM0003 for thread, LM1001-LM1003 for fabric).
*/

#![allow(non_snake_case)]
#![allow(unused_imports)]

use loom::helpers::diagnostics::logDiagnostic;
use loom::helpers::file_handler::giveFilePayload;
use loom::analysis::syntactic::{parseThread, parseFabric};

/*
Valid Examples Tests (5 files)
*/

/*
Tests parsing valid authentication.thread specification file.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testParseAuthenticationThread() -> ()
{
    let payload = giveFilePayload("../examples/valid/authentication.thread").expect("Failed to ingest authentication.thread");
    let result = parseThread(&payload.threadContent, &payload.threadFileMapping);
    assert!(result.is_ok(), "Failed to parse valid file authentication.thread: {:?}", result.err());
}

/*
Tests parsing valid auth_service.thread specification file.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testParseAuthServiceThread() -> ()
{
    let payload = giveFilePayload("../examples/valid/auth_service.thread").expect("Failed to ingest auth_service.thread");
    let result = parseThread(&payload.threadContent, &payload.threadFileMapping);
    assert!(result.is_ok(), "Failed to parse valid file auth_service.thread: {:?}", result.err());
}

/*
Tests parsing valid auth_protocol.thread specification file.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testParseAuthProtocolThread() -> ()
{
    let payload = giveFilePayload("../examples/valid/auth_protocol.thread").expect("Failed to ingest auth_protocol.thread");
    let result = parseThread(&payload.threadContent, &payload.threadFileMapping);
    assert!(result.is_ok(), "Failed to parse valid file auth_protocol.thread: {:?}", result.err());
}

/*
Tests parsing valid app_storage.thread specification file.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testParseAppStorageThread() -> ()
{
    let payload = giveFilePayload("../examples/valid/app_storage.thread").expect("Failed to ingest app_storage.thread");
    let result = parseThread(&payload.threadContent, &payload.threadFileMapping);
    assert!(result.is_ok(), "Failed to parse valid file app_storage.thread: {:?}", result.err());
}

/*
Tests parsing valid system.fabric macro architecture blueprint file.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testParseSystemFabric() -> ()
{
    let payload = giveFilePayload("../examples/valid/system.fabric").expect("Failed to ingest system.fabric");
    let fabricContent = payload.fabricContent.as_deref().unwrap_or("");
    let result = parseFabric(fabricContent, &payload.fabricFileMapping);
    assert!(result.is_ok(), "Failed to parse valid file system.fabric: {:?}", result.err());
}

/*
Invalid Examples Tests mapping to Tier 1 domain errors (LM0001-LM0003 and LM1001-LM1003)
*/

/*
Tests LM0001: Illegal identifier starting with digit or illegal symbol in thread specification.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLM0001IllegalIdent() -> ()
{
    let payload = giveFilePayload("../examples/invalid/lm0001_illegal_ident.thread").expect("Failed to ingest lm0001_illegal_ident.thread");
    let result = parseThread(&payload.threadContent, &payload.threadFileMapping);
    assert!(result.is_err(), "Expected parser error for lm0001_illegal_ident.thread");
    let diag = result.err().unwrap();
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "LM0001");
}

/*
Tests LM0002: Unclosed block at end of thread specification file.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLM0002UnclosedBlock() -> ()
{
    let payload = giveFilePayload("../examples/invalid/lm0002_unclosed_block.thread").expect("Failed to ingest lm0002_unclosed_block.thread");
    let result = parseThread(&payload.threadContent, &payload.threadFileMapping);
    assert!(result.is_err(), "Expected parser error for lm0002_unclosed_block.thread");
    let diag = result.err().unwrap();
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "LM0002");
}

/*
Tests LM0003: Malformed scoped path containing dangling colons in thread specification.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLM0003MalformedPath() -> ()
{
    let payload = giveFilePayload("../examples/invalid/lm0003_malformed_path.thread").expect("Failed to ingest lm0003_malformed_path.thread");
    let result = parseThread(&payload.threadContent, &payload.threadFileMapping);
    assert!(result.is_err(), "Expected parser error for lm0003_malformed_path.thread");
    let diag = result.err().unwrap();
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "LM0003");
}

/*
Tests LM1001: Illegal identifier starting with digit in fabric blueprint file.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLM1001IllegalIdent() -> ()
{
    let payload = giveFilePayload("../examples/invalid/lm1001_illegal_ident.fabric").expect("Failed to ingest lm1001_illegal_ident.fabric");
    let fabricContent = payload.fabricContent.as_deref().unwrap_or("");
    let result = parseFabric(fabricContent, &payload.fabricFileMapping);
    assert!(result.is_err(), "Expected parser error for lm1001_illegal_ident.fabric");
    let diag = result.err().unwrap();
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "LM1001");
}

/*
Tests LM1002: Unclosed block in fabric blueprint file.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLM1002UnclosedBlock() -> ()
{
    let payload = giveFilePayload("../examples/invalid/lm1002_unclosed_block.fabric").expect("Failed to ingest lm1002_unclosed_block.fabric");
    let fabricContent = payload.fabricContent.as_deref().unwrap_or("");
    let result = parseFabric(fabricContent, &payload.fabricFileMapping);
    assert!(result.is_err(), "Expected parser error for lm1002_unclosed_block.fabric");
    let diag = result.err().unwrap();
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "LM1002");
}

/*
Tests LM1003: Malformed path in fabric blueprint file.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLM1003MalformedPath() -> ()
{
    let payload = giveFilePayload("../examples/invalid/lm1003_malformed_path.fabric").expect("Failed to ingest lm1003_malformed_path.fabric");
    let fabricContent = payload.fabricContent.as_deref().unwrap_or("");
    let result = parseFabric(fabricContent, &payload.fabricFileMapping);
    assert!(result.is_err(), "Expected parser error for lm1003_malformed_path.fabric");
    let diag = result.err().unwrap();
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "LM1003");
}
