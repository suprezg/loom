# Loom Valid Examples Reference & Architecture Blueprint

This document details the 5 canonical reference specification files located in `examples/valid/`. Use these files as patterns for creating, modifying, and reviewing Loom specifications.

---

## 1. `examples/valid/authentication.thread` (Feature Entity)

Represents behavior-driven scenarios, acceptance criteria, and user flows.

```loom
Feature Authentication
{
    !Note [[
        Governs user identity verification, session creation, and security compliance.
        This feature enforces OAuth2 tokens and strict password complexity rules.
    ]]

    !Diagram SequenceLogin
    [[
        ```mermaid
        sequenceDiagram
            autonumber
            actor User as User
            participant LoginUI as LoginUI
            participant AuthService as AuthService
            participant VaultEngine as VaultEngine

            User->>LoginUI: Visits website
            LoginUI-->>User: Serves login page
            User->>LoginUI: Clicks login button
            LoginUI->>+AuthService: login("user", "pass")
            AuthService->>+VaultEngine: verifyManifest("manifest.json")
            VaultEngine-->>-AuthService: ManifestStatus::Valid
            AuthService-->>-LoginUI: AuthResult::Success
            LoginUI-->>User: Displays success toast and redirects to dashboard
        ```
    ]]

    !Diagram StateLogin
    [[
        ```mermaid
        stateDiagram-v2
            [*] --> LoggedOut
            LoggedOut --> Authenticating : ClickLogin
            Authenticating --> LoggedIn : AuthResult::Success
            Authenticating --> LoggedOut : AuthResult::InvalidCredentials
            LoggedIn --> LoggedOut : ClickLogout
        ```
    ]]

    Background
    {
        Given "the system authentication service is initialized and running"
        And "the local encrypted database connection pool is active"
    }

    Rule "User Login Workflow"
    {
        @component(AuthService::login)
        @component(VaultEngine::verifyManifest)
        @diagram(SequenceLogin)
        @diagram(StateLogin)
        Scenario SuccessfulUserLogin
        {
            Title "Successful user login with valid credentials"

            Given "a registered user is on the login page"
            When "the user enters valid credentials and clicks login"
            Then "the system authenticates the user and redirects to the main dashboard"
        }

        @component(AuthService::login)
        Scenario UnsuccessfulUserLogin
        {
            Title "Unsuccessful user login with invalid password"

            Given "a registered user is on the login page"
            When "the user enters an incorrect password"
            Then "the system displays an error toast message 'Invalid credentials'"
        }

        @component(AuthService::login)
        Scenario Outline MultiRoleLoginValidation
        {
            Title "Validating login credentials against multiple user roles"

            Given "a user with username <username> and password <password>"
            When "the user submits the login form"
            Then "the authentication result should be <status>"

            Examples
            {
                | "username" | "password" | "status"  |
                | "alice"    | "pass123"  | "success" |
                | "bob"      | "wrong"    | "failure" |
            }
        }
    }
}
```

---

## 2. `examples/valid/auth_service.thread` (Component Entity)

Defines business logic components (`AuthService`, `VaultEngine`), data models, and formal behavioral contracts.

```loom
Component AuthService
{
    !Note [[
        Core authentication service managing user identity validation, credential checks,
        and session token generation across the application lifetime.
    ]]

    !Diagram LoginFlow
    [[
        ```mermaid
        flowchart TD
            Start([Start login]) --> FetchHash[Fetch password hash from AppStorage::users]
            FetchHash --> VerifyHash{Verify Hash?}
            VerifyHash -- Invalid --> ReturnInvalid[Return AuthResult::InvalidCredentials]
            VerifyHash -- Valid --> CallVault[Call VaultEngine::verifyManifest]
            CallVault --> EmitToken[Broadcast token to AuthProtocol::TokenBroadcastPipe]
            EmitToken --> ReturnSuccess[Return AuthResult::Success]
        ```
    ]]

    Invariants
    {
        "SecurityBoundary: Failed login attempts MUST be logged and rate-limited."
        "CredentialPrivacy: User passwords MUST NEVER be stored or transmitted in plaintext."
    }

    Model AuthResult
    {
        Type "Enum"
        Members
        {
            "Success"
            "InvalidCredentials"
            "LockedOut"
        }
    }

    @storage(AppStorage::users)
    Model Credentials
    {
        Type "Struct"
        Members
        {
            "username": "String (min_length=3, max_length=64)"
            "password": "String (min_length=8)"
        }
    }

    @storage(AppStorage::audit_logs)
    Model AuditEvent
    {
        Type "Struct"
        Members
        {
            "user_id": "UUID"
            "event_type": "String (max_length=64)"
            "ip_address": "String (max_length=45)"
            "timestamp": "Date"
        }
    }

    @storage(AppStorage::users)
    @storage(AppStorage::audit_logs)
    @protocol(AuthProtocol::TokenBroadcastPipe)
    @protocol(AuthProtocol::AuditStream)
    @diagram(LoginFlow)
    Contract login
    {
        Signature "login(credentials: &Credentials)" -> "AuthResult"

        Requires "User account record exists in AppStorage::users database."

        Ensures "Returns AuthResult::Success if credentials match and VaultEngine verifies manifest."

        Precondition
        {
            "Username string is valid UTF-8."
            "Password string length is greater than or equal to 8."
        }

        Postcondition
        {
            "Session token is generated and broadcast to AuthProtocol::TokenBroadcastPipe."
            "Security audit event is logged to AuthProtocol::AuditStream."
        }

        Process
        {
            1. "Fetch user password hash from AppStorage::users."
            2. "Verify password against stored bcrypt hash."
            3. "Call VaultEngine::verifyManifest to check binary compliance."
            4. "Emit session token broadcast and return AuthResult::Success."
        }

        Errors
        {
            "InvalidCredentials"
            "AccountLocked"
            "SystemError"
        }
    }
}

Component VaultEngine
{
    !Note [[
        ACL Vault engine responsible for verifying manifest signatures, validating plugin bundles,
        and isolating sandbox execution permissions across the application lifetime.
    ]]

    !Diagram VerifyProcess
    [[
        ```mermaid
        flowchart TD
            Start([Start verifyManifest]) --> Read[Read raw bytes from disk]
            Read --> Decode{Decode JSON?}
            Decode -- Failed --> InvalidFormat[Return InvalidFormat Error]
            Decode -- Success --> VerifySig{Verify Signature?}
            VerifySig -- Failed --> InvalidSig[Return InvalidSignature Error]
            VerifySig -- Valid --> ReturnValid[Return ManifestStatus::Valid]
        ```
    ]]

    Invariants
    {
        "SecurityBoundary: Non-compliant binaries MUST NOT execute in main process thread."
        "SignatureIntegrity: Manifest SHA-256 hash MUST match developer public key."
    }

    Model ManifestStatus
    {
        Type "Enum"
        Members
        {
            "Valid"
            "InvalidSignature"
            "MissingArtifacts"
        }
    }

    @storage(AppStorage::plugins)
    Model PluginManifest
    {
        Type "Struct"
        Members
        {
            "id": "UUID"
            "name": "String (min_length=1, max_length=128)"
            "version": "String"
            "entrypoint": "String"
        }
    }

    @storage(AppStorage::plugins)
    @protocol(AuthProtocol::AuditStream)
    @diagram(VerifyProcess)
    Contract verifyManifest
    {
        Signature "verifyManifest(path: &str)" -> "Result<ManifestStatus, VaultError>"

        Requires "File system handle to manifest path is readable."

        Ensures "Returns ManifestStatus::Valid if signature and structure comply."

        Precondition
        {
            "Path string is non-empty and well-formed UTF-8."
        }

        Postcondition
        {
            "Manifest handle is closed immediately after evaluation."
        }

        Process
        {
            1. "Read raw manifest bytes using std::fs::read."
            2. "Decode JSON payload into PluginManifest struct."
            3. "Verify cryptographic signature against developer public key."
            4. "Return ManifestStatus::Valid on clean match."
        }

        Errors
        {
            "InvalidSignature"
            "FileNotFound"
            "CorruptedState"
        }
    }
}
```

---

## 3. `examples/valid/auth_protocol.thread` (Protocol Entity)

Specifies communication channels, message formats, and pub/sub routing.

```loom
Protocol AuthProtocol
{
    !Note [[
        Defines inter-component communication channels for authentication events,
        token broadcasting, and security audit streaming.
    ]]

    !Diagram TokenStreamFlow
    [[
        ```mermaid
        sequenceDiagram
            autonumber
            participant AuthService as AuthService::login
            participant Bus as TokenBroadcastPipe
            participant VaultEngine as VaultEngine::verifyManifest

            AuthService->>Bus: Publish Token (AuthService::AuthResult)
            Bus-->>VaultEngine: Deliver Token
        ```
    ]]

    !Diagram AuditFlow
    [[
        ```mermaid
        flowchart TD
            Event([Security Event]) --> Stream[AuthProtocol::AuditStream]
            Stream --> Logger[AppStorage::audit_logs]
        ```
    ]]

    @feature(Authentication::SuccessfulUserLogin)
    @component(AuthService::login)
    @component(AuthService::AuthResult)
    @storage(AppStorage::users)
    @diagram(TokenStreamFlow)
    Channel TokenBroadcastPipe
    {
        Pattern "AsyncBroadcast"
        Transport "tokio::sync::broadcast (Capacity: 1024)"
        Sender "AuthService::login"
        Receiver "VaultEngine::verifyManifest"
        Payload "AuthService::AuthResult"

        Errors
        {
            "ChannelFull"
            "ChannelClosed"
        }
    }

    @feature(Authentication::SuccessfulUserLogin)
    @component(AuthService::login)
    @component(AuthService::AuditEvent)
    @storage(AppStorage::audit_logs)
    @diagram(AuditFlow)
    Channel AuditStream
    {
        Pattern "PointToPoint"
        Transport "IPC Queue"
        Sender "AuthService::login"
        Receiver "AppStorage::audit_logs"
        Payload "AuthService::AuditEvent"
    }
}
```

---

## 4. `examples/valid/app_storage.thread` (Storage Entity)

Defines persistence schema, indexes, and database relations.

```loom
Storage AppStorage
{
    !Note [[
        Local encrypted storage engine for persistent application metadata, user account credentials,
        and security compliance audit logs.
    ]]

    !Diagram SchemaERD
    [[
        ```mermaid
        erDiagram
            users ||--o{ audit_logs : generates
            users ||--o{ plugins : installs
        ```
    ]]

    Engine "SQLite3 / SQLCipher"

    @feature(Authentication::SuccessfulUserLogin)
    @component(AuthService::login)
    @component(AuthService::Credentials)
    @diagram(SchemaERD)
    Table users
    {
        Fields
        {
            "id": "UUID [PrimaryKey, NonNull]"
            "username": "String(64) [NonNull, Unique]"
            "password_hash": "String(255) [NonNull]"
            "status": "String(32) [NonNull, Default('active')]"
            "created_at": "Date [NonNull, Default(NOW)]"
        }

        Indexes
        {
            idx_user_username (username)
            idx_user_status (status)
        }
    }

    @feature(Authentication::SuccessfulUserLogin)
    @component(AuthService::login)
    @component(AuthService::AuditEvent)
    @protocol(AuthProtocol::AuditStream)
    @diagram(SchemaERD)
    Table audit_logs
    {
        Fields
        {
            "id": "UUID [PrimaryKey, NonNull]"
            "user_id": "UUID [NonNull, ForeignKey(users.id)]"
            "event_type": "String(64) [NonNull]"
            "ip_address": "String(45) [NonNull]"
            "timestamp": "Date [NonNull, Default(NOW)]"
        }

        Indexes
        {
            idx_audit_user (user_id)
            idx_audit_timestamp (timestamp)
        }

        Relations
        {
            users.id 1:N audit_logs.user_id
        }
    }

    @feature(Authentication::SuccessfulUserLogin)
    @component(VaultEngine::verifyManifest)
    @component(VaultEngine::PluginManifest)
    @diagram(SchemaERD)
    Table plugins
    {
        Fields
        {
            "id": "UUID [PrimaryKey, NonNull]"
            "name": "String(128) [NonNull, Unique]"
            "version": "String(32) [NonNull]"
            "entrypoint": "String(255) [NonNull]"
            "installed_at": "Date [NonNull, Default(NOW)]"
        }

        Indexes
        {
            idx_plugin_name (name)
        }
    }
}
```

---

## 5. `examples/valid/system.fabric` (Macro Architecture Blueprint)

Defines system boundaries, canvas clusters, and architectural connection flows.

```fabric
system "Loom Macro System Architecture"

# 1. Canvas Cluster Groups (Visual Bounding Boxes on Home Page Canvas)
group "Identity & Access Control"
{
    Feature.Authentication
    Component.AuthService
    Component.VaultEngine
}

group "Persistence & Security Audit"
{
    Storage.AppStorage::users
    Storage.AppStorage::audit_logs
    Storage.AppStorage::plugins
    Protocol.AuthProtocol::AuditStream
}

# 2. Macro Connections & Topology Edges
Feature.Authentication -> Component.AuthService::login : "Triggers user login workflow"
Component.AuthService::login -> Component.VaultEngine::verifyManifest : "Validates binary compliance"
Component.AuthService::login -> Storage.AppStorage::users : "Queries bcrypt password hash"
Component.AuthService::login -> Protocol.AuthProtocol::TokenBroadcastPipe : "Broadcasts AuthResult::Success"
Component.AuthService::login -> Protocol.AuthProtocol::AuditStream : "Emits security audit event"
Protocol.AuthProtocol::AuditStream -> Storage.AppStorage::audit_logs : "Persists audit trail"
Component.VaultEngine::verifyManifest -> Storage.AppStorage::plugins : "Queries installed plugins"
```
