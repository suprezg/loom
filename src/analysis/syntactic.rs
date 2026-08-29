/*
File Name: syntactic.rs
Purpose: Syntactic parser engine for Thread specifications and Fabric blueprints using Pest PEG grammars from the grammar module, producing typed AST pairs or domain LoomDiagnostics (LM0001-LM0003 for Thread, LM1001-LM1003 for Fabric).
*/

#![allow(non_snake_case)]

use pest::Parser;
use pest::error::{Error as PestError, InputLocation, ErrorVariant};
use crate::helpers::diagnostics::LoomDiagnostic;
use crate::helpers::file_handler::{FileSpanMapping, resolveSpan};

pub use crate::grammar::thread::{ThreadParser, Rule as ThreadRule};
pub use crate::grammar::fabric::{FabricParser, Rule as FabricRule};

/*
Extracts the line text around the error location and determines byte start offset and length.

Takes:
	pestErr (&PestError<R>): The low-level Pest error structure.
	sourceContent (&str): The raw text content string.

Gives:
	(usize, usize, String, String): Tuple containing (startOffset, length, invalidToken, lineContent).
*/
fn extractErrorContext<R: pest::RuleType>(
    pestErr: &PestError<R>,
    sourceContent: &str,
) -> (usize, usize, String, String)
{
    let (startOffset, length) = match pestErr.location {
        InputLocation::Pos(pos) => {
            let remaining = if pos < sourceContent.len() { &sourceContent[pos..] } else { "" };
            let tokenLen = remaining
                .find(|c: char| c.is_whitespace() || "{}[],():;".contains(c))
                .unwrap_or(remaining.len());
            (pos, tokenLen.max(1))
        }
        InputLocation::Span((start, end)) => (start, (end - start).max(1)),
    };

    let endIdx = (startOffset + length).min(sourceContent.len());
    let invalidToken = if startOffset < sourceContent.len() {
        sourceContent[startOffset..endIdx].to_string()
    } else {
        String::from("EOF")
    };

    let beforeLine = sourceContent[..startOffset.min(sourceContent.len())]
        .lines()
        .last()
        .unwrap_or("");
    let afterLine = sourceContent[startOffset.min(sourceContent.len())..]
        .lines()
        .next()
        .unwrap_or("");
    let lineContent = format!("{}{}", beforeLine, afterLine);

    (startOffset, length, invalidToken, lineContent)
}

/*
Parses raw Thread specification text content into AST pairs or maps low-level Pest PEG syntax errors into a domain LoomDiagnostic (LM0001-LM0003).

Takes:
	content (&str): Raw text content string of the thread specification.
	fileMap (&[FileSpanMapping]): List of ingested file span offset mappings.

Gives:
	Result<pest::iterators::Pairs<'a, ThreadRule>, LoomDiagnostic>: Ok containing root AST Pairs, or Err containing LoomDiagnostic.
*/
pub fn parseThread<'a>(
    content: &'a str,
    fileMap: &[FileSpanMapping],
) -> Result<pest::iterators::Pairs<'a, ThreadRule>, LoomDiagnostic>
{
    match ThreadParser::parse(ThreadRule::thread_file, content) {
        Ok(pairs) => Ok(pairs),
        Err(pestErr) => {
            let fallbackContent = pestErr.to_string();
            let sourceContent = fileMap.first().map(|f| f.content.as_str()).unwrap_or(&fallbackContent);
            let (startOffset, length, invalidToken, lineContent) = extractErrorContext(&pestErr, sourceContent);
            let trimmedLine = lineContent.trim();
            let (targetPath, targetContent, localStart) = resolveSpan(fileMap, startOffset);

            /* LM0003: Thread Malformed Path */
            if trimmedLine.contains("::)")
                || trimmedLine.contains("(::")
                || trimmedLine.contains(":::")
                || (trimmedLine.contains("::") && (trimmedLine.ends_with("::") || trimmedLine.contains("::\"")))
            {
                return Err(LoomDiagnostic::new(
                    targetPath,
                    targetContent.to_string(),
                    localStart,
                    length,
                    format!("Malformed scoped path '{}'", invalidToken),
                    Some(String::from("Expected 'Entity' or 'Entity::Member'")),
                    String::from("LM0003"),
                    format!("Syntax error: Malformed scoped path '{}'. Expected 'Entity' or 'Entity::Member'.", invalidToken),
                    miette::Severity::Error,
                ));
            }

            let isIllegalIdent = !invalidToken.is_empty()
                && (invalidToken.chars().next().unwrap().is_ascii_digit() || invalidToken.starts_with('-'));

            match &pestErr.variant {
                ErrorVariant::ParsingError { positives, .. } => {
                    /* LM0001: Thread Illegal Identifier */
                    if positives.contains(&ThreadRule::ident) || isIllegalIdent {
                        return Err(LoomDiagnostic::new(
                            targetPath,
                            targetContent.to_string(),
                            localStart,
                            length,
                            format!("Invalid identifier '{}'", invalidToken),
                            Some(String::from("Identifiers must start with [a-zA-Z_]")),
                            String::from("LM0001"),
                            format!("Syntax error: Invalid identifier '{}'. Identifiers must start with [a-zA-Z_].", invalidToken),
                            miette::Severity::Error,
                        ));
                    }

                    /* LM0002: Thread Unclosed Block */
                    if startOffset >= sourceContent.trim_end().len() || positives.contains(&ThreadRule::EOI) {
                        return Err(LoomDiagnostic::new(
                            targetPath,
                            targetContent.to_string(),
                            localStart,
                            length,
                            String::from("Unclosed block"),
                            Some(String::from("Match and close all open braces '{', '[', and quotes")),
                            String::from("LM0002"),
                            String::from("Syntax error: Unclosed block. Expected closing token '}'."),
                            miette::Severity::Error,
                        ));
                    }

                    /* Fallback unexpected token error */
                    Err(LoomDiagnostic::new(
                        targetPath,
                        targetContent.to_string(),
                        localStart,
                        length,
                        format!("Unexpected token '{}'", invalidToken),
                        Some(String::from("Remove or update unrecognized token to conform to standard thread grammar")),
                        String::from("LM0000"),
                        format!("Syntax error: Unexpected token '{}'. Unrecognized token in thread specification.", invalidToken),
                        miette::Severity::Error,
                    ))
                }
                ErrorVariant::CustomError { message } => {
                    Err(LoomDiagnostic::new(
                        targetPath,
                        targetContent.to_string(),
                        localStart,
                        length,
                        message.clone(),
                        Some(String::from("Remove or update unrecognized token to conform to standard thread grammar")),
                        String::from("LM0000"),
                        message.clone(),
                        miette::Severity::Error,
                    ))
                }
            }
        }
    }
}

/*
Parses raw Fabric blueprint text content into AST pairs or maps low-level Pest PEG syntax errors into a domain LoomDiagnostic (LM1001-LM1003).

Takes:
	content (&str): Raw text content string of the fabric blueprint.
	fileMap (&[FileSpanMapping]): List of ingested file span offset mappings.

Gives:
	Result<pest::iterators::Pairs<'a, FabricRule>, LoomDiagnostic>: Ok containing root AST Pairs, or Err containing LoomDiagnostic.
*/
pub fn parseFabric<'a>(
    content: &'a str,
    fileMap: &[FileSpanMapping],
) -> Result<pest::iterators::Pairs<'a, FabricRule>, LoomDiagnostic>
{
    match FabricParser::parse(FabricRule::fabric_file, content) {
        Ok(pairs) => Ok(pairs),
        Err(pestErr) => {
            let fallbackContent = pestErr.to_string();
            let sourceContent = fileMap.first().map(|f| f.content.as_str()).unwrap_or(&fallbackContent);
            let (startOffset, length, invalidToken, lineContent) = extractErrorContext(&pestErr, sourceContent);
            let trimmedLine = lineContent.trim();
            let (targetPath, targetContent, localStart) = resolveSpan(fileMap, startOffset);

            /* LM1003: Fabric Malformed Path */
            if trimmedLine.contains("::") || trimmedLine.contains("..") {
                return Err(LoomDiagnostic::new(
                    targetPath,
                    targetContent.to_string(),
                    localStart,
                    length,
                    String::from("Malformed path in fabric blueprint"),
                    Some(String::from("Provide a valid quoted import path or entity reference path")),
                    String::from("LM1003"),
                    String::from("Syntax error: Malformed path in fabric blueprint."),
                    miette::Severity::Error,
                ));
            }

            let isIllegalIdent = !invalidToken.is_empty()
                && (invalidToken.chars().next().unwrap().is_ascii_digit() || invalidToken.starts_with('-'));

            match &pestErr.variant {
                ErrorVariant::ParsingError { positives, .. } => {
                    /* LM1001: Fabric Illegal Identifier */
                    if positives.contains(&FabricRule::ident) || isIllegalIdent {
                        return Err(LoomDiagnostic::new(
                            targetPath,
                            targetContent.to_string(),
                            localStart,
                            length,
                            format!("Invalid identifier '{}'", invalidToken),
                            Some(String::from("Identifiers must start with [a-zA-Z_]")),
                            String::from("LM1001"),
                            format!("Syntax error: Invalid identifier '{}'. Identifiers must start with [a-zA-Z_].", invalidToken),
                            miette::Severity::Error,
                        ));
                    }

                    /* LM1002: Fabric Unclosed Block */
                    if startOffset >= sourceContent.trim_end().len() || (trimmedLine.starts_with("system \"") && !trimmedLine.ends_with('"')) {
                        return Err(LoomDiagnostic::new(
                            targetPath,
                            targetContent.to_string(),
                            localStart,
                            length,
                            String::from("Unclosed block in fabric blueprint"),
                            Some(String::from("Match and close all open quotes and brackets in .fabric file")),
                            String::from("LM1002"),
                            String::from("Syntax error: Unclosed block in fabric blueprint."),
                            miette::Severity::Error,
                        ));
                    }

                    /* Fallback unexpected token error for fabric */
                    Err(LoomDiagnostic::new(
                        targetPath,
                        targetContent.to_string(),
                        localStart,
                        length,
                        format!("Unexpected token '{}'", invalidToken),
                        Some(String::from("Remove or update unrecognized token to conform to standard fabric grammar")),
                        String::from("LM1001"),
                        format!("Syntax error: Unexpected token '{}'. Unrecognized token in fabric blueprint.", invalidToken),
                        miette::Severity::Error,
                    ))
                }
                ErrorVariant::CustomError { message } => {
                    Err(LoomDiagnostic::new(
                        targetPath,
                        targetContent.to_string(),
                        localStart,
                        length,
                        message.clone(),
                        Some(String::from("Remove or update unrecognized token to conform to standard fabric grammar")),
                        String::from("LM1000"),
                        message.clone(),
                        miette::Severity::Error,
                    ))
                }
            }
        }
    }
}
