---
name: spec-designer
description: Design, review, modify, add, or remove Loom contract and behavior specifications (.thread and .fabric files). Always use this skill whenever the user mentions "lets design specifications", "project specification review/modification/addition/removing", "i want to design this specification for this project", "i want to make change in this specification", "review specifications", "audit Loom specs", or wants to create, edit, inspect, validate, or weave software behavior specifications and architectural blueprints.
---

# Spec Designer: Loom Specification Architecture & Design Guide

Spec Designer is your dedicated guide for designing, reviewing, adding, modifying, removing, and compiling Loom specifications (`.thread` and `.fabric` files).

Loom operates on a contract-driven, behavioral specification model that links user-facing feature acceptance criteria with internal component invariants, formal contracts, data models, async communication protocols, database storage schemas, and macro-architecture visual blueprints.

---

## 1. The Loom Specification Paradigm

A Loom system specification is composed of two primary file types:

1. **Thread Specifications (`.thread`)**: Fine-grained behavioral and structural declarations consisting of four universal entity types:
   - **`Feature`**: Captures user behaviors, rules, Gherkin-style scenarios, and acceptance criteria.
   - **`Component`**: Defines internal service logic, invariants, models (`Struct`, `Enum`, `Primitive`), and formal contracts (`Signature`, `Requires`, `Ensures`, `Precondition`, `Postcondition`, `Process`, `Errors`).
   - **`Protocol`**: Defines asynchronous communication channels, transport patterns (`AsyncBroadcast`, `PointToPoint`, `PubSub`), senders, receivers, and message payloads.
   - **`Storage`**: Defines database engines, tables, fields, constraints (`PrimaryKey`, `NonNull`, `Unique`, `ForeignKey`), indexes, and relational cardinalities (`1:1`, `1:N`, `N:1`, `N:M`).

2. **Fabric Blueprints (`.fabric`)**: Macro-architectural topology and system boundaries:
   - **`system`**: System title.
   - **`group`**: Visual cluster grouping on canvas view.
   - **Connections (`->`)**: Directed dependency and communication flows between entities and contracts.

---

## 2. Green Flags (Best Practices for Clean Spec Design)

- **Consistent Identifiers**: Always start identifiers with `[a-zA-Z_]` using camelCase or PascalCase (e.g., `AuthService`, `login`, `TokenBroadcastPipe`).
- **Precise Scoped Paths**: Use `EntityName::MemberName` (e.g., `@component(AuthService::login)`).
- **Universal Cross-Linking**:
  - Decorate Feature scenarios with `@component(Service::method)` to anchor scenarios to real contracts.
  - Decorate Contracts and Models with `@storage(Storage::table)` to trace data dependencies.
  - Decorate Channels with `@component(...)` and `@storage(...)` to trace emission and persistence.
- **Embedded Mermaid Diagrams**: Add `!Diagram <Id> [[ ```mermaid ... ``` ]]` for sequence flows, state diagrams, and ERDs. Link them to scenarios or contracts via `@diagram(<Id>)`.
- **Formal Contract Completeness**: Always provide explicit `Signature`, `Requires`, `Ensures`, `Precondition`, `Postcondition`, and step-by-step numbered `Process` blocks.
- **Strict Storage Constraints**: Include explicit primary keys, non-null flags, foreign keys, and indexes for queried columns.
- **Topological Fabric Blueprints**: Map high-level flows between entities in `.fabric` with clear descriptive labels (e.g., `Feature.Auth -> Component.AuthService::login : "Triggers login"`).

---

## 3. Red Flags & Common Pitfalls

Loom validates syntax and semantics strictly through two diagnostic tiers:

### Tier 1: Syntactic Errors (Parser Level)
- **`LM0001` / `LM1001` (Illegal Identifier)**: Starting an identifier with a digit or hyphen (e.g., `Feature 123Auth`).
- **`LM0002` / `LM1002` (Unclosed Block)**: Missing a closing brace `}` or unclosed string/bracket.
- **`LM0003` / `LM1003` (Malformed Scoped Path)**: Dangling colons (e.g., `AuthService::` or `::login` or `AuthService:::login`).

### Tier 2: Semantic Errors & Warnings (Analyzer Level)
- **`LM2001` (Unresolved Reference)**: Decorator points to a non-existent entity, contract, or model (`@component(UnknownService::method)`).
- **`LM2002` (Duplicate Declaration)**: Two entities or members share the same name in scope.
- **`LM2003` (Storage Index Column Error)**: Indexing a column that is not declared in `Fields`.
- **`LM2004` (Storage Relation Target Error)**: `Relations` block references a non-existent table or column.
- **`LM2005` (Protocol Channel Target Error)**: Channel `Sender`, `Receiver`, or `Payload` references an undefined symbol.
- **`LM2006` (Missing Scenario Decorator - Warning)**: A Scenario is declared without any `@component`, `@storage`, or `@protocol` link.
- **`LM2007` (Unused Symbol - Warning)**: An entity or member is declared but never referenced in any decorator, relation, or topology flow.
- **`LM3001` (Fabric Unresolved Reference)**: `.fabric` blueprint connection references an entity or member not present in any `.thread` file.

*For comprehensive error explanations and resolution examples, see [references/diagnostics.md](references/diagnostics.md).*

---

## 4. Specification Lifecycle Workflows

### Workflow 1: Understanding & Exploring Specifications
1. Locate all `.thread` and `.fabric` files in the project.
2. Read `.fabric` first to understand the macro-level system architecture and cluster groups.
3. Trace `.thread` entities (`Feature` -> `Component` -> `Protocol` -> `Storage`) along the decorator relationships.
4. Validate the current state using `loom scout`.

### Workflow 2: Reviewing Specifications
1. **Syntactic Check**: Verify balanced braces, valid identifiers, and well-formed scoped paths.
2. **Semantic Cross-Reference Check**: Verify all `@component`, `@storage`, and `@protocol` decorators resolve to actual declared members.
3. **Behavioral Integrity**: Ensure every `Scenario` has a backing `@component` contract, and every contract has preconditions, postconditions, and clear step-by-step processes.
4. **Data Integrity**: Verify database table relations (`1:N`, `1:1`) match ERD diagrams and have corresponding foreign keys and indexes.
5. **Linting Warnings**: Check for orphaned or unused symbols (`LM2007`) or undecorated scenarios (`LM2006`).

### Workflow 3: Adding New Specifications
1. **Define the Feature**: Create user-facing scenarios in a `.thread` file with BDD steps (`Given`, `When`, `Then`).
2. **Define the Component & Contract**: Define the service contract implementing the behavior with signatures, preconditions, and process steps.
3. **Define Models & Storage**: Declare data structures and database tables for persistence.
4. **Define Channels**: If messaging or async events are involved, declare the `Channel` under a `Protocol`.
5. **Connect via Decorators**: Add `@component(...)`, `@storage(...)`, and `@protocol(...)` decorators to link the entities.
6. **Update the Fabric**: Add the new entities into appropriate cluster groups and add connection arrows in `.fabric`.
7. **Validate**: Run `loom scout` to ensure clean compilation.

### Workflow 4: Modifying Existing Specifications
1. Search for all references to the entity/member being modified (decorators in `.thread` and connection arrows in `.fabric`).
2. Apply changes across contracts, fields, and decorators simultaneously.
3. If renaming a symbol, update all decorator callers to prevent `LM2001` / `LM3001` errors.
4. Run `loom scout` to verify that all cross-references remain intact.

### Workflow 5: Removing Specifications
1. Identify all incoming references to the target entity/member across all `.thread` and `.fabric` files.
2. Remove or replace dependent decorators and connection edges.
3. Remove the target declaration.
4. Run `loom scout` to ensure no dangling references remain.

---

## 5. Loom CLI Verification & Compilation

Always verify and compile specifications using the Loom CLI:

### Validate Specifications (Syntax & Semantics)
```bash
# Validate single or multiple files / directories
cargo run --bin loom -- scout examples/valid
# Or using the built binary
loom scout examples/valid/authentication.thread examples/valid/auth_service.thread examples/valid/system.fabric
```

### Weave Specifications (Compile to JSON AST Documentation)
```bash
# Weave specifications into structured JSON files in target output directory
cargo run --bin loom -- weave examples/valid -o data
# Or using the built binary
loom weave examples/valid -o data
```

### CLI Log Level Flags
- `-v`, `--verbose`: Enable detailed trace logging of token parsing, span offsets, and discovery.
- `-q`, `--quiet`: Suppress advisory logs, showing only diagnostic errors and warnings.

---

## 6. Reference Guides

- **[Grammar Reference](references/grammar.md)**: Full PEG grammar syntax and structural rules for all entities.
- **[Diagnostics & Error Reference](references/diagnostics.md)**: Complete guide to `LM0001`-`LM3001` codes, root causes, and fixes.
- **[Valid Examples Reference](references/examples.md)**: Deep dive into the 5 reference specifications in `examples/valid/`.
