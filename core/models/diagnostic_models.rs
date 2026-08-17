/*
File Name: diagnostics_models.rs
Purpose: Data models for the diagnostics and logging system of the Loom compiler.
*/

#![allow(non_snake_case)]

/**
 * Severity level of a compiler or runtime diagnostic message.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticSeverity
{
    Error,
    Warning,
    Info,
}

/**
 * Represents a single diagnostic report containing location metadata, a diagnostic code, and the explanation.
 */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic
{
    pub path: String,
    pub line: usize,
    pub column: usize,
    pub code: String,
    pub message: String,
    pub severity: DiagnosticSeverity,
}

/**
 * Verbosity level of the logger to control which levels of details are output to standard streams.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel
{
    Quiet,
    Normal,
    Verbose,
}

/**
 * Holds overall metrics, warnings, error counts, and target output logs for execution summary.
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
