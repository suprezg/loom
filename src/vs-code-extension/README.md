# Loom VS Code Extension

Official Visual Studio Code extension providing language support, context-aware autocompletion, entity boilerplates, and syntax highlighting for Loom specification files (`.fabric` and `.thread`).

---

## Extension Capabilities

* **Syntax Highlighting**: Comprehensive TextMate grammar definitions for Loom Fabric blueprints (`.fabric`) and Loom Thread specifications (`.thread`).
* **Context-Aware Autocompletion**: Smart completion provider that analyzes line block hierarchy—suggesting only valid keywords, sub-blocks, and decorators for the active entity domain (`Feature`, `Component`, `Storage`, `Protocol`, or `Fabric`).
* **Entity & Sub-Block Boilerplates**: Full code snippet templates triggered by entity prefix (`$Thread.Feature`, `$Thread.Component`, `$Thread.Storage`, `$Thread.Protocol`, `$Fabric.System`) or sub-block keyword (`Model`, `Contract`, `Table`, `Channel`, `Examples`, `Rule`, `Scenario`, `Scenario Outline`, `Background`, `!Note`, `!Diagram`).
* **File Icons**: Custom file icon badges for `.fabric` blueprints and `.thread` specifications in the workspace explorer.
* **Bracket Matching & Comments**: Configured bracket pairs (`{}` `[]` `""`) and comment toggling (`#` line comments and `/* */` block comments).

---

## Boilerplate Snippet Triggers

### 1. Top-Level Entity Boilerplates
Type the trigger prefix anywhere at the top-level of a file:

| Trigger Prefix | Description |
| --- | --- |
| `$Thread.Feature` or `Feature` | Generates a complete Feature entity with `!Note` and initial `Scenario` steps. |
| `$Thread.Component` or `Component` | Generates a complete Component entity with `Invariants`, `Model`, and `Contract` blocks. |
| `$Thread.Storage` or `Storage` | Generates a complete Storage entity with `Engine`, `Table`, `Fields`, and `Indexes`. |
| `$Thread.Protocol` or `Protocol` | Generates a complete Protocol entity with a `Channel` (Pattern, Transport, Sender, Receiver, Payload). |
| `$Fabric.System` or `system` | Generates a complete `.fabric` system blueprint with `group` clusters and entity connections. |

### 2. Sub-Block Boilerplates
Type the keyword inside the corresponding parent entity:

| Trigger | Parent Entity | Generated Structure |
| --- | --- | --- |
| `Model` | `Component` | `Model ModelName { Type "Struct" Members { "field": "Type" } }` |
| `Contract` | `Component` | `Contract contractName { Signature "..." Requires "..." Ensures "..." Process { 1. "Step" } }` |
| `Table` | `Storage` | `Table tableName { Fields { "id": "UUID" } Indexes { Idx (id) } }` |
| `Channel` | `Protocol` | `Channel ChannelName { Pattern "..." Transport "..." Sender "..." Receiver "..." Payload "..." }` |
| `Rule` | `Feature` | `Rule "Description" { Scenario "Name" { Given ... When ... Then ... } }` |
| `Scenario` | `Feature` | `Scenario "Name" { Given ... When ... Then ... }` |
| `Scenario Outline` | `Feature` | `Scenario Outline "Name" { Given ... Examples { \| "param" \| } }` |
| `Examples` | `Scenario Outline` | `Examples { \| "param1" \| "param2" \| }` |
| `!Note` | Universal | `!Note [ Documentation note text ]` |
| `!Diagram` | Universal | `!Diagram DiagramName [ ```mermaid sequenceDiagram ... ``` ]` |

---

## Loom 2.0 Language Cheat Sheet

### 1. Loom Thread Specifications (`.thread`)

#### Entities
* **`Feature <Ident> { ... }`**: Defines behavioral user scenarios and Gherkin steps (`Given`, `When`, `Then`, `And`, `But`).
* **`Component <Ident> { ... }`**: Defines low-level service contracts, invariant assertions, and data models (`Model`, `Contract`, `Invariants`).
* **`Storage <Ident> { ... }`**: Defines database storage tables, field types, indexes, and relations (`Engine`, `Table`, `Fields`, `Indexes`, `Relations`).
* **`Protocol <Ident> { ... }`**: Defines communication channels, transports, senders, receivers, and message payloads (`Channel`, `Pattern`, `Transport`, `Sender`, `Receiver`, `Payload`).

#### Universal Decorators
* `@component(ComponentEntity::Member)`
* `@storage(StorageEntity::Table)`
* `@protocol(ProtocolEntity::Channel)`
* `@feature(FeatureEntity)`
* `@diagram(DiagramName)`

---

### 2. Loom Fabric Blueprints (`.fabric`)

#### Structure
* **`system "SystemName"`**: Top-level system name header.
* **`group "GroupName" { Feature.Auth Component.Service }`**: Clusters thread entities into visual macro-architecture groups.
* **`Feature.Auth -> Component.Service : "Triggers"`**: Direct connection edge between entities with optional label.

---

## Example Specifications

### 1. Thread Specification (`authentication.thread`)
```thread
Feature Authentication
{
    !Note [ User authentication feature specification ]

    Scenario "Successful User Login"
    {
        @component(AuthService::login)
        Given "a registered user with valid credentials"
        When "the user submits login credentials"
        Then "the system issues an authentication token"
    }
}

Component AuthService
{
    !Note [ Microservice handling identity verification ]

    Invariants
    {
        "Passwords MUST be hashed using bcrypt before storage"
    }

    Model AuthToken
    {
        Type "Struct"
        Members
        {
            "token": "String"
            "expires_at": "Timestamp"
        }
    }

    Contract login
    {
        Signature "login(credentials: UserCredentials) -> AuthToken"
        Requires "Credentials must not be empty"
        Ensures "Valid token returned on success"
        Process
        {
            1. "Validate user input format"
            2. "Query database for matching user record"
            3. "Verify password hash"
        }
    }
}
```

### 2. Fabric Blueprint (`system.fabric`)
```fabric
system "CoreArchitecture"

group "IdentityCluster"
{
    Feature.Authentication
    Component.AuthService
    Storage.AuthDatabase
}

Feature.Authentication -> Component.AuthService : "Submits Credentials"
Component.AuthService -> Storage.AuthDatabase : "Queries User Hash"
```