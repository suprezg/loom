/*
File Name: diagnostics.rs
Purpose: Implementation of the DiagnosticsLogger helper component using LoomDiagnostic, logInfo, renderSummary, and miette diagnostic dispatching.
*/

#![allow(non_snake_case)]

use std::sync::OnceLock;
use std::io::Write;
use std::format;

/*
Verbosity level of the logger to control which levels of details are output to standard streams.
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel
{
    Quiet,
    Normal,
    Verbose,
}

/*
Represents a unified Loom diagnostic error, warning, or informational report implementing miette::Diagnostic.
*/
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{message}")]
pub struct LoomDiagnostic
{
    pub src: miette::NamedSource<String>,
    pub startOffset: usize,
    pub length: usize,
    pub labelText: String,
    pub helpText: Option<String>,
    pub code: String,
    pub message: String,
    pub severityLevel: miette::Severity,
}

impl miette::Diagnostic for LoomDiagnostic
{
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>>
    {
        Some(Box::new(self.code.clone()) as Box<dyn std::fmt::Display>)
    }

    fn severity(&self) -> Option<miette::Severity>
    {
        Some(self.severityLevel)
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode>
    {
        Some(&self.src)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>>
    {
        let span = miette::SourceSpan::new(self.startOffset.into(), self.length);
        Some(Box::new(std::iter::once(
            miette::LabeledSpan::new_with_span(
                Some(self.labelText.clone()),
                span,
            ),
        )))
    }

    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>>
    {
        self.helpText.as_ref().map(|h| Box::new(h.as_str()) as Box<dyn std::fmt::Display>)
    }
}

impl LoomDiagnostic
{
    /*
    Creates a new LoomDiagnostic instance with full source code context, labels, and help message.

    Takes:
    	path (&str): The file path string.
    	sourceCode (String): The raw source code content string.
    	startOffset (usize): The starting byte or character offset in the source.
    	length (usize): The length of the span.
    	labelText (String): The span annotation label string.
    	helpText (Option<String>): Optional remediation advice string.
    	code (String): The error or diagnostic code string.
    	message (String): The main explanation message string.
    	severityLevel (miette::Severity): The severity level of the diagnostic.

    Gives:
    	LoomDiagnostic: The populated LoomDiagnostic instance.
    */
    pub fn new(
        path: &str,
        sourceCode: String,
        startOffset: usize,
        length: usize,
        labelText: String,
        helpText: Option<String>,
        code: String,
        message: String,
        severityLevel: miette::Severity,
    ) -> Self
    {
        Self {
            src: miette::NamedSource::new(path, sourceCode),
            startOffset,
            length,
            labelText,
            helpText,
            code,
            message,
            severityLevel,
        }
    }

    /*
    Converts the LoomDiagnostic instance into a miette::Report container.

    Takes:
    	None.

    Gives:
    	miette::Report: The miette report object ready for logging.
    */
    pub fn toReport(&self) -> miette::Report
    {
        miette::Report::new(self.clone())
    }
}

/*
Holds overall metrics, warnings, error counts, and target output logs for execution summary.
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionSummary
{
    pub filesWritten: Vec<String>,
    pub totalDocuments: usize,
    pub errorCount: usize,
    pub warningCount: usize,
    pub elapsedMs: u64,
}

static ACTIVE_LOG_LEVEL: OnceLock<LogLevel> = OnceLock::new();

/*
Retrieves the active log level threshold, defaulting to Normal if uninitialized.

Takes:
	None.

Gives:
	LogLevel: The active log level.
*/
fn getLogLevel() -> LogLevel
{
    *ACTIVE_LOG_LEVEL.get().unwrap_or(&LogLevel::Normal)
}

/*
Initializes the global diagnostics logger threshold once with a specified verbosity level.

Takes:
	level (LogLevel): The log level to set as the active threshold.

Gives:
	(): Unit type.
*/
pub fn initLogLevel(level: LogLevel) -> ()
{
    let _ = ACTIVE_LOG_LEVEL.set(level);
}

/*
Determines whether a diagnostic of the given severity should be logged under the specified active log level.

Takes:
	severity (miette::Severity): The severity level of the diagnostic.
	activeLevel (LogLevel): The active log level threshold.

Gives:
	bool: True if the diagnostic should be logged, false otherwise.
*/
fn shouldLogDiagnostic(severity: miette::Severity, activeLevel: LogLevel) -> bool
{
    match severity {
        miette::Severity::Error => true,
        miette::Severity::Warning => activeLevel >= LogLevel::Normal,
        miette::Severity::Advice => activeLevel >= LogLevel::Verbose,
    }
}

/*
Logs a miette diagnostic report if the active log level permits it based on diagnostic severity.

Takes:
	report (&miette::Report): The miette diagnostic report structure.

Gives:
	(): Unit type.
*/
pub fn logDiagnostic(report: &miette::Report) -> ()
{
    let activeLevel = getLogLevel();
    let severity = report.severity().unwrap_or(miette::Severity::Error);

    if !shouldLogDiagnostic(severity, activeLevel) {
        return;
    }

    eprintln!("{:?}", report);
}

/*
Logs an informational message to stdout if the active log level threshold is Verbose.

Takes:
	message (&str): The status or informational message string.

Gives:
	(): Unit type.
*/
pub fn logInfo(message: &str) -> ()
{
    let activeLevel = getLogLevel();

    if activeLevel < LogLevel::Verbose {
        return;
    }

    let formattedMessage = if message.starts_with('[') {
        message.to_string()
    } else {
        format!("[INFO] {}", message)
    };

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    let _ = writeln!(handle, "{}", formattedMessage);
}

/*
Renders the execution summary (success or failure) to stdout or stderr if active log level is not Quiet.

Takes:
	summary (&ExecutionSummary): The execution statistics structure.

Gives:
	(): Unit type.
*/
pub fn renderSummary(summary: &ExecutionSummary) -> ()
{
    let activeLevel = getLogLevel();

    if activeLevel == LogLevel::Quiet {
        return;
    }

    if summary.errorCount > 0 {
        let mut output = String::new();
        output.push_str("\n\x1b[31m========================================\x1b[0m\n");
        output.push_str("WEAVE FAILED\n");
        output.push_str(&format!("Errors: {}\n", summary.errorCount));
        output.push_str(&format!("Warnings: {}\n", summary.warningCount));
        output.push_str(&format!("Elapsed Time: {}ms\n", summary.elapsedMs));
        output.push_str("\x1b[31m========================================\x1b[0m\n");

        let stderr = std::io::stderr();
        let mut handle = stderr.lock();
        let _ = write!(handle, "{}", output);
    } else {
        let mut output = String::new();
        output.push_str("\n\x1b[32m========================================\x1b[0m\n");
        output.push_str("WEAVE SUCCESSFUL\n");
        output.push_str("Files Written:\n");
        for file in &summary.filesWritten {
            output.push_str(&format!("  - {}\n", file));
        }
        output.push_str(&format!("Total Documents: {}\n", summary.totalDocuments));
        output.push_str(&format!("Warnings: {}\n", summary.warningCount));
        output.push_str(&format!("Elapsed Time: {}ms\n", summary.elapsedMs));
        output.push_str("\x1b[32m========================================\x1b[0m\n");

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        let _ = write!(handle, "{}", output);
    }
}
