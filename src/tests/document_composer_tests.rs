/*
File Name: document_composer_tests.rs
Purpose: End-to-end integration test ingesting valid thread and fabric specifications, resolving file paths using LoomPathResolver, parsing ASTs, executing semantic analysis, composing structured JSON representations, and writing JSON files to the output data directory using FileHandler.
*/

#![allow(non_snake_case)]

use loom::helpers::diagnostics::{logDiagnostic, initLogLevel, LogLevel};
use loom::helpers::file_handler::{giveFilePayload, writeFile};
use loom::helpers::path_resolver::{resolvePath, getParentDir};
use loom::helpers::document_composer::{composeThreadAst, composeFabricAst};
use loom::analysis::syntactic::{parseThread, parseFabric};
use loom::analysis::semantic::{checkThread, checkFabric};

/*
End-to-end integration test ingesting valid examples, resolving paths via PathResolver, analyzing ASTs, composing structured JSON documents, and writing output JSON files to /data using FileHandler.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testProduceJsonFromValidExamples() -> ()
{
    let inputDir = "../examples/valid";
    let outputDir = "../data";
    let threadJsonFileName = "thread_ast.json";
    let fabricJsonFileName = "fabric_ast.json";

    initLogLevel(LogLevel::Normal);

    let payload = giveFilePayload(inputDir).expect("Failed to ingest valid examples directory");
    let fabricContent = payload.fabricContent.as_deref().unwrap_or("");

    assert!(!payload.threadContent.is_empty(), "Thread source content should not be empty");
    assert!(!fabricContent.is_empty(), "Fabric source content should not be empty");

    /* Single Syntax Parse for Thread AST */
    let threadPairs = match parseThread(&payload.threadContent, &payload.threadFileMapping) {
        Ok(pairs) => pairs,
        Err(diag) => {
            logDiagnostic(&diag.toReport());
            panic!("Thread AST syntax parsing failed");
        }
    };

    /* Single Syntax Parse for Fabric AST */
    let fabricPairs = match parseFabric(fabricContent, &payload.fabricFileMapping) {
        Ok(pairs) => pairs,
        Err(diag) => {
            logDiagnostic(&diag.toReport());
            panic!("Fabric AST syntax parsing failed");
        }
    };

    /* Execute Thread Semantic Analysis with File Span Mappings */
    let threadSemanticResult = checkThread(&threadPairs, &payload.threadFileMapping);
    if let Err(report) = threadSemanticResult {
        logDiagnostic(&report);
        panic!("Thread semantic analysis error occurred");
    }
    if let Ok(warnings) = threadSemanticResult {
        for warn in &warnings {
            logDiagnostic(warn);
        }
    }

    /* Execute Fabric Cross-Referencing Semantic Analysis by Reference */
    let fabricSemanticResult = checkFabric(&threadPairs, &fabricPairs, &payload.fabricFileMapping);
    if let Err(report) = fabricSemanticResult {
        logDiagnostic(&report);
        panic!("Fabric semantic analysis error occurred");
    }
    if let Ok(warnings) = fabricSemanticResult {
        for warn in &warnings {
            logDiagnostic(warn);
        }
    }

    /* Compose JSON Documents from AST Pairs References */
    let threadJson = composeThreadAst(&threadPairs).expect("Failed to compose thread JSON AST");
    let fabricJson = composeFabricAst(&fabricPairs).expect("Failed to compose fabric JSON AST");

    /* Write Output JSON Files */
    let threadOutputPath = format!("{}/{}", outputDir, threadJsonFileName);
    let fabricOutputPath = format!("{}/{}", outputDir, fabricJsonFileName);

    writeFile(&threadOutputPath, &threadJson).expect("Failed to write thread JSON to data directory");
    writeFile(&fabricOutputPath, &fabricJson).expect("Failed to write fabric JSON to data directory");

    /* Verify written output JSON files using getParentDir and resolvePath */
    let threadOutParent = getParentDir(&threadOutputPath).expect("Failed to get parent directory for output thread JSON");
    let resolvedThreadOut = resolvePath(&threadOutParent, threadJsonFileName).expect("Failed to resolve output thread JSON LoomPath");
    assert!(!resolvedThreadOut.absolute.is_empty(), "thread_ast.json absolute path should not be empty");

    let fabricOutParent = getParentDir(&fabricOutputPath).expect("Failed to get parent directory for output fabric JSON");
    let resolvedFabricOut = resolvePath(&fabricOutParent, fabricJsonFileName).expect("Failed to resolve output fabric JSON LoomPath");
    assert!(!resolvedFabricOut.absolute.is_empty(), "fabric_ast.json absolute path should not be empty");
}
