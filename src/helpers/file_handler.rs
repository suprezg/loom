/*
File Name: file_handler.rs
Purpose: Implementation of the FileHandler helper component providing file ingestion, payload resolution, byte span mapping, and file writing utilities for the Loom compiler using PathResolver.
*/

#![allow(non_snake_case)]

use std::fs;
use std::path::Path;
use crate::helpers::path_resolver::{resolvePath, getParentDir};

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
Enumeration of file ingestion errors.
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngestError
{
    PermissionDenied,
    InvalidUtf8,
    IoError,
}

/*
Reads raw text content from a file given its path string.

Takes:
	path (&str): The file path string to read.

Gives:
	Result<String, IngestError>: Ok containing raw file text content string or IngestError.
*/
fn readFile(path: &str) -> Result<String, IngestError>
{
    if path.is_empty() {
        return Err(IngestError::IoError);
    }
    match fs::read(path) {
        Ok(bytes) => {
            match String::from_utf8(bytes) {
                Ok(content) => Ok(content),
                Err(_) => Err(IngestError::InvalidUtf8),
            }
        }
        Err(err) => {
            if err.kind() == std::io::ErrorKind::PermissionDenied {
                Err(IngestError::PermissionDenied)
            } else {
                Err(IngestError::IoError)
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
    let path = Path::new(targetPath);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|err| format!("Failed to create parent directory '{}': {}", parent.display(), err))?;
        }
    }

    fs::write(path, content).map_err(|err| format!("Failed to write content to file '{}': {}", targetPath, err))
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
	Result<IngestedPayload, IngestError>: Ok containing populated IngestedPayload struct or IngestError.
*/
pub fn giveFilePayload(path: &str) -> Result<IngestedPayload, IngestError>
{
    if path.is_empty() {
        return Err(IngestError::IoError);
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
        let parentDir = getParentDir(path).map_err(|_| IngestError::IoError)?;
        let fileName = pathObj
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or(IngestError::IoError)?;

        let loomPath = resolvePath(&parentDir, fileName).map_err(|_| IngestError::IoError)?;
        let content = readFile(&loomPath.absolute)?;

        if path.ends_with(".thread") {
            mapThreadFileEntry(&loomPath.absolute, content, &mut payload.threadContent, &mut payload.threadFileMapping);
        } else if path.ends_with(".fabric") {
            mapFabricFileEntry(&loomPath.absolute, content.clone(), &mut payload.fabricFileMapping);
            payload.fabricContent = Some(content);
        } else {
            return Err(IngestError::IoError);
        }

        return Ok(payload);
    }

    /* Directory Ingestion via PathResolver (resolvePath per directory entry) using loomPath.absolute */
    if pathObj.is_dir() {
        let entries = fs::read_dir(pathObj).map_err(|err| {
            if err.kind() == std::io::ErrorKind::PermissionDenied {
                IngestError::PermissionDenied
            } else {
                IngestError::IoError
            }
        })?;

        for entry in entries {
            if let Ok(dirEntry) = entry {
                let entryPath = dirEntry.path();
                if entryPath.is_file() {
                    let fileName = dirEntry.file_name();
                    let fileNameStr = fileName.to_str().unwrap_or("");

                    if fileNameStr.ends_with(".thread") {
                        if let Ok(loomPath) = resolvePath(path, fileNameStr) {
                            let content = readFile(&loomPath.absolute)?;
                            mapThreadFileEntry(&loomPath.absolute, content, &mut payload.threadContent, &mut payload.threadFileMapping);
                        }
                    } else if fileNameStr.ends_with(".fabric") {
                        if let Ok(loomPath) = resolvePath(path, fileNameStr) {
                            let content = readFile(&loomPath.absolute)?;
                            mapFabricFileEntry(&loomPath.absolute, content.clone(), &mut payload.fabricFileMapping);
                            payload.fabricContent = Some(content);
                        }
                    }
                }
            }
        }

        return Ok(payload);
    }

    Err(IngestError::IoError)
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
    Tests readFile with a directory path (should return IngestError).

    Takes:
    	None.

    Gives:
    	(): Unit type.
    */
    #[test]
    fn testReadFileWithDirectoryPath() -> ()
    {
        let result = readFile("helpers");
        assert_eq!(result, Err(IngestError::IoError));
    }
}
