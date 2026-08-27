/*
File Name: diagnostics_tests.rs
Purpose: Integration tests for the DiagnosticsLogger component verifying miette diagnostic logging, logInfo, and renderSummary when log level is initialized to Quiet.
*/

#![allow(non_snake_case)]

use loom::helpers::diagnostics::{
    initLogLevel, logDiagnostic, logInfo, renderSummary, ExecutionSummary, LogLevel, LoomDiagnostic,
};

/*
Helper function to create three sample miette diagnostic reports (Info/Advice, Warning, and Error) using LoomDiagnostic::new.

Takes:
	None.

Gives:
	(miette::Report, miette::Report, miette::Report): A tuple containing Info, Warning, and Error diagnostic reports.
*/
fn createSampleDiagnostics() -> (miette::Report, miette::Report, miette::Report)
{
    let sampleSource = String::from("Feature Authentication {\n    Component 123Service {}\n}");

    let infoDiag = LoomDiagnostic::new(
        "specs/auth.thread",
        sampleSource.clone(),
        0,
        22,
        String::from("Feature block initialized"),
        Some(String::from("Feature capabilities ready for processing")),
        String::from("INF001"),
        String::from("Component discovery completed"),
        miette::Severity::Advice,
    );

    let warningDiag = LoomDiagnostic::new(
        "specs/auth.thread",
        sampleSource.clone(),
        29,
        10,
        String::from("Unused component declaration"),
        Some(String::from("Remove unreferenced component to clean up specification")),
        String::from("WRN001"),
        String::from("Unused import detected"),
        miette::Severity::Warning,
    );

    let errorDiag = LoomDiagnostic::new(
        "specs/auth.thread",
        sampleSource.clone(),
        39,
        10,
        String::from("Invalid identifier starting with digit"),
        Some(String::from("Identifiers must start with [a-zA-Z_]")),
        String::from("ERR001"),
        String::from("Syntax error: invalid identifier '123Service'"),
        miette::Severity::Error,
    );

    (
        infoDiag.toReport(),
        warningDiag.toReport(),
        errorDiag.toReport(),
    )
}

/*
Tests logging Info, Warning, and Error diagnostics, logInfo, and renderSummary after initializing log level to Quiet.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLogDiagnosticsInQuietLogLevel() -> ()
{
    initLogLevel(LogLevel::Quiet);
    let (infoReport, warningReport, errorReport) = createSampleDiagnostics();

    logDiagnostic(&infoReport);
    logDiagnostic(&warningReport);
    logDiagnostic(&errorReport);

    logInfo("Test info message - should be suppressed in quiet mode");

    let summary = ExecutionSummary {
        filesWritten: vec![String::from("docs/DESIGN.md")],
        totalDocuments: 1,
        errorCount: 0,
        warningCount: 0,
        elapsedMs: 10,
    };
    renderSummary(&summary);
}
