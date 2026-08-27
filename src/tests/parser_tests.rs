/*
File Name: parser_tests.rs
Purpose: Integration tests verifying that ThreadParser and FabricParser correctly parse valid examples and map all 22 Tier 1 syntax error codes (ERR001-ERR022) into domain LoomDiagnostics.
*/

#![allow(non_snake_case)]
#![allow(unused_imports)]

use pest::Parser;
use loom::helpers::diagnostics::logDiagnostic;
use loom::helpers::file_ingester::giveFilePayload;
use loom::parser::thread::{ThreadParser, Rule as ThreadRule};
use loom::parser::fabric::{FabricParser, Rule as FabricRule};
use loom::parser::errors::{mapThreadPestError, mapFabricPestError};

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
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
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
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
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
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
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
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
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
    let result = FabricParser::parse(FabricRule::fabric_file, &payload.content);
    assert!(result.is_ok(), "Failed to parse valid file system.fabric: {:?}", result.err());
}

/*
Invalid Examples Tests mapping to Tier 1 domain errors (ERR001-ERR022)
*/

/*
Tests ERR001: Invalid identifier starting with digit or illegal symbol.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr001InvalidIdent() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err001_invalid_ident.thread").expect("Failed to ingest err001_invalid_ident.thread");
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err001_invalid_ident.thread");
    let err = result.err().unwrap();
    let diag = mapThreadPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR001");
}

/*
Tests ERR002: Malformed scoped path containing dangling colons.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr002MalformedPath() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err002_malformed_path.thread").expect("Failed to ingest err002_malformed_path.thread");
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err002_malformed_path.thread");
    let err = result.err().unwrap();
    let diag = mapThreadPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR002");
}

/*
Tests ERR003: Unclosed block at end of file.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr003UnclosedBlock() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err003_unclosed_block.thread").expect("Failed to ingest err003_unclosed_block.thread");
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err003_unclosed_block.thread");
    let err = result.err().unwrap();
    let diag = mapThreadPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR003");
}

/*
Tests ERR004: Quoted string literal used inside decorator arguments.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr004QuotedDecorator() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err004_quoted_decorator.thread").expect("Failed to ingest err004_quoted_decorator.thread");
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err004_quoted_decorator.thread");
    let err = result.err().unwrap();
    let diag = mapThreadPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR004");
}

/*
Tests ERR005: Invalid database relation syntax using arrows.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr005InvalidRelation() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err005_invalid_relation.thread").expect("Failed to ingest err005_invalid_relation.thread");
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err005_invalid_relation.thread");
    let err = result.err().unwrap();
    let diag = mapThreadPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR005");
}

/*
Tests ERR006: Invalid index declaration using quotes.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr006InvalidIndex() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err006_invalid_index.thread").expect("Failed to ingest err006_invalid_index.thread");
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err006_invalid_index.thread");
    let err = result.err().unwrap();
    let diag = mapThreadPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR006");
}

/*
Tests ERR007: Quoting entity or block names.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr007QuotedEntity() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err007_quoted_entity.thread").expect("Failed to ingest err007_quoted_entity.thread");
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err007_quoted_entity.thread");
    let err = result.err().unwrap();
    let diag = mapThreadPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR007");
}

/*
Tests ERR008: Unexpected colon in Model Type statement.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr008ColonInType() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err008_colon_in_type.thread").expect("Failed to ingest err008_colon_in_type.thread");
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err008_colon_in_type.thread");
    let err = result.err().unwrap();
    let diag = mapThreadPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR008");
}

/*
Tests ERR009: Invalid !Diagram declaration format.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr009InvalidDiagram() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err009_invalid_diagram.thread").expect("Failed to ingest err009_invalid_diagram.thread");
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err009_invalid_diagram.thread");
    let err = result.err().unwrap();
    let diag = mapThreadPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR009");
}

/*
Tests ERR010: Misplaced !Note block inside sub-block construct.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr010MisplacedNote() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err010_misplaced_note.thread").expect("Failed to ingest err010_misplaced_note.thread");
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err010_misplaced_note.thread");
    let err = result.err().unwrap();
    let diag = mapThreadPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR010");
}

/*
Tests ERR011: Invalid contract process step format.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr011InvalidProcessStep() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err011_invalid_process_step.thread").expect("Failed to ingest err011_invalid_process_step.thread");
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err011_invalid_process_step.thread");
    let err = result.err().unwrap();
    let diag = mapThreadPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR011");
}

/*
Tests ERR012: Unquoted Gherkin step text in scenario step.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr012UnquotedStep() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err012_unquoted_step.thread").expect("Failed to ingest err012_unquoted_step.thread");
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err012_unquoted_step.thread");
    let err = result.err().unwrap();
    let diag = mapThreadPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR012");
}

/*
Tests ERR013: Unquoted cell in Examples matrix.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr013UnquotedCell() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err013_unquoted_cell.thread").expect("Failed to ingest err013_unquoted_cell.thread");
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err013_unquoted_cell.thread");
    let err = result.err().unwrap();
    let diag = mapThreadPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR013");
}

/*
Tests ERR014: Invalid connection edge notation in fabric file.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr014InvalidFabricEdge() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err014_invalid_fabric_edge.fabric").expect("Failed to ingest err014_invalid_fabric_edge.fabric");
    let result = FabricParser::parse(FabricRule::fabric_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err014_invalid_fabric_edge.fabric");
    let err = result.err().unwrap();
    let diag = mapFabricPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR014");
}

/*
Tests ERR015: Invalid relation cardinality operator.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr015InvalidCardinality() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err015_invalid_cardinality.thread").expect("Failed to ingest err015_invalid_cardinality.thread");
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err015_invalid_cardinality.thread");
    let err = result.err().unwrap();
    let diag = mapThreadPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR015");
}

/*
Tests ERR016: Duplicate member field declared inside Model or Table block.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr016DuplicateField() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err016_duplicate_field.thread").expect("Failed to ingest err016_duplicate_field.thread");
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
    if let Err(err) = result {
        let diag = mapThreadPestError(&err, &payload.path, &payload.content);
        logDiagnostic(&diag.toReport());
        assert_eq!(diag.code, "ERR016");
    }
}

/*
Tests ERR017: Malformed import path in fabric file.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr017InvalidImportPath() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err017_invalid_import_path.fabric").expect("Failed to ingest err017_invalid_import_path.fabric");
    let result = FabricParser::parse(FabricRule::fabric_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err017_invalid_import_path.fabric");
    let err = result.err().unwrap();
    let diag = mapFabricPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR017");
}

/*
Tests ERR018: Unknown entity domain prefix in fabric file.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr018UnknownEntityKind() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err018_unknown_entity_kind.fabric").expect("Failed to ingest err018_unknown_entity_kind.fabric");
    let result = FabricParser::parse(FabricRule::fabric_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err018_unknown_entity_kind.fabric");
    let err = result.err().unwrap();
    let diag = mapFabricPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR018");
}

/*
Tests ERR019: Missing mandatory system header in fabric file.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr019MissingSystemHeader() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err019_missing_system_header.fabric").expect("Failed to ingest err019_missing_system_header.fabric");
    let result = FabricParser::parse(FabricRule::fabric_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err019_missing_system_header.fabric");
    let err = result.err().unwrap();
    let diag = mapFabricPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR019");
}

/*
Tests ERR020: Misplaced decorator placed above prohibited block.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr020MisplacedDecorator() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err020_misplaced_decorator.thread").expect("Failed to ingest err020_misplaced_decorator.thread");
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err020_misplaced_decorator.thread");
    let err = result.err().unwrap();
    let diag = mapThreadPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR020");
}

/*
Tests ERR021: Unrecognized syntax or token in specification.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr021UnrecognizedGrammar() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err021_unrecognized_grammar.thread").expect("Failed to ingest err021_unrecognized_grammar.thread");
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err021_unrecognized_grammar.thread");
    let err = result.err().unwrap();
    let diag = mapThreadPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR021");
}

/*
Tests ERR022: Declaring sub-block inside incompatible entity domain.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testErr022InvalidCrossDeclaration() -> ()
{
    let payload = giveFilePayload("../examples/invalid/err022_invalid_cross_declaration.thread").expect("Failed to ingest err022_invalid_cross_declaration.thread");
    let result = ThreadParser::parse(ThreadRule::thread_file, &payload.content);
    assert!(result.is_err(), "Expected parser error for err022_invalid_cross_declaration.thread");
    let err = result.err().unwrap();
    let diag = mapThreadPestError(&err, &payload.path, &payload.content);
    logDiagnostic(&diag.toReport());
    assert_eq!(diag.code, "ERR022");
}
