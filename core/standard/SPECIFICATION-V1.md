# LOOM SPECIFICATION STANDARD (LSS) — VERSION 1 (V1)

```text
Document Identifier : LSS-V1-2026
Standard Version    : 1.0 (V1)
Maturity Level      : Official Technical Specification
Author              : Loom Architecture Working Group
License             : MIT License (https://opensource.org/licenses/MIT)
```

## 1. Scope

### 1.1 Purpose
<a id="sec-1-1"></a>
The Loom Specification Standard (LSS-V1) defines the formal grammar, structure, data typing, and behavioral verification rules for authoring software specifications in GitHub-Flavored Markdown (.spec.md).

Specifications written under LSS-V1 are simultaneously:
1. Human-Readable: Clearly read and rendered in any markdown viewer, text editor, or Git web interface.
2. Machine-Parseable: Compiled deterministically by the Loom Compiler (loom) into a typed Abstract Syntax Tree (AST) for code generation, migrations, and test harnesses.

### 1.2 Conformance
<a id="sec-1-2"></a>
The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, MAY, and OPTIONAL in this document are interpreted as described in IETF RFC 2119.

- MUST / REQUIRED: Absolute requirement. Compilers MUST reject non-conforming documents.
- MUST NOT: Absolute prohibition.
- SHOULD / RECOMMENDED: Strong recommendation with valid exceptions only upon careful deliberation.
- MAY / OPTIONAL: Permitted optional construct.

## 2. Philosophy

### 2.1 Construction
<a id="sec-2-1"></a>
LSS-V1 builds software from features outward by defining seven unified layers:
1. Intent: The purpose, user story, and non-negotiable rules.
2. Behaviors: Operational paths defined through Given/When/Then scenarios.
3. Tests: Concrete verification cases with explicit inputs and expected outputs.
4. Models: Data structures, algebraic data types (ALDT), and abstract data types (ABDT).
5. Contracts: Procedure signatures, parameters, preconditions, postconditions, processes, and outcomes.
6. Storage: Relational database tables, nullability, primary keys, foreign keys, and indexes.
7. Protocols: Synchronous endpoints and asynchronous communication events.

### 2.2 Goals
<a id="sec-2-2"></a>
High-level terms like Authentication or Billing are Goals, not features.
- Goal: A business milestone or bounded context (e.g., authentication, billing).
- Feature: Any discrete capability that creates tangible value towards attaining a goal.
- Scope: Both user-facing capabilities and internal mechanisms (such as token issuers, password validators, and storage adapters) are features.

### 2.3 Truths
<a id="sec-2-3"></a>
Specifications MUST capture domain truths rather than programming language idioms:
- State Shapes: What data exists and its member constraints.
- Behavioral Logic: Why the feature exists, who triggers it, and how it behaves.
- Verifiable Outputs: Deterministic outcomes from specific inputs.
- Persistent Schemas: Physical table columns, keys, and indexes.
- Interface Contracts: Messages and payload structures across components.

### 2.4 Paradigms
<a id="sec-2-4"></a>
LSS-V1 is strictly paradigm-agnostic. A specification can be compiled to:
- Object-Oriented Programming (OOP): Models become classes, contracts become methods, storage maps to ORMs.
- Functional Programming (FP): Models become immutable records, contracts become pure functions.
- Data-Oriented Design (DOD): Models become flat structs, contracts become batch pipelines, storage maps to tables.
- Actor and Event Systems: Contracts become message handlers, protocols become event streams.

Language-specific pointers, inheritance hierarchies, and framework annotations MUST NOT appear in declarations.

## 3. Structure

### 3.1 Scoping
<a id="sec-3-1"></a>
Features are categorized into two structural patterns:
- Ungrouped Features: Standalone, autonomous capabilities with no shared domain models (e.g., specs/url-slug-generator.spec.md).
- Grouped Features: Cohesive sub-features collaborating under a larger goal (e.g., specs/authentication/).

### 3.2 Naming
<a id="sec-3-2"></a>
File and folder naming follows strict rules of identity:
- For Ungrouped Features: The goal IS the file name (e.g., specs/health-check.spec.md).
- For Grouped Features: The goal IS the folder name (e.g., specs/authentication/), and files inside declare what they do (e.g., session-token-generator.spec.md).

### 3.3 Namespaces
<a id="sec-3-6"></a>
Every specification declares a unique Rust-style namespace in its YAML frontmatter:
- For Autonomous (Ungrouped) Features: A single PascalCase word derived from the feature file name (e.g., `namespace: SlugGenerator`).
- For Grouped Features: Exactly two PascalCase words separated by `::` representing `Folder::File` (e.g., `namespace: Auth::Session`).
- For Global Commons: Exactly two PascalCase words starting with Commons, formatted as `Commons::File` (e.g., `namespace: Commons::Errors`).

### 3.4 Groups
<a id="sec-3-3"></a>
Group folders use a flat structure. Nested features/ subdirectories are PROHIBITED.

Shared group foundations MUST use the 0- prefix to establish compiler precedence:
- `0-models.spec.md`: Shared group models, ALDTs, and ABDTs.
- `0-contracts.spec.md`: Shared group contracts, policies, and reusable procedures.
- `0-storage.spec.md`: Shared group database tables and indexes.
- `0-protocols.spec.md`: Shared group communication envelopes and channels.

### 3.5 Commons
<a id="sec-3-5"></a>
Universal system-wide contracts and models reside in specs/commons/.
- No 0- Prefix: Files in specs/commons/ MUST NOT use the 0- prefix (e.g., errors.spec.md, formatting.spec.md, pagination.spec.md). The 0- prefix is reserved exclusively for group-level shared files.
- Global Access: Both ungrouped and grouped features MAY link to files in specs/commons/.

```text
specs/
├── commons/                                 # Global Commons (No 0- Prefix, namespace: Commons::<File>)
│   ├── errors.spec.md
│   └── formatting.spec.md
│
├── url-slug-generator.spec.md               # Ungrouped Feature (namespace: <File>)
│
└── authentication/                          # Grouped Features Folder (namespace: Auth::<File>)
    ├── 0-models.spec.md                     # Group Foundation (0- Prefix Required)
    ├── 0-contracts.spec.md                  # Group Foundation
    ├── 0-storage.spec.md                    # Group Foundation
    ├── 0-protocols.spec.md                  # Group Foundation
    ├── session-token-generator.spec.md      # Feature Slice
    └── credential-validator.spec.md         # Feature Slice
```

### 3.6 Boundaries
<a id="sec-3-4"></a>
To prevent architectural coupling, LSS-V1 enforces strict folder boundary rules:
- Intra-Group Linking: Sibling feature files inside the same folder MAY cross-reference each other and group 0- foundation files.
- Cross-Group Prohibition: Features in one group MUST NOT link to files in another group. Cross-group references cause fatal compiler errors in loom scout.
- Autonomous Isolation: Ungrouped features MUST NOT link to grouped features.

## 4. Markdown

### 4.1 Elements
<a id="sec-4-1"></a>
All specifications MUST adhere to strict Markdown formatting rules:
- Headings Limit: Exactly 3 levels permitted (#, ##, ###). Level 4 or deeper is PROHIBITED.
- Anchor Tags: An anchor tag `<a id="..."></a>` is REQUIRED directly below every Level 3 heading.
- Unordered Lists: MUST use the hyphen `-` character exclusively. Asterisks are PROHIBITED.
- Ordered Lists: MUST use decimal numbering: `1.`, `2.`, `3.`.
- No Bold or Emphasis: Bold and italic emphasis using asterisks or underscores are PROHIBITED in specification prose and headings.
- No Horizontal Rules: Divider lines (`---`) are PROHIBITED in the document body. Only YAML frontmatter delimiters may use dashes.
- Tables: Tables are permitted and required for ALDT, ABDT, and Storage definitions.
- Paragraphs: Standard explanatory paragraphs are permitted.
- Code Blocks: Inline code (wrapped in single backticks) and fenced code blocks are permitted.
- Mermaid Blocks: Fenced mermaid blocks are permitted for diagrams.
- Math: Inline math (`$equation$`) and math display blocks (`$$equation$$`) are permitted anywhere.
- Checklists: Task checkboxes (`- [ ]`, `- [x]`) are PROHIBITED.

### 4.2 Identifiers
<a id="sec-4-2"></a>
Declared names across all specifications MUST be valid Identifiers:
- MUST NOT start with a number.
- MUST be exactly one word with no whitespace.
- MUST consist only of ASCII letters (`a-z`, `A-Z`), digits (`0-9`), hyphens (`-`), and underscores (`_`).
- MUST be wrapped in inline code backticks (e.g., `user_id`, `TokenRequest`, `issue-tokens`).
- Applies to: Contract names, Model names, ALDT members, ABDT operations, Table fields.

### 4.3 Hyperlinks
<a id="sec-4-3"></a>
Cross-referencing and hyperlinking in Loom are strictly governed by Rust-style namespaces and explicit scope boundaries:

1. Qualified Symbols:
   Every referenced entity across specifications is addressed by its fully qualified namespace string:
   - Models: `Namespace::ModelName` (e.g., `Auth::Session::TokenRequest`).
   - Members: `Namespace::ModelName.membername` (e.g., `Auth::Session::TokenRequest.user_id`).
   - Contracts: `Namespace::ContractName` (e.g., `Auth::Session::generate-session-tokens`).
   - Tables: `Namespace::TableName` (e.g., `Auth::Session::user_sessions`).
   - Table Fields: `Namespace::TableName.fieldname` (e.g., `Auth::Session::user_sessions.id`).
   - Protocols: `Namespace::ProtocolName` (e.g., `Auth::Session::issue-tokens`).

2. Link Formatting:
   Links MUST use standard markdown reference format enclosing the full qualified namespace string:
   `[Namespace::Symbol](path)`
   - Internal Spec Link: `[Namespace::Symbol](#anchor-id)`
   - Sibling Group Link: `[Namespace::Symbol](./other-file.spec.md#anchor-id)`
   - Commons Link: `[Commons::Symbol](../commons/file.spec.md#anchor-id)`
   - Anchor Resolution for Members: Cross-links targeting specific model members or table fields resolve to the anchor ID of their enclosing parent model or table (e.g., `[Auth::Session::TokenRequest.user_id](#model-1)` and `[Auth::Session::user_sessions.id](#table-1)`).

3. Strictly Permitted Linking Locations:
   Cross-linking is strictly allowed ONLY in the following locations and NOWHERE else:
   - Section 3 (Tests): In `Input([Namespace::Symbol](path), ...):` and `Expect([Namespace::Symbol](path), ...):`.
   - Section 4 (Models): In ALDT `Type` column: `[Namespace::Model](path)` or primitive or collection.
   - Section 5 (Contracts): In Parameters `<param>([Namespace::Model](path) or primitive):`, Preconditions `Assumption([Namespace::Symbol](path)):`, Postconditions `Guarantee([Namespace::Symbol](path)):`, and Outcomes `Success([Namespace::Model](path)):`, `Error([Namespace::Symbol](path)):`.
   - Section 7 (Protocols): In `Sender([Namespace::Symbol](path)):`, `Receiver([Namespace::Symbol](path)):`, `Payload([Namespace::Model](path)):`, and `Errors([Namespace::Symbol](path)):`.

4. Strictly Prohibited Linking Locations:
   Cross-linking is PROHIBITED in the following locations:
   - Section 1 (Intent): Prohibited in prose, story, and rules.
   - Section 2 (Behaviors): Prohibited in scenario statements.
   - Section 4 (Models): Prohibited in Member column, Requirement column, and ABDT & ALDT descriptions.
   - Section 5 (Contracts): Prohibited in Process steps (`1.`, `2.`).
   - Section 6 (Storage): Prohibited in table columns, keys, and indexes.

5. Mandatory Non-Empty Invariant:
   Every designated parenthesized linking slot MUST contain a valid qualified cross-link or recognized primitive type. Leaving a slot blank or empty is a fatal compiler validation error. If a contract takes no parameters, or a protocol has no payload or produces no errors, authors MUST explicitly use the `unit` or `void` primitive types (e.g., `request(unit):`, `Payload(unit):`, or `Errors(void):`).

### 4.4 Additions
<a id="sec-4-4"></a>
Specifications are precise architecture documents. To provide context without corrupting machine parseability:
- Explanatory Paragraphs: Freely placeable before or after any canonical block.
- Mermaid Diagrams: Freely placeable to illustrate workflows, sequence calls, state machines, or ER diagrams.
- Math Blocks: Freely placeable to illustrate formal calculations, cryptographic formulas, and algorithms.

## 5. Types

### 5.1 Primitives
<a id="sec-5-1"></a>
LSS-V1 defines canonical primitive data types:
- `string`: UTF-8 textual string.
- `integer`: Signed 64-bit integer.
- `float`: 64-bit IEEE 754 floating-point number.
- `boolean`: Logical true or false.
- `timestamp`: UTC ISO 8601 extended timestamp.
- `uuid`: Canonical 128-bit UUID format.
- `any`: Dynamic or unconstrained type representing arbitrary structured or scalar values.
- `unit`: Singleton type containing exactly one value, used to indicate parameterless inputs, empty payloads, or unit variants.
- `void`: Empty type denoting the complete absence of value, uninstantiable outcomes, or infallible error slots.

### 5.2 Collections
<a id="sec-5-2"></a>
LSS-V1 natively supports canonical composite and collection types:
- `Array`: Fixed-size sequence of elements of type `T` (e.g., `[T; N]` or `Array<T, N>`).
- `List`: Variable-length ordered sequence of elements of type `T` (e.g., `List<T>` or `[T]`).
- `Tuple`: Fixed-size heterogeneous product of types (e.g., `(T1, T2)`).
- `Dictionary`: Key-value associative mapping from key type `K` to value type `V` (e.g., `Dictionary<K, V>`).

### 5.3 Storage
<a id="sec-5-3"></a>
Database column declarations MUST use standard relational types:
- `varchar(n)`: Variable-length character string with explicit maximum length `n`.
- `text`: Arbitrary length textual data.
- `integer`: Standard 32-bit signed integer.
- `bigint`: 64-bit signed integer for large ranges or high-volume sequences.
- `timestamp`: UTC timestamp with timezone.
- `boolean`: Relational boolean flag.
- `uuid`: 128-bit universally unique identifier.
- `decimal(p, s)`: Fixed-point exact numeric with precision `p` and scale `s`.

### 5.4 Domain
<a id="sec-5-4"></a>
Complex domain types MUST be explicitly declared in Section 4 (Models) as ALDT or ABDT structures before they can be referenced in Contracts, Tests, or Protocols.

## 6. Frontmatter

### 6.1 Schema
<a id="sec-6-1"></a>
Every `.spec.md` file MUST open with a YAML frontmatter block enclosed between `---` delimiters:

```yaml
---
title: Session Token Generator
namespace: Auth::Session
version: 1.0
modified: 24-10-2026
status: active
phase: approved
targets:
  python:
    service_file: "src/identity/services/session_token_generator.py"
    test_file: "tests/identity/test_session_token_generator.py"
  typescript:
    service_file: "src/identity/services/sessionTokenGenerator.ts"
    test_file: "tests/identity/sessionTokenGenerator.test.ts"
---
```

### 6.2 Fields
<a id="sec-6-2"></a>
The frontmatter properties are strictly validated:

| Property | Type | Status | Description |
|---|---|---|---|
| `title` | `String` | MANDATORY | Human-readable specification title. |
| `namespace` | `String` | MANDATORY | Rust-like namespace: `Word` or `Folder::File`. |
| `version` | `String` | MANDATORY | Two-part version string (`1.0`, `1.1`). |
| `modified` | `String` | MANDATORY | Modification date strictly in `dd-mm-yyyy` format. |
| `status` | `String` | MANDATORY | Exactly two permitted values: `active` or `inactive`. |
| `phase` | `String` | MANDATORY | Exactly three permitted values: `draft`, `review`, or `approved`. |
| `targets` | `Object` | OPTIONAL | File paths for downstream code generation plugins. |

### 6.3 Versioning
<a id="sec-6-3"></a>
Specifications follow two-part numeric versioning (`Major.Minor`):
- Major increment (`X.0`): Breaking behavioral changes, removing models or fields, changing contract signatures.
- Minor increment (`X.Y`): Backwards-compatible changes, adding optional fields, new test cases, documentation updates.
- Date component: Stored in `modified` as `dd-mm-yyyy`. Time components and ISO timestamps are PROHIBITED.

## 7. Specification

### 7.1 Sections
<a id="sec-7-1"></a>
Every specification MUST open with the Level 1 title followed sequentially by canonical Level 2 headings.

Syntax for document root structure:
```markdown
# <Title>

## 1. Intent
## 2. Behaviors
## 3. Tests
## 4. Models
## 5. Contracts
## 6. Storage
## 7. Protocols
```

### 7.2 Intent
<a id="sec-7-2"></a>
The Intent section establishes why the capability exists. Cross-linking is PROHIBITED in this section.

Syntax for Intent section:
```markdown
## 1. Intent

<Introductory paragraph: Explain what it is, what it tends to achieve, and how it achieves it.>

### Story
<a id="story"></a>
As an <actor>
I want to <action>
So that <outcome>

### Rules
<a id="rules"></a>
- <Rule statement in active voice using MUST, MUST NOT, or SHOULD>
- <Additional non-negotiable architectural or domain rule statement>
```

### 7.3 Behaviors
<a id="sec-7-3"></a>
The Behaviors section defines interaction workflows using scenario headings. Cross-linking is PROHIBITED in this section.

Syntax for Behaviors section:
```markdown
## 2. Behaviors

### Scenario <number>: <short 4-6 word description>
<a id="scenario-<number>"></a>
- Given <initial context or precondition state>
- When <triggering action or event occurs>
- Then <expected observable state change or response>
- And <additional consecutive outcome or side effect>
- But <counter-condition or exception handling path>
```

### 7.4 Tests
<a id="sec-7-4"></a>
The Tests section defines concrete verification cases with mandatory qualified cross-links.

Syntax for Tests section:
```markdown
## 3. Tests

### Case <number>: <short 4-6 word description>
<a id="case-<number>"></a>
- Input([<Namespace::Symbol>](<path-or-anchor>), [<Namespace::Symbol>](<path-or-anchor>)): <Parameters and input condition sentence>
- Expect([<Namespace::Symbol>](<path-or-anchor>), [<Namespace::Symbol>](<path-or-anchor>)): <Expected return payload and state assertion sentence>
```

Parentheses following `Input` and `Expect` MUST contain one or more comma-separated qualified cross-links. Slots MUST NOT be left empty.

### 7.5 Models
<a id="sec-7-5"></a>
The Models section declares data structures using Level 3 headings. The model name MUST be an inline code identifier wrapped in backticks after the colon.

1. Algebraic Data Type (`ALDT`):
   Table with four columns: `Member | Type | Requirement | Description`.
   - `Member`: Field name or variant tag as an inline code identifier without links.
   - `Type`: Recognized primitive type, collection type, or qualified cross-link `[<Namespace::Model>](<path-or-anchor>)`.
   - `Requirement`: Exactly `required` or `optional` when defining a struct (product type), or `variant` when defining an enum (sum type).
   - `Description`: Domain meaning and business constraints.

2. Abstract Data Type (`ABDT`):
   Table with two columns: `Operation | Description`.
   - `Operation`: Operational capability name as an inline code identifier without links.
   - `Description`: Operational signature, input parameters, behavioral semantics, and invariants.

Syntax for Models section:
```markdown
## 4. Models

### ALDT: `<struct_identifier>`
<a id="model-<number>"></a>

| Member | Type | Requirement | Description |
|---|---|---|---|
| `<field_name>` | `<primitive-or-collection-type>` or [<Namespace::Model>](<path-or-anchor>) | required | <Field description sentence> |
| `<field_name>` | `<primitive-or-collection-type>` or [<Namespace::Model>](<path-or-anchor>) | optional | <Field description sentence> |

### ALDT: `<enum_identifier>`
<a id="model-<number>"></a>

| Member | Type | Requirement | Description |
|---|---|---|---|
| `<variant_name>` | `<primitive-or-collection-type>` or [<Namespace::Model>](<path-or-anchor>) | variant | <Variant description sentence> |
| `<variant_name>` | `<primitive-or-collection-type>` or [<Namespace::Model>](<path-or-anchor>) | variant | <Variant description sentence> |

### ABDT: `<abstract_type_identifier>`
<a id="model-<number>"></a>

| Operation | Description |
|---|---|
| `<operation_name>` | <Active-voice operational contract, parameter semantics, and mutation behavior> |
```

### 7.6 Contracts
<a id="sec-7-6"></a>
The Contracts section defines procedures, methods, and routines. The contract name MUST be an inline code identifier wrapped in backticks after the colon.

Syntax for Contracts section:
````markdown
## 5. Contracts

### Contract: `<contract_identifier>`
<a id="contract-<number>"></a>

```text
<contract-name>(<param_name>: <ParamType>) -> <ReturnType>
```

Parameters:
- `<param_name>`([<Namespace::Model>](<path-or-anchor>) or <primitive-type>): <Parameter description sentence>

Preconditions:
- Assumption([<Namespace::Symbol>](<path-or-anchor>)): <Precondition assertion sentence>

Postconditions:
- Guarantee([<Namespace::Symbol>](<path-or-anchor>)): <Postcondition outcome assertion sentence>

Process:
1. <Procedural execution step sentence without cross-links>
2. <Next procedural execution step sentence without cross-links>

Outcomes:
- Success([<Namespace::Model>](<path-or-anchor>)): <Success outcome description sentence>
- Error([<Namespace::Symbol>](<path-or-anchor>)): <Error outcome description sentence>
````

All parenthesized slots in Parameters, Assumptions, Guarantees, and Outcomes MUST contain qualified cross-links or valid types and MUST NOT be left empty. Parameterless contracts MUST use `unit` (e.g., `- request(unit): Operational trigger requiring no payload.`). Cross-linking in Process steps is PROHIBITED.

### 7.7 Storage
<a id="sec-7-7"></a>
The Storage section defines persistent relational tables. The table name MUST be an inline code identifier wrapped in backticks after the colon.

Syntax for Storage section:
```markdown
## 6. Storage

### Table: `<table_identifier>`
<a id="table-<number>"></a>

| Field | Type | Nullable | Description |
|---|---|---|---|
| `<field_name>` | `<storage-type>` | NOT NULL | <Column description sentence> |
| `<field_name>` | `<storage-type>` | NULL | <Column description sentence> |

- Primary Key: `<field_name>`
- Foreign Key: `<field_name>` references `<parent_table>`(`<parent_field>`)
- Index: `<field_name>`
```

Rules for Storage declarations:
- `- Primary Key:` is MANDATORY for every declared table. Composite primary keys are specified as comma-separated field lists (e.g., `- Primary Key: <field1>, <field2>`).
- `- Foreign Key:` is OPTIONAL. Omit the bullet entirely if the table has no relational foreign keys.
- `- Index:` is OPTIONAL. Omit the bullet entirely if no secondary indexes are declared. Composite indexes are specified as comma-separated field lists.
- Cross-linking is PROHIBITED in Section 6. State mutations sections are PROHIBITED.

### 7.8 Protocols
<a id="sec-7-8"></a>
The Protocols section defines synchronous interfaces and asynchronous event communications. The protocol name MUST be an inline code identifier wrapped in backticks after the colon.

Syntax for Protocols section:
```markdown
## 7. Protocols

### Protocol: `<protocol_identifier>`
<a id="protocol-<number>"></a>
- Communication: <Type (REST, gRPC, IPC, ITC) followed by method and path or socket/channel identifier>
- Sender([<Namespace::Symbol>](<path-or-anchor>)): <Description of sending entity or service>
- Receiver([<Namespace::Symbol>](<path-or-anchor>)): <Description of receiving contract or component>
- Payload([<Namespace::Model>](<path-or-anchor>) or <primitive-type>): <Description of transmitted payload structure>
- Errors([<Namespace::Symbol>](<path-or-anchor>) or <primitive-type>): <Description of fault conditions and error payloads>
- Description: <Active-voice summary of the communication interaction>
```

All parenthesized slots (`Sender`, `Receiver`, `Payload`, `Errors`) MUST contain a valid cross-link or primitive type and MUST NOT be left empty. Bodyless protocol messages MUST use `unit` for Payload. Infallible event broadcasts MUST use `void` for Errors.

### 7.9 Anchors
<a id="sec-7-9"></a>
Standardized anchor IDs and cross-referencing syntax:

| Entity Type | Level 3 Heading Standard | Anchor ID Syntax | Local Reference Syntax | Group Reference Syntax | Commons Reference Syntax |
|---|---|---|---|---|---|
| Story | `### Story` | `story` | `[Namespace::story](#story)` | `[Namespace::story](./other.spec.md#story)` | N/A |
| Rules | `### Rules` | `rules` | `[Namespace::rules](#rules)` | `[Namespace::rules](./other.spec.md#rules)` | N/A |
| Behavior | `### Scenario <n>: <name>` | `scenario-<number>` | `[Namespace::Scenario1](#scenario-1)` | `[Namespace::Scenario1](./other.spec.md#scenario-1)` | N/A |
| Test Case | `### Case <n>: <name>` | `case-<number>` | `[Namespace::Case1](#case-1)` | `[Namespace::Case1](./other.spec.md#case-1)` | N/A |
| Model (ALDT) | `### ALDT: `<name>`` | `model-<number>` | `[Namespace::Model](#model-1)` | `[Namespace::Model](./0-models.spec.md#model-1)` | `[Commons::Model](../commons/models.spec.md#model-1)` |
| Model (ABDT) | `### ABDT: `<name>`` | `model-<number>` | `[Namespace::Model](#model-1)` | `[Namespace::Model](./0-models.spec.md#model-1)` | `[Commons::Model](../commons/models.spec.md#model-1)` |
| Contract | `### Contract: `<name>`` | `contract-<number>` | `[Namespace::Contract](#contract-1)` | `[Namespace::Contract](./0-contracts.spec.md#contract-1)` | `[Commons::Contract](../commons/contracts.spec.md#contract-1)` |
| Table | `### Table: `<name>`` | `table-<number>` | `[Namespace::Table](#table-1)` | `[Namespace::Table](./0-storage.spec.md#table-1)` | N/A |
| Protocol | `### Protocol: `<name>`` | `protocol-<number>` | `[Namespace::Protocol](#protocol-1)` | `[Namespace::Protocol](./0-protocols.spec.md#protocol-1)` | `[Commons::Protocol](../commons/protocols.spec.md#protocol-1)` |

## 8. Review

### 8.1 Guidelines
<a id="sec-8-1"></a>
The table below specifies architectural practices to follow and anti-patterns to avoid:

| Category | Prescribed Practice (Do) | Prohibited Anti-Pattern (Don't) |
|---|---|---|
| Headings | Restrict headings strictly to 3 levels (#, ##, ###). | Using Level 4 headings (####) or deeper. |
| Anchors | Include an anchor tag under every Level 3 heading. | Omitting anchor tags or using custom anchor names. |
| Identifiers | Wrap identifiers in backticks after colons in headings. | Omitting backticks on identifiers in Level 3 headings. |
| Typography | Use plain text and inline code without bold or italics. | Using double asterisks for bold or single asterisks for italics in text. |
| Lists | Use `-` for unordered lists and `1.` for ordered lists. | Using asterisks for unordered lists or mixing numbering styles. |
| Dividers | Use section headings to separate content. | Inserting horizontal rules (`---`) in the body. |
| Namespaces | Use `Folder::File` for grouped and `File` for autonomous. | Omitting namespaces or using un-delimited strings. |
| Linking | Author links as `[Namespace::Symbol](path)` in valid slots. | Cross-linking in Intent, Behaviors, Process, or Tables. |
| Empty Slots | Fill every linking slot with a cross-link or primitive. | Leaving parenthesized linking slots blank. |
| Infallibles | Use unit or void for parameterless or infallible slots. | Leaving slots blank or inventing dummy models. |
| Models | Use ALDT with required/optional for structs, variant for enums. | Placing sum types or enums in ABDT instead of ALDT. |
| Storage | Declare mandatory primary key; omit empty foreign keys. | Leaving empty foreign key or index bullet lines. |
| Commons | Use plain names without `0-` and `Commons::File` namespace. | Adding `0-` prefix to files in commons. |
| Additions | Add paragraphs, mermaid diagrams, and math blocks freely. | Adding unauthorized list structures or subheadings. |

### 8.2 Checklist
<a id="sec-8-2"></a>
Use this TL;DR checklist to verify that a specification file is fully compliant before committing:

1. Frontmatter contains title, namespace (`Word` or `Folder::File`), version, modified (dd-mm-yyyy), status, phase.
2. Root heading is exactly `# <Title>` with no prefixes.
3. Exactly 3 heading levels used (#, ##, ###); zero Level 4 or deeper headings exist.
4. Every `###` heading has `<a id="..."></a>` on the immediately following line.
5. All Level 3 headings declaring entities format their name as an inline code identifier in backticks after the colon.
6. Zero bold or italic markup exists in the entire document.
7. Zero horizontal divider lines (`---`) exist in the specification body.
8. All unordered lists use `-` exclusively (zero asterisk list bullets).
9. All ordered lists use `1.` numbering format.
10. All declared names, types, members, fields, and operations are inline code identifiers.
11. Models declare ALDT with columns `Member | Type | Requirement | Description`.
12. Sum types and enums use `Requirement: variant`, while struct fields use `required` or `optional`.
13. Abstract operational types are modeled under ABDT with columns `Operation | Description`.
14. All cross-references use `[Namespace::Symbol](path)` format and appear ONLY in permitted slots.
15. Zero cross-links exist in Section 1 (Intent), Section 2 (Behaviors), Section 5 Process, or Section 6 Tables.
16. Tests use `Input([Namespace::Symbol](path), ...):` and `Expect([Namespace::Symbol](path), ...):`.
17. Contract parameters use `<name>([Namespace::Model](path) or primitive):`.
18. Contract preconditions use `Assumption([Namespace::Symbol](path)):`.
19. Contract postconditions use `Guarantee([Namespace::Symbol](path)):`.
20. Contract outcomes use `Success([Namespace::Model](path)):` and `Error([Namespace::Symbol](path)):`.
21. Protocols use `Sender(...)`, `Receiver(...)`, `Payload(...)`, `Errors(...)` with valid cross-links.
22. All linking slots are filled; zero slots are left empty. Parameterless, bodyless, or infallible slots use `unit` or `void`.
23. Storage uses `### Table: `<name>`` with mandatory primary key; foreign keys and indexes are omitted if empty.
24. Extra explanatory paragraphs, mermaid diagrams, and math blocks appear only where needed.
