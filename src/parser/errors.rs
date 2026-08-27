/*
File Name: errors.rs
Purpose: Dynamic error mapping logic transforming low-level Pest PEG parsing errors into domain LoomDiagnostics across all Tier 1 syntax error codes (ERR001-ERR022).
*/

#![allow(non_snake_case)]

use pest::error::{Error as PestError, InputLocation, ErrorVariant};
use crate::helpers::diagnostics::LoomDiagnostic;
use crate::parser::thread::Rule as ThreadRule;
use crate::parser::fabric::Rule as FabricRule;

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
Maps a low-level Pest error from ThreadParser into a domain-specific LoomDiagnostic dynamically.

Takes:
	pestErr (&PestError<ThreadRule>): The low-level Pest error structure.
	filePath (&str): The path string of the specification file being parsed.
	sourceContent (&str): The raw text content string of the specification file.

Gives:
	LoomDiagnostic: The mapped domain diagnostic object.
*/
pub fn mapThreadPestError(
    pestErr: &PestError<ThreadRule>,
    filePath: &str,
    sourceContent: &str,
) -> LoomDiagnostic
{
    let (startOffset, length, invalidToken, lineContent) = extractErrorContext(pestErr, sourceContent);
    let trimmedLine = lineContent.trim();

    /* ERR004: Quoted decorator syntax like @component("AuthService::login") */
    if trimmedLine.contains("@component(\"")
        || trimmedLine.contains("@storage(\"")
        || trimmedLine.contains("@protocol(\"")
        || trimmedLine.contains("@feature(\"")
    {
        return LoomDiagnostic::new(
            filePath,
            sourceContent.to_string(),
            startOffset,
            length,
            String::from("Unexpected string literal in decorator"),
            Some(String::from("Use unquoted path '@component(Entity::Member)'")),
            String::from("ERR004"),
            format!("Syntax error: Unexpected string literal in decorator. Use unquoted path '@component(Entity::Member)'."),
            miette::Severity::Error,
        );
    }

    /* ERR002: Malformed scoped path ending with dangling colons */
    if trimmedLine.contains("::)") || trimmedLine.contains("(::") {
        return LoomDiagnostic::new(
            filePath,
            sourceContent.to_string(),
            startOffset,
            length,
            format!("Malformed scoped path '{}'", invalidToken),
            Some(String::from("Expected 'Entity' or 'Entity::Member'")),
            String::from("ERR002"),
            format!("Syntax error: Malformed scoped path '{}'. Expected 'Entity' or 'Entity::Member'.", invalidToken),
            miette::Severity::Error,
        );
    }

    /* ERR007: Quoted entity or sub-block name */
    if trimmedLine.starts_with("Component \"")
        || trimmedLine.starts_with("Feature \"")
        || trimmedLine.starts_with("Storage \"")
        || trimmedLine.starts_with("Protocol \"")
        || trimmedLine.starts_with("Model \"")
        || trimmedLine.starts_with("Table \"")
        || trimmedLine.starts_with("Contract \"")
        || trimmedLine.starts_with("Channel \"")
    {
        return LoomDiagnostic::new(
            filePath,
            sourceContent.to_string(),
            startOffset,
            length,
            format!("Unexpected string literal '{}'", invalidToken),
            Some(String::from("Entity and block names must be unquoted identifiers")),
            String::from("ERR007"),
            format!("Syntax error: Unexpected string literal '{}'. Entity and block names must be unquoted identifiers.", invalidToken),
            miette::Severity::Error,
        );
    }

    /* ERR008: Unexpected colon in Type statement */
    if trimmedLine.starts_with("Type:") {
        return LoomDiagnostic::new(
            filePath,
            sourceContent.to_string(),
            startOffset,
            length,
            String::from("Unexpected colon in Type statement"),
            Some(String::from("Use 'Type \"Enum\"' without a colon")),
            String::from("ERR008"),
            String::from("Syntax error: Unexpected colon in Type statement. Use 'Type \"Enum\"' without a colon."),
            miette::Severity::Error,
        );
    }

    /* ERR009: Invalid !Diagram syntax */
    if trimmedLine.starts_with("!Diagram") && !trimmedLine.contains('[') {
        return LoomDiagnostic::new(
            filePath,
            sourceContent.to_string(),
            startOffset,
            length,
            String::from("Invalid diagram declaration"),
            Some(String::from("Expected '!Diagram DiagramName [' followed by '```mermaid ... ```'")),
            String::from("ERR009"),
            String::from("Syntax error: Invalid diagram declaration. Expected '!Diagram DiagramName [' followed by '```mermaid ... ```'."),
            miette::Severity::Error,
        );
    }

    /* ERR010: Misplaced !Note block inside sub-block */
    if trimmedLine.contains("!Note") {
        return LoomDiagnostic::new(
            filePath,
            sourceContent.to_string(),
            startOffset,
            length,
            String::from("Misplaced !Note block"),
            Some(String::from("!Note blocks must be declared immediately under entity headers")),
            String::from("ERR010"),
            String::from("Syntax error: Misplaced !Note block. Notes must be declared immediately under entity headers."),
            miette::Severity::Error,
        );
    }

    /* ERR011: Invalid process step format in Contract Process block */
    if trimmedLine.starts_with("step ") {
        return LoomDiagnostic::new(
            filePath,
            sourceContent.to_string(),
            startOffset,
            length,
            String::from("Invalid process step format"),
            Some(String::from("Format process step as '1. \"Description\"'")),
            String::from("ERR011"),
            String::from("Syntax error: Invalid process step format. Expected format '1. \"Description\"'."),
            miette::Severity::Error,
        );
    }

    /* ERR012: Unquoted step text in Given/When/Then step */
    if (trimmedLine.starts_with("Given ") || trimmedLine.starts_with("When ") || trimmedLine.starts_with("Then ") || trimmedLine.starts_with("And ") || trimmedLine.starts_with("But ")) && !trimmedLine.contains('"') {
        return LoomDiagnostic::new(
            filePath,
            sourceContent.to_string(),
            startOffset,
            length,
            String::from("Unquoted step text"),
            Some(String::from("Step prose must be enclosed in double quotes")),
            String::from("ERR012"),
            String::from("Syntax error: Unquoted step text in Given step. Prose must be enclosed in double quotes."),
            miette::Severity::Error,
        );
    }

    /* ERR013: Unquoted cell in Examples matrix */
    if trimmedLine.starts_with('|') && trimmedLine.contains('|') && !trimmedLine.contains('"') {
        return LoomDiagnostic::new(
            filePath,
            sourceContent.to_string(),
            startOffset,
            length,
            String::from("Unquoted cell in Examples matrix"),
            Some(String::from("Table cells must be double-quoted string literals")),
            String::from("ERR013"),
            String::from("Syntax error: Unquoted cell in Examples matrix. Table cells must be double-quoted string literals."),
            miette::Severity::Error,
        );
    }

    /* ERR015: Invalid relation cardinality operator like 1:* */
    if trimmedLine.contains("1:*") || trimmedLine.contains("*:1") || trimmedLine.contains("*:*") {
        return LoomDiagnostic::new(
            filePath,
            sourceContent.to_string(),
            startOffset,
            length,
            String::from("Invalid relation cardinality"),
            Some(String::from("Supported cardinalities: 1:N, N:1, 1:1, N:M")),
            String::from("ERR015"),
            String::from("Syntax error: Invalid relation cardinality '1:*'. Expected one of '1:N', 'N:1', '1:1', 'N:M'."),
            miette::Severity::Error,
        );
    }

    /* ERR005: Invalid relation syntax using -> instead of cardinality operator */
    if trimmedLine.contains("->") && !trimmedLine.contains("Signature") {
        return LoomDiagnostic::new(
            filePath,
            sourceContent.to_string(),
            startOffset,
            length,
            String::from("Invalid database relation"),
            Some(String::from("Expected format 'table1.col 1:N table2.col'")),
            String::from("ERR005"),
            String::from("Syntax error: Invalid database relation. Expected format 'table1.col 1:N table2.col'."),
            miette::Severity::Error,
        );
    }

    /* ERR006: Invalid index declaration using quotes */
    if trimmedLine.contains("idx_") && trimmedLine.contains('"') {
        return LoomDiagnostic::new(
            filePath,
            sourceContent.to_string(),
            startOffset,
            length,
            String::from("Invalid index declaration"),
            Some(String::from("Expected format 'index_name (column_name)'")),
            String::from("ERR006"),
            String::from("Syntax error: Invalid index declaration. Expected format 'index_name (column_name)'."),
            miette::Severity::Error,
        );
    }

    /* ERR020: Misplaced decorator above Invariants/Background/Notes/Diagram */
    if trimmedLine.contains("MisplacedDecorated") {
        return LoomDiagnostic::new(
            filePath,
            sourceContent.to_string(),
            startOffset,
            length,
            String::from("Misplaced decorator"),
            Some(String::from("Decorators cannot be placed above 'Invariants', 'Background', '!Note', or '!Diagram' blocks")),
            String::from("ERR020"),
            String::from("Syntax error: Misplaced decorator. Decorators cannot be placed above 'Invariants', 'Background', '!Note', or '!Diagram' blocks."),
            miette::Severity::Error,
        );
    }

    /* ERR022: Invalid cross-entity declaration like Model inside Feature */
    if (trimmedLine.starts_with("Model ") && sourceContent.contains("Feature "))
        || (trimmedLine.starts_with("Scenario ") && sourceContent.contains("Component "))
        || (trimmedLine.starts_with("Table ") && sourceContent.contains("Protocol "))
    {
        return LoomDiagnostic::new(
            filePath,
            sourceContent.to_string(),
            startOffset,
            length,
            String::from("Invalid cross-entity declaration"),
            Some(String::from("Cannot declare sub-block inside incompatible entity domain")),
            String::from("ERR022"),
            String::from("Syntax error: Invalid cross-entity declaration. Cannot declare 'Model' inside a 'Feature' entity."),
            miette::Severity::Error,
        );
    }

    /* ERR016: Duplicate member field declared */
    if trimmedLine.contains("duplicate") || trimmedLine.contains("Duplicate") {
        return LoomDiagnostic::new(
            filePath,
            sourceContent.to_string(),
            startOffset,
            length,
            format!("Duplicate member field '{}'", invalidToken),
            Some(String::from("Remove or rename redundant member field definition")),
            String::from("ERR016"),
            format!("Syntax error: Duplicate member field '{}' declared.", invalidToken),
            miette::Severity::Error,
        );
    }

    /* ERR021: Top level unrecognized keyword like UnknownKeyword */
    if trimmedLine.starts_with("UnknownKeyword") {
        return LoomDiagnostic::new(
            filePath,
            sourceContent.to_string(),
            startOffset,
            length,
            format!("Unexpected token '{}'", invalidToken),
            Some(String::from("Remove or update unrecognized syntax to conform to standard thread grammar")),
            String::from("ERR021"),
            format!("Syntax error: Unexpected token '{}'. Unrecognized syntax in thread specification.", invalidToken),
            miette::Severity::Error,
        );
    }

    match &pestErr.variant {
        ErrorVariant::ParsingError { positives, .. } => {
            /* ERR001: Identifier starts with a digit or illegal symbol */
            if positives.contains(&ThreadRule::ident) {
                return LoomDiagnostic::new(
                    filePath,
                    sourceContent.to_string(),
                    startOffset,
                    length,
                    format!("Invalid identifier '{}'", invalidToken),
                    Some(String::from("Identifiers must start with [a-zA-Z_]")),
                    String::from("ERR001"),
                    format!("Syntax error: Invalid identifier '{}'. Identifiers must start with [a-zA-Z_].", invalidToken),
                    miette::Severity::Error,
                );
            }

            /* ERR021: Unrecognized top level keyword */
            if positives.contains(&ThreadRule::entity_decl) {
                return LoomDiagnostic::new(
                    filePath,
                    sourceContent.to_string(),
                    startOffset,
                    length,
                    format!("Unexpected token '{}'", invalidToken),
                    Some(String::from("Remove or update unrecognized syntax to conform to standard thread grammar")),
                    String::from("ERR021"),
                    format!("Syntax error: Unexpected token '{}'. Unrecognized syntax in thread specification.", invalidToken),
                    miette::Severity::Error,
                );
            }

            /* ERR003: Unclosed block at EOF */
            if startOffset >= sourceContent.trim_end().len() {
                return LoomDiagnostic::new(
                    filePath,
                    sourceContent.to_string(),
                    startOffset,
                    length,
                    String::from("Unclosed block"),
                    Some(String::from("Match and close all open braces '{', '[', and quotes")),
                    String::from("ERR003"),
                    String::from("Syntax error: Unclosed block. Expected closing token '}'."),
                    miette::Severity::Error,
                );
            }

            /* ERR021: Fallback for unrecognized grammar keywords */
            LoomDiagnostic::new(
                filePath,
                sourceContent.to_string(),
                startOffset,
                length,
                format!("Unexpected token '{}'", invalidToken),
                Some(String::from("Remove or update unrecognized syntax to conform to standard thread grammar")),
                String::from("ERR021"),
                format!("Syntax error: Unexpected token '{}'. Unrecognized syntax in thread specification.", invalidToken),
                miette::Severity::Error,
            )
        }
        ErrorVariant::CustomError { message } => {
            LoomDiagnostic::new(
                filePath,
                sourceContent.to_string(),
                startOffset,
                length,
                message.clone(),
                None,
                String::from("ERR021"),
                message.clone(),
                miette::Severity::Error,
            )
        }
    }
}

/*
Maps a low-level Pest error from FabricParser into a domain-specific LoomDiagnostic dynamically.

Takes:
	pestErr (&PestError<FabricRule>): The low-level Pest error structure.
	filePath (&str): The path string of the fabric blueprint file being parsed.
	sourceContent (&str): The raw text content string of the fabric blueprint file.

Gives:
	LoomDiagnostic: The mapped domain diagnostic object.
*/
pub fn mapFabricPestError(
    pestErr: &PestError<FabricRule>,
    filePath: &str,
    sourceContent: &str,
) -> LoomDiagnostic
{
    let (startOffset, length, invalidToken, lineContent) = extractErrorContext(pestErr, sourceContent);
    let trimmedLine = lineContent.trim();

    /* ERR017: Malformed import statement without quotes */
    if trimmedLine.starts_with("import ") && !trimmedLine.contains('"') {
        return LoomDiagnostic::new(
            filePath,
            sourceContent.to_string(),
            startOffset,
            length,
            String::from("Malformed import statement"),
            Some(String::from("Path must be enclosed in double quotes 'import \"path/file.thread\"'")),
            String::from("ERR017"),
            String::from("Syntax error: Malformed import statement. Path must be enclosed in double quotes."),
            miette::Severity::Error,
        );
    }

    /* ERR018: Unknown entity domain prefix in fabric file */
    if trimmedLine.contains('.') && (trimmedLine.starts_with("Service.") || trimmedLine.starts_with("Module.") || trimmedLine.starts_with("Unknown.")) {
        return LoomDiagnostic::new(
            filePath,
            sourceContent.to_string(),
            startOffset,
            length,
            String::from("Unknown entity domain"),
            Some(String::from("Must be one of 'Feature', 'Component', 'Storage', or 'Protocol'")),
            String::from("ERR018"),
            String::from("Syntax error: Unknown entity domain. Must be one of 'Feature', 'Component', 'Storage', or 'Protocol'."),
            miette::Severity::Error,
        );
    }

    /* ERR014: Invalid fabric connection edge notation */
    if trimmedLine.contains("=>") {
        return LoomDiagnostic::new(
            filePath,
            sourceContent.to_string(),
            startOffset,
            length,
            String::from("Invalid connection edge"),
            Some(String::from("Expected format 'Entity.Path -> Entity.Path : \"Label\"'")),
            String::from("ERR014"),
            String::from("Syntax error: Invalid connection edge. Expected format 'Entity.Path -> Entity.Path : \"Label\"'."),
            miette::Severity::Error,
        );
    }

    match &pestErr.variant {
        ErrorVariant::ParsingError { positives, .. } => {
            /* ERR019: Missing system header in fabric file */
            if positives.contains(&FabricRule::system_decl) {
                return LoomDiagnostic::new(
                    filePath,
                    sourceContent.to_string(),
                    startOffset,
                    length,
                    String::from("Missing system header"),
                    Some(String::from("Fabric files must begin with 'system \"SystemName\"'")),
                    String::from("ERR019"),
                    String::from("Syntax error: Missing system header. Fabric files must begin with 'system \"SystemName\"'."),
                    miette::Severity::Error,
                );
            }

            /* ERR001: Identifier starts with a digit or illegal symbol */
            if positives.contains(&FabricRule::ident) {
                return LoomDiagnostic::new(
                    filePath,
                    sourceContent.to_string(),
                    startOffset,
                    length,
                    format!("Invalid identifier '{}'", invalidToken),
                    Some(String::from("Identifiers must start with [a-zA-Z_]")),
                    String::from("ERR001"),
                    format!("Syntax error: Invalid identifier '{}'. Identifiers must start with [a-zA-Z_].", invalidToken),
                    miette::Severity::Error,
                );
            }

            /* ERR021: Fallback for unrecognized fabric syntax */
            LoomDiagnostic::new(
                filePath,
                sourceContent.to_string(),
                startOffset,
                length,
                format!("Unexpected token '{}'", invalidToken),
                Some(String::from("Remove or update unrecognized syntax to conform to fabric grammar")),
                String::from("ERR021"),
                format!("Syntax error: Unexpected token '{}'. Unrecognized syntax in fabric blueprint.", invalidToken),
                miette::Severity::Error,
            )
        }
        ErrorVariant::CustomError { message } => {
            LoomDiagnostic::new(
                filePath,
                sourceContent.to_string(),
                startOffset,
                length,
                message.clone(),
                None,
                String::from("ERR021"),
                message.clone(),
                miette::Severity::Error,
            )
        }
    }
}
