/*
File Name: file_handler.rs
Purpose: Implementation of the FileHandler helper component providing file ingestion, payload resolution, byte span mapping, and file writing utilities for the Loom compiler using PathResolver.
*/

#![allow(non_snake_case)]

use std::fs;
use std::path::Path;
use crate::helpers::path_resolver::{resolvePath, getParentDir};
use crate::helpers::diagnostics::{logMessage, LoomMessage};

/*
Structure representing a file's byte span location mapping within a concatenated source payload.
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileSpanMapping
{
    pub filePath: String,
    pub content: String,
    pub startOffset: usize,
    pub endOffset: usize,
}

/*
Represents an ingested file payload containing resolved thread file mappings, thread content, fabric file mappings, and fabric content.
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngestedPayload
{
    pub fabricFileMapping: Vec<FileSpanMapping>,
    pub fabricContent: Option<String>,
    pub threadFileMapping: Vec<FileSpanMapping>,
    pub threadContent: String,
}

/*
Resolves a byte offset into its original source file path, content string, and local byte offset using a file span mapping table.

Takes:
	fileMap (&[FileSpanMapping]): List of ingested file span mappings.
	offset (usize): Byte offset within concatenated content.

Gives:
	(&str, &str, usize): Tuple of (resolved file path, resolved source content, resolved local start offset).
*/
pub fn resolveSpan<'a>(
    fileMap: &'a [FileSpanMapping],
    offset: usize,
) -> (&'a str, &'a str, usize)
{
    for file in fileMap {
        if offset >= file.startOffset && offset < file.endOffset {
            let localOffset = offset - file.startOffset;
            return (&file.filePath, &file.content, localOffset);
        }
    }
    if let Some(first) = fileMap.first() {
        (&first.filePath, &first.content, offset.saturating_sub(first.startOffset))
    } else {
        ("", "", offset)
    }
}

/*
Reads raw text content from a file given its path string.

Takes:
	path (&str): The file path string to read.

Gives:
	Result<String, String>: Ok containing raw file text content string or Error string.
*/
fn readFile(path: &str) -> Result<String, String>
{
    logMessage(&LoomMessage::new(
        format!("Reading raw file content from path '{}'", path),
        miette::Severity::Advice,
    ));

    if path.is_empty() {
        let err = String::from("File read failed: Target path string is empty");
        logMessage(&LoomMessage::new(&err, miette::Severity::Error));
        return Err(err);
    }

    match fs::read(path) {
        Ok(bytes) => {
            logMessage(&LoomMessage::new(
                format!("Read {} bytes from file '{}'", bytes.len(), path),
                miette::Severity::Advice,
            ));
            match String::from_utf8(bytes) {
                Ok(content) => {
                    logMessage(&LoomMessage::new(
                        format!("Successfully decoded UTF-8 string content for '{}'", path),
                        miette::Severity::Advice,
                    ));
                    Ok(content)
                }
                Err(err) => {
                    let errMsg = format!("Invalid UTF-8 encoding in file '{}': {}", path, err);
                    logMessage(&LoomMessage::new(&errMsg, miette::Severity::Error));
                    Err(errMsg)
                }
            }
        }
        Err(err) => {
            if err.kind() == std::io::ErrorKind::PermissionDenied {
                let errMsg = format!("Access denied reading file '{}': Permission denied", path);
                logMessage(&LoomMessage::new(&errMsg, miette::Severity::Error));
                Err(errMsg)
            } else {
                let errMsg = format!("I/O error reading file '{}': {}", path, err);
                logMessage(&LoomMessage::new(&errMsg, miette::Severity::Error));
                Err(errMsg)
            }
        }
    }
}

/*
Writes content to a target file path, creating any parent directories automatically if they do not exist.

Takes:
	targetPath (&str): The absolute or relative file path to write to.
	content (&str): The string content to write.

Gives:
	Result<(), String>: Returns Ok(()) on successful write, or Err(String) error message on failure.
*/
pub fn writeFile(
    targetPath: &str,
    content: &str,
) -> Result<(), String>
{
    logMessage(&LoomMessage::new(
        format!("Attempting to write {} characters to target path '{}'", content.len(), targetPath),
        miette::Severity::Advice,
    ));

    let path = Path::new(targetPath);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            logMessage(&LoomMessage::new(
                format!("Creating missing parent directories for target path '{}'", parent.display()),
                miette::Severity::Advice,
            ));
            if let Err(err) = fs::create_dir_all(parent) {
                let errMsg = format!("Failed to create parent directory '{}': {}", parent.display(), err);
                logMessage(&LoomMessage::new(&errMsg, miette::Severity::Error));
                return Err(errMsg);
            }
        }
    }

    match fs::write(path, content) {
        Ok(_) => {
            logMessage(&LoomMessage::new(
                format!("Successfully wrote content to target file '{}'", targetPath),
                miette::Severity::Advice,
            ));
            Ok(())
        }
        Err(err) => {
            let errMsg = format!("Failed to write content to file '{}': {}", targetPath, err);
            logMessage(&LoomMessage::new(&errMsg, miette::Severity::Error));
            Err(errMsg)
        }
    }
}

/*
Appends a thread specification file entry to the thread content buffer and updates the thread file mapping with absolute file path.

Takes:
	filePath (&str): The absolute file path string of the thread specification.
	content (String): The raw text content of the thread specification.
	threadContent (&mut String): Reference to active concatenated thread content string.
	threadFileMapping (&mut Vec<FileSpanMapping>): Reference to list of thread file span offset mappings.

Gives:
	(): Unit type.
*/
fn mapThreadFileEntry(
    filePath: &str,
    content: String,
    threadContent: &mut String,
    threadFileMapping: &mut Vec<FileSpanMapping>,
)
{
    let startOffset = threadContent.len();
    threadContent.push_str(&content);
    threadContent.push('\n');
    let endOffset = threadContent.len();

    logMessage(&LoomMessage::new(
        format!("Mapped thread file span for '{}': byte range [{}..{}]", filePath, startOffset, endOffset),
        miette::Severity::Advice,
    ));

    threadFileMapping.push(FileSpanMapping {
        filePath: filePath.to_string(),
        content,
        startOffset,
        endOffset,
    });
}

/*
Appends a fabric blueprint file entry to the fabric file mapping vector with absolute file path.

Takes:
	filePath (&str): The absolute file path string of the fabric blueprint.
	content (String): The raw text content of the fabric blueprint.
	fabricFileMapping (&mut Vec<FileSpanMapping>): Reference to list of fabric file span offset mappings.

Gives:
	(): Unit type.
*/
fn mapFabricFileEntry(
    filePath: &str,
    content: String,
    fabricFileMapping: &mut Vec<FileSpanMapping>,
)
{
    let startOffset = 0;
    let endOffset = content.len();

    logMessage(&LoomMessage::new(
        format!("Mapped fabric blueprint span for '{}': byte range [{}..{}]", filePath, startOffset, endOffset),
        miette::Severity::Advice,
    ));

    fabricFileMapping.push(FileSpanMapping {
        filePath: filePath.to_string(),
        content,
        startOffset,
        endOffset,
    });
}

/*
Reads file or directory content and packages it into an IngestedPayload using PathResolver functions (getParentDir and resolvePath) storing canonical absolute paths.

Takes:
	path (&str): The target file or directory path string.

Gives:
	Result<IngestedPayload, String>: Ok containing populated IngestedPayload struct or Error string.
*/
pub fn giveFilePayload(path: &str) -> Result<IngestedPayload, String>
{
    logMessage(&LoomMessage::new(
        format!("Starting file payload ingestion process for path '{}'", path),
        miette::Severity::Advice,
    ));

    if path.is_empty() {
        let err = String::from("Ingestion failed: Provided path string is empty");
        logMessage(&LoomMessage::new(&err, miette::Severity::Error));
        return Err(err);
    }

    let mut payload = IngestedPayload {
        fabricFileMapping: Vec::new(),
        fabricContent: None,
        threadFileMapping: Vec::new(),
        threadContent: String::new(),
    };

    let pathObj = Path::new(path);

    /* Single File Ingestion via PathResolver (getParentDir + resolvePath) using loomPath.absolute */
    if pathObj.is_file() {
        logMessage(&LoomMessage::new(
            format!("Path '{}' identified as a single target file", path),
            miette::Severity::Advice,
        ));

        let parentDir = getParentDir(path)?;
        let fileName = pathObj
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| {
                let err = format!("Failed to extract file name from path '{}'", path);
                logMessage(&LoomMessage::new(&err, miette::Severity::Error));
                err
            })?;

        let loomPath = resolvePath(&parentDir, fileName)?;
        let content = readFile(&loomPath.absolute)?;

        if path.ends_with(".thread") {
            mapThreadFileEntry(&loomPath.absolute, content, &mut payload.threadContent, &mut payload.threadFileMapping);
        } else if path.ends_with(".fabric") {
            mapFabricFileEntry(&loomPath.absolute, content.clone(), &mut payload.fabricFileMapping);
            payload.fabricContent = Some(content);
        } else {
            let err = format!("Unsupported file extension for '{}'. Only .thread and .fabric are accepted", path);
            logMessage(&LoomMessage::new(&err, miette::Severity::Error));
            return Err(err);
        }

        logMessage(&LoomMessage::new(
            format!("Single file ingestion completed successfully for '{}'", loomPath.absolute),
            miette::Severity::Advice,
        ));

        return Ok(payload);
    }

    /* Directory Ingestion via PathResolver (resolvePath per directory entry) using loomPath.absolute */
    if pathObj.is_dir() {
        logMessage(&LoomMessage::new(
            format!("Path '{}' identified as a directory, scanning for specification entries", path),
            miette::Severity::Advice,
        ));

        let entries = match fs::read_dir(pathObj) {
            Ok(e) => e,
            Err(err) => {
                let errMsg = format!("Failed to read directory entries in '{}': {}", path, err);
                logMessage(&LoomMessage::new(&errMsg, miette::Severity::Error));
                return Err(errMsg);
            }
        };

        for entry in entries {
            if let Ok(dirEntry) = entry {
                let entryPath = dirEntry.path();
                if entryPath.is_file() {
                    let fileName = dirEntry.file_name();
                    let fileNameStr = fileName.to_str().unwrap_or("");

                    if fileNameStr.ends_with(".thread") {
                        logMessage(&LoomMessage::new(
                            format!("Found thread specification entry '{}' in directory", fileNameStr),
                            miette::Severity::Advice,
                        ));
                        if let Ok(loomPath) = resolvePath(path, fileNameStr) {
                            if let Ok(content) = readFile(&loomPath.absolute) {
                                mapThreadFileEntry(&loomPath.absolute, content, &mut payload.threadContent, &mut payload.threadFileMapping);
                            }
                        }
                    } else if fileNameStr.ends_with(".fabric") {
                        logMessage(&LoomMessage::new(
                            format!("Found fabric blueprint entry '{}' in directory", fileNameStr),
                            miette::Severity::Advice,
                        ));
                        if let Ok(loomPath) = resolvePath(path, fileNameStr) {
                            if let Ok(content) = readFile(&loomPath.absolute) {
                                mapFabricFileEntry(&loomPath.absolute, content.clone(), &mut payload.fabricFileMapping);
                                payload.fabricContent = Some(content);
                            }
                        }
                    }
                }
            }
        }

        logMessage(&LoomMessage::new(
            format!("Directory ingestion completed for '{}': {} thread files, {} fabric files ingested",
                path, payload.threadFileMapping.len(), payload.fabricFileMapping.len()),
            miette::Severity::Advice,
        ));

        return Ok(payload);
    }

    let err = format!("Ingestion failed: Target path '{}' is neither a valid file nor directory", path);
    logMessage(&LoomMessage::new(&err, miette::Severity::Error));
    Err(err)
}

#[cfg(test)]
mod tests
{
    use super::*;

    /*
    Tests readFile with a valid file path (helpers/path_resolver.rs).

    Takes:
    	None.

    Gives:
    	(): Unit type.
    */
    #[test]
    fn testReadFileWithValidFile() -> ()
    {
        let result = readFile("helpers/path_resolver.rs");
        assert!(result.is_ok());
        let content = result.unwrap();
        assert!(!content.is_empty());
    }

    /*
    Tests readFile with a directory path (should return Error string).

    Takes:
    	None.

    Gives:
    	(): Unit type.
    */
    #[test]
    fn testReadFileWithDirectoryPath() -> ()
    {
        let result = readFile("helpers");
        assert!(result.is_err());
    }
}
