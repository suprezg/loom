# Loom Language Grammar Reference

This document provides a comprehensive specification of the Loom grammar rules for `.thread` and `.fabric` files based on the Pest PEG grammar definitions.

---

## Table of Contents
1. [Lexical Tokens and Primitives](#1-lexical-tokens-and-primitives)
2. [Universal Elements (Notes, Diagrams, Decorators)](#2-universal-elements)
3. [Thread Entities (`.thread`)](#3-thread-entities-thread)
   - [Feature Entity](#feature-entity)
   - [Component Entity](#component-entity)
   - [Protocol Entity](#protocol-entity)
   - [Storage Entity](#storage-entity)
4. [Fabric Blueprints (`.fabric`)](#4-fabric-blueprints-fabric)
   - [System Declaration](#system-declaration)
   - [Canvas Cluster Groups](#canvas-cluster-groups)
   - [Topology Connections](#topology-connections)

---

## 1. Lexical Tokens and Primitives

- **Whitespace**: Space (` `), Tab (`\t`), Carriage Return (`\r`), Line Feed (`\n`).
- **Comments**:
  - Line comments: `# Single line comment`
  - Block comments: `/* Multi-line block comment */`
- **Identifiers (`ident`)**:
  - Rule: `(ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_" | "-")*`
  - Must start with a letter (`a-z`, `A-Z`) or underscore (`_`).
  - Cannot start with a number or hyphen.
- **Scoped Paths (`scoped_path`)**:
  - Rule: `ident ~ ("::" ~ ident)*`
  - Examples: `AuthService`, `AuthService::login`, `AppStorage::users`
- **String Literals (`string_lit`)**:
  - Double-quoted strings with support for escape characters: `"Hello \"World\""`

---

## 2. Universal Elements

### Note Blocks
Freeform markdown documentation attached at the entity level:
```loom
!Note [[
    Detailed architectural note explaining context,
    security boundaries, or requirements.
]]
```

### Diagram Blocks
Embedded Mermaid diagrams attached at the entity level:
```loom
!Diagram DiagramIdentifier
[[
    ```mermaid
    flowchart TD
        A[Start] --> B[Process]
        B --> C[End]
    ```
]]
```

### Universal Decorators
Used to cross-reference entities, contracts, models, tables, channels, and diagrams:
- `@feature(FeatureName::ScenarioName)` or `@feature(FeatureName)`
- `@component(ComponentName::ContractName)` or `@component(ComponentName::ModelName)`
- `@storage(StorageName::TableName)` or `@storage(StorageName)`
- `@protocol(ProtocolName::ChannelName)` or `@protocol(ProtocolName)`
- `@diagram(DiagramIdentifier)`

---

## 3. Thread Entities (`.thread`)

A `.thread` file contains one or more top-level entity declarations: `Feature`, `Component`, `Protocol`, or `Storage`.

### Feature Entity
Captures user behaviors, BDD scenarios, and acceptance criteria.
```loom
Feature FeatureName
{
    !Note [[ Optional markdown notes ]]
    !Diagram OptionalDiagram [[ ```mermaid ... ``` ]]

    Background
    {
        Given "initial precondition statement"
        And "additional context"
    }

    Rule "Business Rule Title"
    {
        @component(ComponentName::ContractName)
        @diagram(OptionalDiagram)
        Scenario ScenarioName
        {
            Title "Human readable scenario title"
            Given "a specific system state"
            When "an action occurs"
            Then "an expected outcome is observed"
        }

        @component(ComponentName::ContractName)
        Scenario Outline ScenarioOutlineName
        {
            Title "Parameterized test scenarios"
            Given "user <username>"
            When "submitting <input>"
            Then "result is <status>"

            Examples
            {
                | "username" | "input" | "status"  |
                | "alice"    | "valid" | "success" |
                | "bob"      | "bad"   | "failure" |
            }
        }
    }
}
```

### Component Entity
Defines internal business logic, data models, invariant boundaries, and formal behavioral contracts.
```loom
Component ComponentName
{
    !Note [[ Component purpose and constraints ]]
    !Diagram ComponentDiagram [[ ```mermaid ... ``` ]]

    Invariants
    {
        "SecurityBoundary: Description of mandatory invariant."
        "StateIntegrity: All mutations must satisfy invariant rules."
    }

    Model ModelName
    {
        Type "Enum" # or "Struct" or "Primitive"
        Members
        {
            "VariantOne"
            "VariantTwo"
        }
    }

    @storage(StorageName::TableName)
    Model StructModelName
    {
        Type "Struct"
        Members
        {
            "id": "UUID"
            "username": "String (min_length=3, max_length=64)"
        }
    }

    @storage(StorageName::TableName)
    @protocol(ProtocolName::ChannelName)
    @diagram(ComponentDiagram)
    Contract ContractName
    {
        Signature "methodName(arg: &ArgType) -> ReturnType"
        Requires "Pre-execution requirement in system state."
        Ensures "Post-execution guarantee."

        Precondition
        {
            "Argument validation condition."
        }

        Postcondition
        {
            "State modification guarantee."
        }

        Process
        {
            1. "Sequential step one description."
            2. "Sequential step two description."
            3. "Sequential step three description."
        }

        Errors
        {
            "ErrorCodeOne"
            "ErrorCodeTwo"
        }
    }
}
```

### Protocol Entity
Specifies asynchronous channels, streaming events, IPC boundaries, and messaging topologies.
```loom
Protocol ProtocolName
{
    !Note [[ Communication protocol description ]]
    !Diagram ProtocolDiagram [[ ```mermaid ... ``` ]]

    @feature(FeatureName::ScenarioName)
    @component(ComponentName::ContractName)
    @component(ComponentName::ModelName)
    @storage(StorageName::TableName)
    @diagram(ProtocolDiagram)
    Channel ChannelName
    {
        Pattern "AsyncBroadcast" # or "PointToPoint", "PubSub", "RequestResponse"
        Transport "tokio::sync::broadcast (Capacity: 1024)"
        Sender "ComponentName::ContractName"
        Receiver "TargetComponent::ContractName"
        Payload "ComponentName::ModelName"

        Errors
        {
            "ChannelFull"
            "ChannelClosed"
        }
    }
}
```

### Storage Entity
Defines persistence schemas, database engines, table fields, indexes, and relational cardinality.
```loom
Storage StorageName
{
    !Note [[ Database purpose and storage characteristics ]]
    !Diagram SchemaDiagram [[ ```mermaid ... ``` ]]

    Engine "SQLite3 / SQLCipher" # or "PostgreSQL", "Redis", etc.

    @feature(FeatureName::ScenarioName)
    @component(ComponentName::ContractName)
    @diagram(SchemaDiagram)
    Table TableOne
    {
        Fields
        {
            "id": "UUID [PrimaryKey, NonNull]"
            "name": "String(128) [NonNull, Unique]"
            "created_at": "Date [NonNull, Default(NOW)]"
        }

        Indexes
        {
            idx_table_name (name)
        }
    }

    @feature(FeatureName::ScenarioName)
    @component(ComponentName::ContractName)
    Table TableTwo
    {
        Fields
        {
            "id": "UUID [PrimaryKey, NonNull]"
            "table_one_id": "UUID [NonNull, ForeignKey(TableOne.id)]"
            "detail": "String(255) [NonNull]"
        }

        Indexes
        {
            idx_t2_t1_id (table_one_id)
        }

        Relations
        {
            TableOne.id 1:N TableTwo.table_one_id
        }
    }
}
```

---

## 4. Fabric Blueprints (`.fabric`)

A `.fabric` blueprint defines macro system architecture, canvas visual grouping, and component connection topology.

### Structure
```fabric
system "System Architecture Name"

# 1. Canvas Cluster Groups
group "Cluster Group Label"
{
    Feature.FeatureName
    Component.ComponentName
    Storage.StorageName::TableName
    Protocol.ProtocolName::ChannelName
}

# 2. Topology Connections
Feature.FeatureName -> Component.ComponentName::ContractName : "Edge label description"
Component.ComponentName::ContractName -> Storage.StorageName::TableName : "Reads / Writes table"
Component.ComponentName::ContractName -> Protocol.ProtocolName::ChannelName : "Emits event payload"
Protocol.ProtocolName::ChannelName -> Storage.StorageName::TableName : "Persists stream"
```

### Rules:
1. `system` declaration must appear once at the top of the file.
2. Selectors use `EntityKind.ScopedPath` where `EntityKind` is one of `Feature`, `Component`, `Storage`, `Protocol`.
3. Arrow connections use `->` with an optional string label `: "Label"`.
