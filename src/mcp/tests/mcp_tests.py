"""
File Name: mcp_tests.py
Purpose: Integration and unit test suite for Loom FastMCP server methods (loomList, loomGet, loomGrep).
"""

import sys
from pathlib import Path

MCP_DIR = str(Path(__file__).resolve().parent.parent)
if MCP_DIR not in sys.path:
    sys.path.insert(0, MCP_DIR)

import unittest
from server import loomList, loomGet, loomGrep, syncMemory, FILE_CACHE, NOT_FOUND_MESSAGE

VALID_EXAMPLES_DIR: str = str(Path(__file__).resolve().parent.parent.parent.parent / "examples" / "valid")


class LoomMcpTests(unittest.TestCase):
    """
    Test suite verifying in-memory synchronization, listing, extraction, and pattern search capabilities of Loom FastMCP server.
    """

    def setUp(self) -> None:
        """
        Initializes memory cache before each test case execution.

        Takes:
        	None.

        Gives:
        	None.
        """
        syncMemory(VALID_EXAMPLES_DIR)

    def testLoomListAll(self) -> None:
        """
        Tests loomList with 'all' returning all specification files, entities, and members.

        Takes:
        	None.

        Gives:
        	None.
        """
        result = loomList(VALID_EXAMPLES_DIR, "all")
        self.assertIn("File: authentication.thread", result)
        self.assertIn("Feature: Authentication", result)
        self.assertIn("Scenario: SuccessfulUserLogin", result)
        self.assertIn("File: auth_service.thread", result)
        self.assertIn("Component: AuthService", result)
        self.assertIn("Contract: login", result)
        self.assertIn("Component: VaultEngine", result)
        self.assertIn("Contract: verifyManifest", result)
        self.assertIn("File: app_storage.thread", result)
        self.assertIn("Storage: AppStorage", result)
        self.assertIn("Table: users", result)
        self.assertIn("File: auth_protocol.thread", result)
        self.assertIn("Protocol: AuthProtocol", result)
        self.assertIn("Channel: TokenBroadcastPipe", result)

    def testLoomListSingleEntity(self) -> None:
        """
        Tests loomList with a specific entity name returning only that entity and its members.

        Takes:
        	None.

        Gives:
        	None.
        """
        result = loomList(VALID_EXAMPLES_DIR, "AuthService")
        self.assertIn("Component: AuthService", result)
        self.assertIn("Contract: login", result)
        self.assertIn("Model: Credentials", result)
        self.assertNotIn("VaultEngine", result)
        self.assertNotIn("Authentication", result)

    def testLoomListNotFound(self) -> None:
        """
        Tests loomList with a non-existent entity returning NotFound.

        Takes:
        	None.

        Gives:
        	None.
        """
        result = loomList(VALID_EXAMPLES_DIR, "NonExistentEntity")
        self.assertEqual(result, NOT_FOUND_MESSAGE)

    def testLoomGetFullEntity(self) -> None:
        """
        Tests loomGet with entityName only returning the complete entity block.

        Takes:
        	None.

        Gives:
        	None.
        """
        result = loomGet(VALID_EXAMPLES_DIR, "AuthService")
        self.assertNotEqual(result, NOT_FOUND_MESSAGE)
        self.assertIn("Component AuthService", result)
        self.assertIn("Contract login", result)
        self.assertIn("Model Credentials", result)

    def testLoomGetSpecificMember(self) -> None:
        """
        Tests loomGet with entityName and valid memberName returning only that member block.

        Takes:
        	None.

        Gives:
        	None.
        """
        result = loomGet(VALID_EXAMPLES_DIR, "AuthService", "login")
        self.assertNotEqual(result, NOT_FOUND_MESSAGE)
        self.assertIn("Contract login", result)
        self.assertIn('Signature "login(credentials: &Credentials)"', result)
        self.assertNotIn("Model Credentials", result)
        self.assertNotIn("Component VaultEngine", result)
        self.assertNotIn("verifyManifest(path:", result)

    def testLoomGetWrongEntity(self) -> None:
        """
        Tests loomGet with a non-existent entity returning NotFound.

        Takes:
        	None.

        Gives:
        	None.
        """
        result = loomGet(VALID_EXAMPLES_DIR, "UnknownEntity")
        self.assertEqual(result, NOT_FOUND_MESSAGE)

    def testLoomGetWrongMember(self) -> None:
        """
        Tests loomGet with valid entity but invalid member returning NotFound.

        Takes:
        	None.

        Gives:
        	None.
        """
        result = loomGet(VALID_EXAMPLES_DIR, "AuthService", "nonExistentMethod")
        self.assertEqual(result, NOT_FOUND_MESSAGE)

    def testLoomGetWrongEntityRightMember(self) -> None:
        """
        Tests loomGet when member exists in system but does not belong to specified entity.

        Takes:
        	None.

        Gives:
        	None.
        """
        result = loomGet(VALID_EXAMPLES_DIR, "AuthService", "verifyManifest")
        self.assertEqual(result, NOT_FOUND_MESSAGE)

    def testLoomGrepMatch(self) -> None:
        """
        Tests loomGrep finding matching pattern occurrences with surrounding line context.

        Takes:
        	None.

        Gives:
        	None.
        """
        result = loomGrep(VALID_EXAMPLES_DIR, "TokenBroadcastPipe", 2)
        self.assertIn("TokenBroadcastPipe", result)
        self.assertIn("auth_service.thread", result)
        self.assertIn("auth_protocol.thread", result)
        self.assertIn("system.fabric", result)
        self.assertIn(">", result)

    def testLoomGrepNoMatch(self) -> None:
        """
        Tests loomGrep returning no matches message when pattern does not exist.

        Takes:
        	None.

        Gives:
        	None.
        """
        result = loomGrep(VALID_EXAMPLES_DIR, "ZzZzNonExistentString12345", 5)
        self.assertIn("No matches found", result)

    def testLoomGrepInvalidRegex(self) -> None:
        """
        Tests loomGrep handling malformed regex pattern gracefully.

        Takes:
        	None.

        Gives:
        	None.
        """
        result = loomGrep(VALID_EXAMPLES_DIR, "[unclosed_regex", 5)
        self.assertIn("Invalid regex pattern", result)


if __name__ == "__main__":
    unittest.main()
