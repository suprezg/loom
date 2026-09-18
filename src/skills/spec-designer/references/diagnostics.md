# Loom Domain Diagnostics & Error Reference

Loom performs a two-tier compilation and validation pipeline:
- **Tier 1 (Syntactic)**: PEG Grammar parsing validation (`LM0001`-`LM0003` for Thread, `LM1001`-`LM1003` for Fabric).
- **Tier 2 (Semantic)**: Cross-entity symbol discovery, reference resolution, relational constraints, and unused symbol linting (`LM2001`-`LM2007`, `LM3001`).

---

## Diagnostic Quick Index

| Code | Severity | Tier | Description |
|---|---|---|---|
| `LM0001` | Error | Syntactic | Illegal identifier in `.thread` file |
| `LM0002` | Error | Syntactic | Unclosed block or unclosed string literal in `.thread` file |
| `LM0003` | Error | Syntactic | Malformed scoped path in `.thread` file |
| `LM1001` | Error | Syntactic | Illegal identifier in `.fabric` file |
| `LM1002` | Error | Syntactic | Unclosed block or string in `.fabric` file |
| `LM1003` | Error | Syntactic | Malformed path or entity selector in `.fabric` file |
| `LM2001` | Error | Semantic | Unresolved cross-entity decorator or symbol reference in `.thread` file |
| `LM2002` | Error | Semantic | Duplicate entity or member declaration in `.thread` file |
| `LM2003` | Error | Semantic | Storage index referencing non-existent column |
| `LM2004` | Error | Semantic | Storage relation referencing invalid or non-existent table/column target |
| `LM2005` | Error | Semantic | Protocol channel Sender, Receiver, or Payload referencing non-existent symbol |
| `LM2006` | Warning | Semantic | Scenario missing linking decorator (`@component`, `@storage`, `@protocol`, etc.) |
| `LM2007` | Warning | Semantic | Unused entity, member, or diagram declared but never referenced |
| `LM3001` | Error | Semantic | Fabric blueprint connection or group referencing non-existent entity/member |

---

## Detailed Diagnostics Guide

### LM0001: Thread Illegal Identifier
- **Cause**: An identifier starts with a number, hyphen, or invalid special character instead of `[a-zA-Z_]`.
- **Red Flag**:
  ```loom
  # RED FLAG: Identifier starts with digit
  Feature 123Auth
  {
  }
  ```
- **Green Flag**:
  ```loom
  # GREEN FLAG: Valid identifier
  Feature Auth123
  {
  }
  ```

---

### LM0002: Thread Unclosed Block
- **Cause**: Missing closing brace `}`, square bracket `]]`, or unmatched quote `"`.
- **Red Flag**:
  ```loom
  # RED FLAG: Missing closing brace for Feature
  Feature Auth {
      Rule "Login" {
          Scenario S1 {
              Given "active user"
          }
      }
  # EOF without closing Auth brace
  ```
- **Green Flag**:
  ```loom
  Feature Auth {
      Rule "Login" {
          Scenario S1 {
              Given "active user"
          }
      }
  }
  ```

---

### LM0003: Thread Malformed Path
- **Cause**: A scoped path contains dangling colons (e.g. `AuthService::`), leading colons (`::login`), or triple colons (`AuthService:::login`).
- **Red Flag**:
  ```loom
  # RED FLAG: Dangling colons
  @component(AuthService::)
  Scenario S1 { ... }
  ```
- **Green Flag**:
  ```loom
  # GREEN FLAG: Full scoped path or single identifier
  @component(AuthService::login)
  Scenario S1 { ... }
  ```

---

### LM1001: Fabric Illegal Identifier
- **Cause**: An identifier in `.fabric` starts with a digit or illegal character.
- **Red Flag**:
  ```fabric
  # RED FLAG: Invalid selector identifier
  Component.123Service -> Storage.DB
  ```
- **Green Flag**:
  ```fabric
  Component.AuthService -> Storage.DB
  ```

---

### LM1002: Fabric Unclosed Block
- **Cause**: A group block is opened with `{` but lacks the closing `}`.
- **Red Flag**:
  ```fabric
  group "Core" {
      Component.AuthService
  ```
- **Green Flag**:
  ```fabric
  group "Core" {
      Component.AuthService
  }
  ```

---

### LM1003: Fabric Malformed Path
- **Cause**: Dangling colons or invalid entity kind in `.fabric`.
- **Red Flag**:
  ```fabric
  Component.AuthService:: -> Storage.AppStorage::
  ```
- **Green Flag**:
  ```fabric
  Component.AuthService::login -> Storage.AppStorage::users
  ```

---

### LM2001: Thread Unresolved Reference
- **Cause**: A decorator (`@component`, `@storage`, `@protocol`, `@feature`, `@diagram`) points to a target that is not declared in any loaded `.thread` file.
- **Red Flag**:
  ```loom
  # RED FLAG: NonExistentService does not exist in any loaded file
  @component(NonExistentService::process)
  Scenario Login { ... }
  ```
- **Green Flag**:
  ```loom
  # Ensure NonExistentService (and member process) is declared in a Component block
  @component(AuthService::login)
  Scenario Login { ... }
  ```

---

### LM2002: Thread Duplicate Declaration
- **Cause**: Two entities have the exact same name, or two members (Contracts, Models, Tables, Channels) have the same name within the same entity.
- **Red Flag**:
  ```loom
  # RED FLAG: Duplicate Feature entity name
  Feature Authentication { ... }
  Feature Authentication { ... }
  ```
- **Green Flag**:
  ```loom
  Feature Authentication { ... }
  Feature Authorization { ... }
  ```

---

### LM2003: Storage Index Column Error
- **Cause**: An index in `Indexes { idx_name (col) }` references a column name `col` that is not declared in the `Fields { ... }` block of that table.
- **Red Flag**:
  ```loom
  Table users {
      Fields {
          "id": "UUID [PrimaryKey]"
          "username": "String(64)"
      }
      Indexes {
          # RED FLAG: email column is not in Fields
          idx_user_email (email)
      }
  }
  ```
- **Green Flag**:
  ```loom
  Table users {
      Fields {
          "id": "UUID [PrimaryKey]"
          "username": "String(64)"
          "email": "String(255)"
      }
      Indexes {
          idx_user_email (email)
      }
  }
  ```

---

### LM2004: Storage Relation Target Error
- **Cause**: A relational definition `TableA.col 1:N TableB.col` references a table or column that does not exist in the storage schema.
- **Red Flag**:
  ```loom
  Table audit_logs {
      Fields {
          "id": "UUID [PrimaryKey]"
          "user_id": "UUID"
      }
      Relations {
          # RED FLAG: NonExistentTable or bad_id does not exist
          NonExistentTable.id 1:N audit_logs.user_id
      }
  }
  ```
- **Green Flag**:
  ```loom
  Table audit_logs {
      Fields {
          "id": "UUID [PrimaryKey]"
          "user_id": "UUID"
      }
      Relations {
          users.id 1:N audit_logs.user_id
      }
  }
  ```

---

### LM2005: Protocol Channel Target Error
- **Cause**: A Channel's `Sender`, `Receiver`, or `Payload` declaration references a non-existent entity, contract, or model.
- **Red Flag**:
  ```loom
  Channel AuditStream {
      Pattern "PointToPoint"
      Transport "IPC Queue"
      # RED FLAG: FakeService is not defined
      Sender "FakeService::sendAudit"
      Receiver "AppStorage::audit_logs"
      Payload "AuthService::AuditEvent"
  }
  ```
- **Green Flag**:
  ```loom
  Channel AuditStream {
      Pattern "PointToPoint"
      Transport "IPC Queue"
      Sender "AuthService::login"
      Receiver "AppStorage::audit_logs"
      Payload "AuthService::AuditEvent"
  }
  ```

---

### LM2006: Missing Scenario Decorator Warning
- **Cause**: A `Scenario` or `Scenario Outline` inside a `Feature` is declared without any `@component(...)`, `@storage(...)`, `@protocol(...)`, or `@diagram(...)` decorator linking it to system contracts or architecture.
- **Fix**: Attach at least one relevant `@component(Service::contract)` or `@storage(...)` decorator to connect the scenario to behavioral implementation.

---

### LM2007: Unused Symbol Warning
- **Cause**: An entity, table, model, channel, or diagram is declared in a `.thread` file, but never referenced anywhere across decorators, scenarios, relations, or fabric blueprints.
- **Fix**: Either reference the symbol in a scenario/channel/decorator or clean up the unused declaration.

---

### LM3001: Fabric Unresolved Reference
- **Cause**: A `.fabric` blueprint connection statement (`A -> B`) or cluster group references an entity or member that is not present in any loaded `.thread` specification.
- **Red Flag**:
  ```fabric
  Feature.MissingFeature -> Component.AuthService::login
  ```
- **Green Flag**:
  ```fabric
  Feature.Authentication -> Component.AuthService::login
  ```
