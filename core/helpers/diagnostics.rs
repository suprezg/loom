/*
File Name: diagnostics.rs
Purpose: Implementation of the DiagnosticsLogger helper component.
*/

#![allow(non_snake_case)]

use crate::models::diagnostic_models::{Diagnostic, DiagnosticSeverity, LogLevel, ExecutionSummary};
use std::sync::RwLock;
use std::io::Write;

static ACTIVE_LOG_LEVEL: RwLock<LogLevel> = RwLock::new(LogLevel::Normal);

/**
 * Initializes the global diagnostics logger with a specified verbosity level.
 *
 * Takes:
 * 	level (LogLevel): The log level to set as the active threshold.
 *
 * Gives:
 * 	(): Unit type.
 */
pub fn init(level: LogLevel) -> ()
{
    if let Ok(mut activeLevel) = ACTIVE_LOG_LEVEL.write() {
        *activeLevel = level;
    }
}

/**
 * Formats and logs a diagnostic message with color-coded severity and optional code snippet.
 *
 * Takes:
 * 	diagnostic (&Diagnostic): The diagnostic report structure.
 * 	sourceCode (Option<&str>): The optional raw source code for context rendering.
 *
 * Gives:
 * 	(): Unit type.
 */
pub fn logDiagnostic(diagnostic: &Diagnostic, sourceCode: Option<&str>) -> ()
{
    let activeLevel = match ACTIVE_LOG_LEVEL.read() {
        Ok(level) => *level,
        Err(_) => LogLevel::Normal,
    };
    
    let shouldLog = match diagnostic.severity {
        DiagnosticSeverity::Error => true,
        DiagnosticSeverity::Warning => activeLevel >= LogLevel::Normal,
        DiagnosticSeverity::Info => activeLevel >= LogLevel::Verbose,
    };
    
    if !shouldLog {
        return;
    }
    
    let severityStr = match diagnostic.severity {
        DiagnosticSeverity::Error => "error",
        DiagnosticSeverity::Warning => "warning",
        DiagnosticSeverity::Info => "info",
    };
    
    let colorCode = match diagnostic.severity {
        DiagnosticSeverity::Error => "\x1b[31m",
        DiagnosticSeverity::Warning => "\x1b[33m",
        DiagnosticSeverity::Info => "\x1b[34m",
    };
    
    let mut output = String::new();
    output.push_str(&format!("{colorCode}{severityStr}[{}]\x1b[0m: {}:{}:{}\n", diagnostic.code, diagnostic.path, diagnostic.line, diagnostic.column));
    output.push_str(&format!("  {}\n", diagnostic.message));
    
    if let Some(codeStr) = sourceCode {
        let lines: Vec<&str> = codeStr.lines().collect();
        if diagnostic.line > 0 && diagnostic.line <= lines.len() {
            let targetLineContent = lines[diagnostic.line - 1];
            let lineNumStr = diagnostic.line.to_string();
            let padding = " ".repeat(lineNumStr.len());
            output.push_str(&format!("{} |\n", padding));
            output.push_str(&format!("{} | {}\n", lineNumStr, targetLineContent));
            let spaces = " ".repeat(diagnostic.column.saturating_sub(1));
            output.push_str(&format!("{} | {}{}^\x1b[0m\n", padding, spaces, colorCode));
        }
    }
    
    let stderr = std::io::stderr();
    let mut handle = stderr.lock();
    let _ = write!(handle, "{}", output);
}

/**
 * Logs an informational message to stdout if the active log level meets the threshold.
 *
 * Takes:
 * 	message (&str): The status or informational message.
 * 	levelThreshold (LogLevel): The required minimum log level.
 *
 * Gives:
 * 	(): Unit type.
 */
pub fn logInfo(message: &str, levelThreshold: LogLevel) -> ()
{
    let activeLevel = match ACTIVE_LOG_LEVEL.read() {
        Ok(level) => *level,
        Err(_) => LogLevel::Normal,
    };
    
    if activeLevel < levelThreshold {
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

/**
 * Renders the execution summary (success or failure) to stdout or stderr.
 *
 * Takes:
 * 	summary (&ExecutionSummary): The execution statistics.
 *
 * Gives:
 * 	(): Unit type.
 */
pub fn renderSummary(summary: &ExecutionSummary) -> ()
{
    let activeLevel = match ACTIVE_LOG_LEVEL.read() {
        Ok(level) => *level,
        Err(_) => LogLevel::Normal,
    };
    
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
