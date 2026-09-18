"""
File Name: server.py
Purpose: FastMCP specification navigator and in-memory query engine for Loom Thread and Fabric software specifications.
"""

import os
import re
from pathlib import Path
from typing import Dict, List, Optional, Tuple, Any
from fastmcp import FastMCP

FILE_CACHE: Dict[str, str] = {
}
MTIME_CACHE: Dict[str, float] = {
}
MCP_SERVER: FastMCP = FastMCP("Loom Spec Navigator")
NOT_FOUND_MESSAGE: str = "NotFound"


def syncMemory(directory: str) -> None:
    """
    Synchronizes in-memory text cache from disk for all .thread and .fabric specification files based on file modification timestamps.

    Takes:
    	directory (str): Absolute or relative filesystem directory path to scan.

    Gives:
    	None: Synchronizes in-memory caches in place.
    """
    dirPath = Path(directory)
    if not dirPath.exists() or not dirPath.is_dir():
        return

    threadFiles = [f for f in dirPath.glob("**/*.thread")]
    fabricFiles = [f for f in dirPath.glob("**/*.fabric")]
    currentFiles = threadFiles + fabricFiles

    for fileEntry in currentFiles:
        fileStr = str(fileEntry.resolve())
        try:
            currentMtime = fileEntry.stat().st_mtime
            if fileStr not in MTIME_CACHE or MTIME_CACHE[fileStr] < currentMtime:
                FILE_CACHE[fileStr] = fileEntry.read_text(encoding="utf-8")
                MTIME_CACHE[fileStr] = currentMtime
        except Exception:
            pass


sync_memory = syncMemory


def extractBalancedBlock(text: str, startIndex: int) -> Tuple[str, int, int]:
    """
    Extracts a balanced curly brace block from text starting from the opening brace.

    Takes:
    	text (str): Source text content string.
    	startIndex (int): Character start offset to begin scanning.

    Gives:
    	Tuple[str, int, int]: Extracted block string, start offset, and end offset.
    """
    openBrace = text.find("{", startIndex)
    if openBrace == -1:
        return ("", startIndex, startIndex)

    depth = 0
    inString = False
    escapeNext = False

    for i in range(openBrace, len(text)):
        char = text[i]

        if escapeNext:
            escapeNext = False
            continue

        if char == "\\" and inString:
            escapeNext = True
            continue

        if char == '"':
            inString = not inString
            continue

        if not inString:
            if char == "{":
                depth += 1
            elif char == "}":
                depth -= 1
                if depth == 0:
                    return (text[startIndex:i + 1].strip(), startIndex, i + 1)

    return (text[startIndex:].strip(), startIndex, len(text))


def findEntityBlocks(content: str) -> List[Dict[str, Any]]:
    """
    Parses all top-level Thread entities and their inner member blocks from file text content.

    Takes:
    	content (str): Full text content of a specification file.

    Gives:
    	List[Dict[str, Any]]: Parsed entity objects containing type, name, full content, and member list.
    """
    entityPattern = re.compile(
        r"^[ \t]*(Feature|Component|Storage|Protocol)\s+([A-Za-z0-9_]+)",
        re.MULTILINE
    )
    memberPattern = re.compile(
        r"^[ \t]*(Contract|Model|Table|Channel|Scenario Outline|Scenario|Rule)\s+([A-Za-z0-9_\"\- ]+?)(?=\s*\{)",
        re.MULTILINE
    )

    entities: List[Dict[str, Any]] = []

    for match in entityPattern.finditer(content):
        eType = match.group(1)
        eName = match.group(2)
        startPos = match.start()

        linesBefore = content[:startPos].splitlines()
        idx = len(linesBefore) - 1
        while idx >= 0:
            lineStr = linesBefore[idx].strip()
            if lineStr.startswith("@") or lineStr.startswith("/*") or lineStr.startswith("*") or lineStr.startswith("!"):
                idx -= 1
            else:
                break
        decoratorStart = sum(len(l) + 1 for l in linesBefore[:idx + 1]) if idx >= 0 else startPos

        blockText, _, _ = extractBalancedBlock(content, decoratorStart)

        members: List[Dict[str, str]] = []
        for mMatch in memberPattern.finditer(blockText):
            mType = mMatch.group(1)
            rawMName = mMatch.group(2).strip()
            mName = rawMName.replace('"', "")
            mStart = mMatch.start()

            mLinesBefore = blockText[:mStart].splitlines()
            mIdx = len(mLinesBefore) - 1
            while mIdx >= 0:
                mLineStr = mLinesBefore[mIdx].strip()
                if mLineStr.startswith("@") or mLineStr.startswith("/*") or mLineStr.startswith("*") or mLineStr.startswith("!"):
                    mIdx -= 1
                else:
                    break
            mDecStart = sum(len(l) + 1 for l in mLinesBefore[:mIdx + 1]) if mIdx >= 0 else mStart
            mBlockText, _, _ = extractBalancedBlock(blockText, mDecStart)

            members.append({
                "type": mType,
                "name": mName,
                "content": mBlockText
            })

        entities.append({
            "type": eType,
            "name": eName,
            "fullContent": blockText,
            "members": members
        })

    return entities


@MCP_SERVER.tool(name="loomList")
def loomList(directory: str, entityName: str = "all") -> str:
    """
    Lists specification entities and their respective members from memory.
    If entityName is 'all', lists all entities and members across all files.
    If entityName is specified, lists all members for that specific entity.

    Takes:
    	directory (str): Path to specification directory.
    	entityName (str): Target entity name or 'all' to list everything.

    Gives:
    	str: Formatted entity and member hierarchy summary, or NotFound.
    """
    syncMemory(directory)

    if not FILE_CACHE:
        return "No specification files found in directory."

    targetEntityLower = entityName.strip().lower()

    if targetEntityLower == "all":
        summaryLines: List[str] = []
        for filePath, content in FILE_CACHE.items():
            fileName = Path(filePath).name
            entities = findEntityBlocks(content)

            if not entities and filePath.endswith(".fabric"):
                groupPattern = re.compile(r'group\s+"([^"]+)"', re.MULTILINE)
                systemPattern = re.compile(r'system\s+"([^"]+)"', re.MULTILINE)
                groups = groupPattern.findall(content)
                systems = systemPattern.findall(content)

                if systems or groups:
                    summaryLines.append(f"File: {fileName}")
                    for s in systems:
                        summaryLines.append(f'  - System: "{s}"')
                    for g in groups:
                        summaryLines.append(f'    * Group: "{g}"')
                continue

            if entities:
                summaryLines.append(f"File: {fileName}")
                for entity in entities:
                    summaryLines.append(f"  - {entity['type']}: {entity['name']}")
                    for member in entity["members"]:
                        summaryLines.append(f"    * {member['type']}: {member['name']}")

        return "\n".join(summaryLines) if summaryLines else "No specification entities found."

    foundEntity: Optional[Dict[str, Any]] = None
    foundFile: str = ""

    for filePath, content in FILE_CACHE.items():
        entities = findEntityBlocks(content)
        for entity in entities:
            if entity["name"].lower() == targetEntityLower:
                foundEntity = entity
                foundFile = Path(filePath).name
                break
        if foundEntity:
            break

    if not foundEntity:
        return NOT_FOUND_MESSAGE

    resultLines = [f"- {foundEntity['type']}: {foundEntity['name']} (in {foundFile})"]
    if foundEntity["members"]:
        for member in foundEntity["members"]:
            resultLines.append(f"  * {member['type']}: {member['name']}")
    else:
        resultLines.append("  (No child members declared)")

    return "\n".join(resultLines)


@MCP_SERVER.tool(name="loomGet")
def loomGet(directory: str, entityName: str, memberName: str = "") -> str:
    """
    Extracts the full raw text specification of an entity or a specific member of that entity.
    Returns NotFound if the entity does not exist or if the member does not belong to that entity.

    Takes:
    	directory (str): Path to specification directory.
    	entityName (str): Target entity name (e.g. AuthService).
    	memberName (str): Optional member name (e.g. login).

    Gives:
    	str: Exact raw text block of the requested entity or member, or NotFound.
    """
    syncMemory(directory)

    if not FILE_CACHE:
        return NOT_FOUND_MESSAGE

    targetEntityClean = entityName.strip()
    targetMemberClean = memberName.strip() if memberName else ""

    foundEntity: Optional[Dict[str, Any]] = None
    sourceFile: str = ""

    for filePath, content in FILE_CACHE.items():
        entities = findEntityBlocks(content)
        for entity in entities:
            if entity["name"].lower() == targetEntityClean.lower():
                foundEntity = entity
                sourceFile = filePath
                break
        if foundEntity:
            break

    if not foundEntity:
        return NOT_FOUND_MESSAGE

    if not targetMemberClean:
        return f"/* Source: {sourceFile} */\n\n{foundEntity['fullContent']}"

    for member in foundEntity["members"]:
        if member["name"].lower() == targetMemberClean.lower():
            return f"/* Source: {sourceFile} | Entity: {foundEntity['name']} */\n\n{member['content']}"

    return NOT_FOUND_MESSAGE


@MCP_SERVER.tool(name="loomGrep")
def loomGrep(directory: str, pattern: str, lineRadius: int = 5) -> str:
    """
    Performs regex pattern search across all in-memory specifications with customizable line radius context.

    Takes:
    	directory (str): Path to specification directory.
    	pattern (str): Regular expression pattern string to match.
    	lineRadius (int): Number of context lines to display above and below matching lines.

    Gives:
    	str: Formatted search results with matched line numbers and contextual snippets.
    """
    syncMemory(directory)

    if not FILE_CACHE:
        return f"No specification files found in directory '{directory}'."

    try:
        compiledRegex = re.compile(pattern, re.IGNORECASE)
    except re.error as parseErr:
        return f"Invalid regex pattern '{pattern}': {parseErr}"

    results: List[str] = []

    for filePath, content in FILE_CACHE.items():
        lines = content.splitlines()
        matchedIndices: List[int] = []

        for idx, line in enumerate(lines):
            if compiledRegex.search(line):
                matchedIndices.append(idx)

        if not matchedIndices:
            continue

        fileMatches: List[str] = []
        for matchIdx in matchedIndices:
            startIdx = max(0, matchIdx - lineRadius)
            endIdx = min(len(lines), matchIdx + lineRadius + 1)

            contextLines: List[str] = []
            for curIdx in range(startIdx, endIdx):
                lineNum = curIdx + 1
                prefix = ">" if curIdx == matchIdx else " "
                contextLines.append(f"  {prefix} {lineNum:4d} | {lines[curIdx]}")

            fileMatches.append("\n".join(contextLines))

        results.append(f"File: {filePath}\n" + "\n\n".join(fileMatches))

    if not results:
        return f"No matches found for pattern '{pattern}'."

    return "\n\n" + ("=" * 60) + "\n\n".join(results)


if __name__ == "__main__":
    MCP_SERVER.run()
