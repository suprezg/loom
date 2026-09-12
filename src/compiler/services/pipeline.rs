/*
File Name: pipeline.rs
Purpose: Compiler service implementation providing high-level build and validate pipeline workflows supporting multiple input paths.
*/

#![allow(non_snake_case)]

use crate::helpers::diagnostics::{logDiagnostic, logMessage, LoomMessage};
use crate::helpers::file_handler::{giveFilePayload, mergeFilePayloads, writeFile};
use crate::helpers::document_composer::{composeThreadAst, composeFabricAst};
use crate::analysis::syntactic::{parseThread, parseFabric};
use crate::analysis::semantic::{checkThread, checkFabric};

/*
Validates Thread and Fabric specification files across one or more input directories or files for syntactic and semantic correctness.

Takes:
	inputPaths (&[String]): List of paths to input files or directories containing specification files.

Gives:
	Result<(), String>: Ok(()) if validation succeeds without errors, or Err(String) on failure.
*/
pub fn validate(inputPaths: &[String]) -> Result<(), String>
{
    logMessage(&LoomMessage::new(
        format!("Starting validation service for {} input path(s)", inputPaths.len()),
        miette::Severity::Advice,
    ));

    if inputPaths.is_empty() {
        let errMsg = String::from("Validation failed: No input paths provided");
        logMessage(&LoomMessage::new(&errMsg, miette::Severity::Error));
        return Err(errMsg);
    }

    let mut payloads = Vec::new();
    for path in inputPaths {
        let payload = giveFilePayload(path)?;
        payloads.push(payload);
    }

    let payload = mergeFilePayloads(payloads)?;

    if payload.threadContent.is_empty() && payload.fabricContent.is_none() {
        let errMsg = String::from("No valid .thread or .fabric specifications found across input paths");
        logMessage(&LoomMessage::new(&errMsg, miette::Severity::Error));
        return Err(errMsg);
    }

    let mut threadPairsOpt = None;

    if !payload.threadContent.is_empty() {
        let threadPairs = match parseThread(&payload.threadContent, &payload.threadFileMapping) {
            Ok(pairs) => pairs,
            Err(diag) => {
                logDiagnostic(&diag.toReport());
                return Err(diag.message);
            }
        };

        match checkThread(&threadPairs, &payload.threadFileMapping) {
            Ok(warnings) => {
                for warn in &warnings {
                    logDiagnostic(&warn.toReport());
                }
            }
            Err(diag) => {
                logDiagnostic(&diag.toReport());
                return Err(diag.message);
            }
        }

        threadPairsOpt = Some(threadPairs);
    }

    if let Some(fabricContent) = &payload.fabricContent {
        let fabricPairs = match parseFabric(fabricContent, &payload.fabricFileMapping) {
            Ok(pairs) => pairs,
            Err(diag) => {
                logDiagnostic(&diag.toReport());
                return Err(diag.message);
            }
        };

        if let Some(threadPairs) = &threadPairsOpt {
            match checkFabric(threadPairs, &fabricPairs, &payload.fabricFileMapping) {
                Ok(warnings) => {
                    for warn in &warnings {
                        logDiagnostic(&warn.toReport());
                    }
                }
                Err(diag) => {
                    logDiagnostic(&diag.toReport());
                    return Err(diag.message);
                }
            }
        }
    }

    logMessage(&LoomMessage::new(
        format!("Validation service completed successfully for {} input path(s)", inputPaths.len()),
        miette::Severity::Advice,
    ));

    Ok(())
}

/*
Builds and weaves Thread and Fabric specifications across one or more input directories or files into JSON AST documentation files in the output directory.

Takes:
	inputPaths (&[String]): List of paths to input files or directories containing specification files.
	outputDir (&str): Path to target output directory for generated JSON files.

Gives:
	Result<(), String>: Ok(()) if build and write succeeds without errors, or Err(String) on failure.
*/
pub fn build(inputPaths: &[String], outputDir: &str) -> Result<(), String>
{
    logMessage(&LoomMessage::new(
        format!("Starting build service for {} input path(s) to output '{}'", inputPaths.len(), outputDir),
        miette::Severity::Advice,
    ));

    if inputPaths.is_empty() {
        let errMsg = String::from("Build failed: No input paths provided");
        logMessage(&LoomMessage::new(&errMsg, miette::Severity::Error));
        return Err(errMsg);
    }

    let mut payloads = Vec::new();
    for path in inputPaths {
        let payload = giveFilePayload(path)?;
        payloads.push(payload);
    }

    let payload = mergeFilePayloads(payloads)?;

    if payload.threadContent.is_empty() && payload.fabricContent.is_none() {
        let errMsg = String::from("No valid .thread or .fabric specifications found across input paths");
        logMessage(&LoomMessage::new(&errMsg, miette::Severity::Error));
        return Err(errMsg);
    }

    let mut threadPairsOpt = None;

    if !payload.threadContent.is_empty() {
        let threadPairs = match parseThread(&payload.threadContent, &payload.threadFileMapping) {
            Ok(pairs) => pairs,
            Err(diag) => {
                logDiagnostic(&diag.toReport());
                return Err(diag.message);
            }
        };

        match checkThread(&threadPairs, &payload.threadFileMapping) {
            Ok(warnings) => {
                for warn in &warnings {
                    logDiagnostic(&warn.toReport());
                }
            }
            Err(diag) => {
                logDiagnostic(&diag.toReport());
                return Err(diag.message);
            }
        }

        threadPairsOpt = Some(threadPairs);
    }

    let mut fabricPairsOpt = None;

    if let Some(fabricContent) = &payload.fabricContent {
        let fabricPairs = match parseFabric(fabricContent, &payload.fabricFileMapping) {
            Ok(pairs) => pairs,
            Err(diag) => {
                logDiagnostic(&diag.toReport());
                return Err(diag.message);
            }
        };

        if let Some(threadPairs) = &threadPairsOpt {
            match checkFabric(threadPairs, &fabricPairs, &payload.fabricFileMapping) {
                Ok(warnings) => {
                    for warn in &warnings {
                        logDiagnostic(&warn.toReport());
                    }
                }
                Err(diag) => {
                    logDiagnostic(&diag.toReport());
                    return Err(diag.message);
                }
            }
        }

        fabricPairsOpt = Some(fabricPairs);
    }

    let cleanOutDir = outputDir.trim_end_matches('/');

    if let Some(threadPairs) = &threadPairsOpt {
        let threadJson = composeThreadAst(threadPairs)
            .map_err(|_| "Failed to compose Thread JSON AST".to_string())?;
        let threadOutputPath = format!("{}/thread_ast.json", cleanOutDir);
        writeFile(&threadOutputPath, &threadJson)?;
    }

    if let Some(fabricPairs) = &fabricPairsOpt {
        let fabricJson = composeFabricAst(fabricPairs)
            .map_err(|_| "Failed to compose Fabric JSON AST".to_string())?;
        let fabricOutputPath = format!("{}/fabric_ast.json", cleanOutDir);
        writeFile(&fabricOutputPath, &fabricJson)?;
    }

    logMessage(&LoomMessage::new(
        format!("Build service completed successfully. Output written to '{}'", outputDir),
        miette::Severity::Advice,
    ));

    Ok(())
}
