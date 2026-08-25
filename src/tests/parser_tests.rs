/*
File Name: parser_tests.rs
Purpose: Integration tests verifying that ThreadParser and FabricParser correctly parse all .thread and .fabric specification examples from examples/.
*/

#![allow(non_snake_case)]

use pest::Parser;
use loom::parser::thread::{ThreadParser, Rule as ThreadRule};
use loom::parser::fabric::{FabricParser, Rule as FabricRule};

fn readFileContent(relativePath: &str) -> String
{
    std::fs::read_to_string(format!("../{}", relativePath))
        .or_else(|_| std::fs::read_to_string(relativePath))
        .unwrap_or_else(|err| panic!("Failed to read spec file '{}': {}", relativePath, err))
}

/*
Tests parsing of examples/authentication.thread (Feature entity) using ThreadParser.
*/
#[test]
fn testParseAuthenticationThread() -> ()
{
    let content = readFileContent("examples/authentication.thread");
    let parseResult = ThreadParser::parse(ThreadRule::thread_file, &content);
    assert!(
        parseResult.is_ok(),
        "Failed to parse authentication.thread: {:?}",
        parseResult.err()
    );
    let pairs = parseResult.unwrap();
    assert!(pairs.count() > 0);
}

/*
Tests parsing of examples/auth_service.thread (Component entity) using ThreadParser.
*/
#[test]
fn testParseAuthServiceThread() -> ()
{
    let content = readFileContent("examples/auth_service.thread");
    let parseResult = ThreadParser::parse(ThreadRule::thread_file, &content);
    assert!(
        parseResult.is_ok(),
        "Failed to parse auth_service.thread: {:?}",
        parseResult.err()
    );
    let pairs = parseResult.unwrap();
    assert!(pairs.count() > 0);
}

/*
Tests parsing of examples/auth_protocol.thread (Protocol entity) using ThreadParser.
*/
#[test]
fn testParseAuthProtocolThread() -> ()
{
    let content = readFileContent("examples/auth_protocol.thread");
    let parseResult = ThreadParser::parse(ThreadRule::thread_file, &content);
    assert!(
        parseResult.is_ok(),
        "Failed to parse auth_protocol.thread: {:?}",
        parseResult.err()
    );
    let pairs = parseResult.unwrap();
    assert!(pairs.count() > 0);
}

/*
Tests parsing of examples/app_storage.thread (Storage entity) using ThreadParser.
*/
#[test]
fn testParseAppStorageThread() -> ()
{
    let content = readFileContent("examples/app_storage.thread");
    let parseResult = ThreadParser::parse(ThreadRule::thread_file, &content);
    assert!(
        parseResult.is_ok(),
        "Failed to parse app_storage.thread: {:?}",
        parseResult.err()
    );
    let pairs = parseResult.unwrap();
    assert!(pairs.count() > 0);
}

/*
Tests parsing of examples/system.fabric (.fabric file) using FabricParser.
*/
#[test]
fn testParseSystemFabric() -> ()
{
    let content = readFileContent("examples/system.fabric");
    let parseResult = FabricParser::parse(FabricRule::fabric_file, &content);
    assert!(
        parseResult.is_ok(),
        "Failed to parse system.fabric: {:?}",
        parseResult.err()
    );
    let pairs = parseResult.unwrap();
    assert!(pairs.count() > 0);
}
