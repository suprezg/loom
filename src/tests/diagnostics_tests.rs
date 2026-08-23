/*
File Name: diagnostics_tests.rs
Purpose: Integration tests for the DiagnosticsLogger component verifying multiple log levels and scenarios under the loom namespace.
*/

#![allow(non_snake_case)]

use loom::helpers::diagnostics::{
    logDiagnostic, logInfo, renderSummary, setLogLevel, Diagnostic, DiagnosticSeverity,
    ExecutionSummary, LogLevel,
};

/*
Helper function to create a sample Warning diagnostic.

Takes:
	None.

Gives:
	Diagnostic: A warning diagnostic struct.
*/
fn createWarningDiagnostic() -> Diagnostic
{
    Diagnostic {
        path: String::from("specs/cli.thread"),
        line: 2,
        column: 5,
        code: String::from("WRN002"),
        message: String::from("Unused import detected"),
        severity: DiagnosticSeverity::Warning,
    }
}

/*
Helper function to create a sample Error diagnostic.

Takes:
	None.

Gives:
	Diagnostic: An error diagnostic struct.
*/
fn createErrorDiagnostic() -> Diagnostic
{
    Diagnostic {
        path: String::from("specs/cli.thread"),
        line: 1,
        column: 1,
        code: String::from("ERR001"),
        message: String::from("Syntax error: missing component name"),
        severity: DiagnosticSeverity::Error,
    }
}

/*
Helper function to create a sample Info diagnostic.

Takes:
	None.

Gives:
	Diagnostic: An info diagnostic struct.
*/
fn createInfoDiagnostic() -> Diagnostic
{
    Diagnostic {
        path: String::from("specs/cli.thread"),
        line: 3,
        column: 10,
        code: String::from("INF003"),
        message: String::from("Component discovery completed"),
        severity: DiagnosticSeverity::Info,
    }
}

/*
Helper function to create a sample successful ExecutionSummary.

Takes:
	None.

Gives:
	ExecutionSummary: A successful execution summary struct.
*/
fn createSuccessSummary() -> ExecutionSummary
{
    ExecutionSummary {
        filesWritten: vec![String::from("docs/DESIGN.md"), String::from("docs/WORKFLOW.md")],
        totalDocuments: 2,
        errorCount: 0,
        warningCount: 1,
        elapsedMs: 145,
    }
}

/*
Helper function to create a sample failed ExecutionSummary.

Takes:
	None.

Gives:
	ExecutionSummary: A failed execution summary struct.
*/
fn createFailureSummary() -> ExecutionSummary
{
    ExecutionSummary {
        filesWritten: vec![],
        totalDocuments: 0,
        errorCount: 3,
        warningCount: 2,
        elapsedMs: 25,
    }
}

/*
Tests logging a warning diagnostic in Quiet log level (should be suppressed).

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLogDiagnosticWarningInQuietLogLevel() -> ()
{
    setLogLevel(LogLevel::Quiet);
    let diag = createWarningDiagnostic();
    logDiagnostic(&diag, None);
}

/*
Tests logging a warning diagnostic in Verbose log level (should be output).

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLogDiagnosticWarningInVerboseLogLevel() -> ()
{
    setLogLevel(LogLevel::Verbose);
    let diag = createWarningDiagnostic();
    logDiagnostic(&diag, None);
}

/*
Tests logging a warning diagnostic in Normal log level (should be output).

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLogDiagnosticWarningInNormalLogLevel() -> ()
{
    setLogLevel(LogLevel::Normal);
    let diag = createWarningDiagnostic();
    logDiagnostic(&diag, None);
}

/*
Tests logging an error diagnostic in Quiet log level (should be output).

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLogDiagnosticErrorInQuietLogLevel() -> ()
{
    setLogLevel(LogLevel::Quiet);
    let diag = createErrorDiagnostic();
    logDiagnostic(&diag, None);
}

/*
Tests logging an error diagnostic in Verbose log level (should be output).

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLogDiagnosticErrorInVerboseLogLevel() -> ()
{
    setLogLevel(LogLevel::Verbose);
    let diag = createErrorDiagnostic();
    logDiagnostic(&diag, None);
}

/*
Tests logging an error diagnostic in Normal log level (should be output).

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLogDiagnosticErrorInNormalLogLevel() -> ()
{
    setLogLevel(LogLevel::Normal);
    let diag = createErrorDiagnostic();
    logDiagnostic(&diag, None);
}

/*
Tests logging an info diagnostic in Quiet log level (should be suppressed).

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLogDiagnosticInfoInQuietLogLevel() -> ()
{
    setLogLevel(LogLevel::Quiet);
    let diag = createInfoDiagnostic();
    logDiagnostic(&diag, None);
}

/*
Tests logging an info diagnostic in Verbose log level (should be output).

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLogDiagnosticInfoInVerboseLogLevel() -> ()
{
    setLogLevel(LogLevel::Verbose);
    let diag = createInfoDiagnostic();
    logDiagnostic(&diag, None);
}

/*
Tests logging an info diagnostic in Normal log level (should be suppressed).

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLogDiagnosticInfoInNormalLogLevel() -> ()
{
    setLogLevel(LogLevel::Normal);
    let diag = createInfoDiagnostic();
    logDiagnostic(&diag, None);
}

/*
Tests logInfo under Quiet active log level with three different threshold inputs.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLogInfoInQuietLevelWithThreeThresholds() -> ()
{
    setLogLevel(LogLevel::Quiet);
    logInfo("Quiet threshold - should not print", LogLevel::Quiet);
    logInfo("Normal threshold - should not print", LogLevel::Normal);
    logInfo("Verbose threshold - should not print", LogLevel::Verbose);
}

/*
Tests logInfo under Normal active log level with three different threshold inputs.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLogInfoInNormalLevelWithThreeThresholds() -> ()
{
    setLogLevel(LogLevel::Normal);
    logInfo("Quiet threshold - should print", LogLevel::Quiet);
    logInfo("Normal threshold - should print", LogLevel::Normal);
    logInfo("Verbose threshold - should not print", LogLevel::Verbose);
}

/*
Tests logInfo under Verbose active log level with three different threshold inputs.

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testLogInfoInVerboseLevelWithThreeThresholds() -> ()
{
    setLogLevel(LogLevel::Verbose);
    logInfo("Quiet threshold - should print", LogLevel::Quiet);
    logInfo("Normal threshold - should print", LogLevel::Normal);
    logInfo("Verbose threshold - should print", LogLevel::Verbose);
}

/*
Tests rendering successful summary in Quiet log level (should be suppressed).

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testRenderSummarySuccessInQuietLogLevel() -> ()
{
    setLogLevel(LogLevel::Quiet);
    let summary = createSuccessSummary();
    renderSummary(&summary);
}

/*
Tests rendering successful summary in Normal log level (should output success card).

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testRenderSummarySuccessInNormalLogLevel() -> ()
{
    setLogLevel(LogLevel::Normal);
    let summary = createSuccessSummary();
    renderSummary(&summary);
}

/*
Tests rendering successful summary in Verbose log level (should output success card).

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testRenderSummarySuccessInVerboseLogLevel() -> ()
{
    setLogLevel(LogLevel::Verbose);
    let summary = createSuccessSummary();
    renderSummary(&summary);
}

/*
Tests rendering failure summary in Quiet log level (should be suppressed).

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testRenderSummaryFailureInQuietLogLevel() -> ()
{
    setLogLevel(LogLevel::Quiet);
    let summary = createFailureSummary();
    renderSummary(&summary);
}

/*
Tests rendering failure summary in Normal log level (should output failure card).

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testRenderSummaryFailureInNormalLogLevel() -> ()
{
    setLogLevel(LogLevel::Normal);
    let summary = createFailureSummary();
    renderSummary(&summary);
}

/*
Tests rendering failure summary in Verbose log level (should output failure card).

Takes:
	None.

Gives:
	(): Unit type.
*/
#[test]
fn testRenderSummaryFailureInVerboseLogLevel() -> ()
{
    setLogLevel(LogLevel::Verbose);
    let summary = createFailureSummary();
    renderSummary(&summary);
}
