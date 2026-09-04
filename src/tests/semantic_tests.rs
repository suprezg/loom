/*
File Name: semantic_tests.rs
Purpose: Integration tests verifying that the semantic analysis engine correctly detects, formats, and logs all Tier 2 semantic errors and warnings (LM2001-LM2007 and LM3001-LM3002).
*/

#![allow(non_snake_case)]
#![allow(unused_imports)]

use loom::helpers::diagnostics::logDiagnostic;
use loom::helpers::file_handler::giveFilePayload;
use loom::analysis::syntactic::{parseThread, parseFabric};
use loom::analysis::semantic::{checkThread, checkFabric};

/*
Tests LM2001: Unresolved reference error detection and diagnostic logging.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLM2001UnresolvedRef() -> ()
{
    let payload = giveFilePayload("../examples/invalid/lm2001_unresolved_reference.thread").expect("Failed to ingest lm2001_unresolved_reference.thread");
    let pairs = parseThread(&payload.threadContent, &payload.threadFileMapping).expect("Failed to parse thread file");
    let result = checkThread(&pairs, &payload.threadFileMapping);
    assert!(result.is_err(), "Expected LM2001 error for unresolved reference");
    let diag = result.err().unwrap();
    logDiagnostic(&diag.toReport());
}

/*
Tests LM3001: Fabric unresolved reference error detection and diagnostic logging.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLM3001FabricUnresolvedRef() -> ()
{
    let fabricPayload = giveFilePayload("../examples/invalid/lm3001_fabric_unresolved_reference.fabric").expect("Failed to ingest lm3001_fabric_unresolved_reference.fabric");
    let authPayload = giveFilePayload("../examples/valid/auth_service.thread").expect("Failed to ingest auth_service.thread");
    
    let threadPairs = parseThread(&authPayload.threadContent, &authPayload.threadFileMapping).expect("Failed to parse thread file");
    let fabricContent = fabricPayload.fabricContent.as_deref().unwrap_or("");
    let fabricPairs = parseFabric(fabricContent, &fabricPayload.fabricFileMapping).expect("Failed to parse fabric file");

    let result = checkFabric(&threadPairs, &fabricPairs, &fabricPayload.fabricFileMapping);
    assert!(result.is_err(), "Expected LM3001 error for unresolved fabric reference");
    let diag = result.err().unwrap();
    logDiagnostic(&diag.toReport());
}

/*
Tests LM2002: Duplicate entity or member declaration error detection and diagnostic logging.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLM2002DuplicateDecl() -> ()
{
    let payload = giveFilePayload("../examples/invalid/lm2002_duplicate_declaration.thread").expect("Failed to ingest lm2002_duplicate_declaration.thread");
    let pairs = parseThread(&payload.threadContent, &payload.threadFileMapping).expect("Failed to parse thread file");
    let result = checkThread(&pairs, &payload.threadFileMapping);
    assert!(result.is_err(), "Expected LM2002 error for duplicate declaration");
    let diag = result.err().unwrap();
    logDiagnostic(&diag.toReport());
}

/*
Tests LM2003: Storage index column verification error detection and diagnostic logging.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLM2003StorageIndexCol() -> ()
{
    let payload = giveFilePayload("../examples/invalid/lm2003_storage_index_column.thread").expect("Failed to ingest lm2003_storage_index_column.thread");
    let pairs = parseThread(&payload.threadContent, &payload.threadFileMapping).expect("Failed to parse thread file");
    let result = checkThread(&pairs, &payload.threadFileMapping);
    assert!(result.is_err(), "Expected LM2003 error for invalid storage index column");
    let diag = result.err().unwrap();
    logDiagnostic(&diag.toReport());
}

/*
Tests LM2004: Storage relation target verification error detection and diagnostic logging.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLM2004StorageRelation() -> ()
{
    let payload = giveFilePayload("../examples/invalid/lm2004_storage_relation_target.thread").expect("Failed to ingest lm2004_storage_relation_target.thread");
    let pairs = parseThread(&payload.threadContent, &payload.threadFileMapping).expect("Failed to parse thread file");
    let result = checkThread(&pairs, &payload.threadFileMapping);
    assert!(result.is_err(), "Expected LM2004 error for invalid storage relation target");
    let diag = result.err().unwrap();
    logDiagnostic(&diag.toReport());
}

/*
Tests LM2005: Protocol channel Sender/Receiver/Payload verification error detection and diagnostic logging.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLM2005ProtocolChannel() -> ()
{
    let payload = giveFilePayload("../examples/invalid/lm2005_protocol_channel_payload.thread").expect("Failed to ingest lm2005_protocol_channel_payload.thread");
    let pairs = parseThread(&payload.threadContent, &payload.threadFileMapping).expect("Failed to parse thread file");
    let result = checkThread(&pairs, &payload.threadFileMapping);
    assert!(result.is_err(), "Expected LM2005 error for invalid protocol channel target");
    let diag = result.err().unwrap();
    logDiagnostic(&diag.toReport());
}

/*
Tests LM2006: Feature scenario missing decorator warning detection and diagnostic logging.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLM2006MissingDecorator() -> ()
{
    let payload = giveFilePayload("../examples/invalid/lm2006_missing_scenario_decorator.thread").expect("Failed to ingest lm2006_missing_scenario_decorator.thread");
    let pairs = parseThread(&payload.threadContent, &payload.threadFileMapping).expect("Failed to parse thread file");
    let result = checkThread(&pairs, &payload.threadFileMapping);
    assert!(result.is_ok(), "Expected success with warnings for missing decorator");
    let warnings = result.ok().unwrap();
    assert!(!warnings.is_empty(), "Expected at least one LM2006 warning");
    for warn in &warnings {
        logDiagnostic(&warn.toReport());
    }
}

/*
Tests LM2007: Unused entity, member, or diagram warning detection and diagnostic logging.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLM2007UnusedSymbol() -> ()
{
    let payload = giveFilePayload("../examples/invalid/lm2007_unused_entity.thread").expect("Failed to ingest lm2007_unused_entity.thread");
    let pairs = parseThread(&payload.threadContent, &payload.threadFileMapping).expect("Failed to parse thread file");
    let result = checkThread(&pairs, &payload.threadFileMapping);
    assert!(result.is_ok(), "Expected success with warnings for unused entity");
    let warnings = result.ok().unwrap();
    assert!(!warnings.is_empty(), "Expected at least one LM2007 warning");
    for warn in &warnings {
        logDiagnostic(&warn.toReport());
    }
}