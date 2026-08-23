/*
File Name: file_ingester.rs
Purpose: Implementation of the FileIngester helper component and payload data models for the Loom compiler.
*/

#![allow(non_snake_case)]

/*
Represents a raw file payload containing path and string content buffer.
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilePayload
{
    pub path: String,
    pub content: String,
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
    match std::fs::read(path) {
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
Reads file content and packages it into a FilePayload using the provided path string.

Takes:
	path (&str): The file path string (relative or absolute).

Gives:
	Result<FilePayload, IngestError>: Ok containing populated FilePayload struct or IngestError.
*/
pub fn giveFilePayload(path: &str) -> Result<FilePayload, IngestError>
{
    let content = readFile(path)?;
    Ok(FilePayload {
        path: path.to_string(),
        content,
    })
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
