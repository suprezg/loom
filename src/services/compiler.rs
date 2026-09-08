/*
File Name: compiler.rs
Purpose: Compiler service implementation providing high-level build and validate pipeline workflows.
*/

#![allow(non_snake_case)]

use crate::helpers::diagnostics::{logDiagnostic, logMessage, LoomMessage};
use crate::helpers::file_handler::{giveFilePayload, writeFile};
use crate::helpers::document_composer::{composeThreadAst, composeFabricAst};
use crate::analysis::syntactic::{parseThread, parseFabric};
use crate::analysis::semantic::{checkThread, checkFabric};

/*
Validates Thread and Fabric specification files in an input directory or file for syntactic and semantic correctness.

Takes:
	inputDir (&str): Path to input file or directory containing specification files.

Gives:
	Result<(), String>: Ok(()) if validation succeeds without errors, or Err(String) on failure.
*/
pub fn validate(inputDir: &str) -> Result<(), String>
{
    logMessage(&LoomMessage::new(
        format!("Starting validation service for input '{}'", inputDir),
        miette::Severity::Advice,
    ));

    let payload = giveFilePayload(inputDir)?;

    if payload.threadContent.is_empty() && payload.fabricContent.is_none() {
        let errMsg = format!("No valid .thread or .fabric specifications found in '{}'", inputDir);
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
        format!("Validation service completed successfully for '{}'", inputDir),
        miette::Severity::Advice,
    ));

    Ok(())
}

/*
Builds and weaves Thread and Fabric specifications into JSON AST documentation files in the output directory.

Takes:
	inputDir (&str): Path to input file or directory containing specification files.
	outputDir (&str): Path to target output directory for generated JSON files.

Gives:
	Result<(), String>: Ok(()) if build and write succeeds without errors, or Err(String) on failure.
*/
pub fn build(inputDir: &str, outputDir: &str) -> Result<(), String>
{
    logMessage(&LoomMessage::new(
        format!("Starting build service for input '{}' to output '{}'", inputDir, outputDir),
        miette::Severity::Advice,
    ));

    let payload = giveFilePayload(inputDir)?;

    if payload.threadContent.is_empty() && payload.fabricContent.is_none() {
        let errMsg = format!("No valid .thread or .fabric specifications found in '{}'", inputDir);
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
