# Loom

<p align="center">
    <img src="./assets/banner.png" alt="Loom Banner">
    <br />
    <br />
    <img src="https://img.shields.io/badge/License-MIT-yellow?style=for-the-badge" alt="License">
    <img src="https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
    <img src="https://img.shields.io/badge/Platforms-Linux_|_Windows-blue?style=for-the-badge" alt="Platforms">
    <br />
    <br />
    <i>An Architecture-as-Code compiler to weave behavioral specifications, component contracts, storage schemas, and protocol channels into structured JSON ASTs, paired with a stateless React SPA web viewer.</i>
</p>

> [!NOTE]
> **Active Development:** This project is currently a work in progress and is being actively built.

## Abstract

Loom is an **Architecture as Code** tool designed to unify high-level system behaviors, low-level component contracts, database storage schemas, and inter-component communication channels into a single specification-to-JSON workflow. 

Traditional specification frameworks like Gherkin excel at defining high-level behavioral scenarios and grouping them into logical features. However, when designing lower-level architectural elements—such as gRPC servers, WebAssembly runtimes, database storage tables, or async message queues—no equivalent behavioral specification standard exists. Developers often face a disjointed process when transitioning from user stories to component-level system design. 

Loom solves this by introducing a unified multi-entity specification language. It combines Gherkin-inspired behavioral scenarios with Design-by-Contract (DbC) component contracts, relational database schemas, and message channel protocols within `.thread` specification files, complemented by optional macro-architecture blueprints (`.fabric`). Loom compiles these specifications into standardized JSON AST datasets. 

To visualize specifications, Loom includes a supplementary, pre-compiled React Single Page Application (SPA) web viewer bundle. The React SPA is completely stateless—it dynamically reads and renders whichever JSON AST dataset is provided to it. Swapping the underlying JSON file instantly updates the UI, giving developers total creative freedom to use the provided React viewer out-of-the-box or build custom web portals, documentation generators, and publication pipelines in any language or framework of their choice.

## Objective

**Unify Behavioral and Structural Specs:** Bridge the gap between high-level user stories, low-level component contracts, storage schemas, and communication channels under a single, human-readable specification umbrella.

**Formalize Component Contracts & Schemas:** Provide a structured language for defining component specifications with explicit invariants, preconditions, postconditions, and process steps, relational database schemas (fields, indexes, relations), and protocol channels (pattern, transport, sender, receiver, payload).

**JSON AST Compilation & Stateless Web Viewing:** Compile machine-parseable specifications into clean JSON AST datasets paired with a supplementary, stateless React SPA viewer bundle. Swapping the underlying JSON dataset instantly updates the React viewer UI without backend re-compilation.

**Maintain Design Integrity:** Provide robust semantic analysis pipelines to verify syntax correctness, cross-reference integrity (`@feature`, `@component`, `@storage`, `@protocol`), and diagram linkages, producing standardized diagnostic error codes (`LM0001`–`LM4000`).

## Features

### Functional

- **Unified 4-Entity Parser:** Support for `Feature`, `Component`, `Storage`, and `Protocol` entities parsed from single or multiple `.thread` specification files.
- **Component & Schema Specification:** Express contracts for lower-level architectural components (invariants, models, signatures, preconditions, postconditions, process steps, errors), database storage tables (fields, indexes, relations), and communication protocol channels.
- **Macro-System Blueprinting:** Uses an optional macro-architecture blueprint (`.fabric`) to define cluster groups (`group`) and macro-topology connection edges (`->`).
- **Mermaid Diagram Integration:** Parses embedded Mermaid code blocks in `!Diagram` directives (`[[ ```mermaid ... ``` ]]`) and cross-references them across entities via `@diagram`.
- **Stateless React SPA Viewer:** Supplementary, pre-compiled React SPA web bundle that dynamically renders JSON AST datasets with an interactive pan/zoom canvas, global fuzzy search, dark mode, and entity cards.
- **Developer Decoupling & Creative Freedom:** Because Loom outputs standardized JSON AST datasets, developers are not locked into a single UI and can freely build custom doc generators, web portals, or publication pipelines in any programming language.
- **Semantic Analysis & Validation:** Structural semantic analyzer (`comb`) to verify syntax rules, resolve cross-references across entities, validate diagram linkages, and report standardized `LMxxxx` diagnostic error codes.
- **VS Code Extension:** TextMate syntax highlighting, context-aware autocompletion for `Feature`, `Component`, `Storage`, `Protocol`, and `.fabric` blueprints, entity boilerplate snippets (`$Thread.Feature`, `$Thread.Component`, `$Thread.Storage`, `$Thread.Protocol`, `$Fabric.System`), and custom file icon badges.
- **Model Context Protocol (MCP) Server:** Expose Loom's AST and specification metadata to agentic workflows and LLMs via standard MCP tool endpoints.
- **Loom Skills:** Pre-packaged agent workflows and specification templates to boost specification writing productivity.

### Non-Functional

- **Lightning-Fast Execution:** Built in Rust to perform parsing, extraction, semantic analysis, and JSON AST compilation in milliseconds.
- **Textile Metaphor Coherency:** System commands and entities are cleanly structured around intuitive textile concepts: *Loom* (compiler engine), *Fabric* (system blueprints), *Threads* (specification files), *Weave* (compilation pipeline), *Comb* (semantic analyzer), and *Ravel* (decompiler).
- **Decoupled Architecture:** Emits clean, machine-readable JSON AST datasets with zero backend or server lock-in.
- **Type-Safe Validation:** Leverages Rust's type system and Pest PEG parser to ensure safe parallel processing, exact AST mapping, and clean error handling.

## Specifications

### Requirements

- **Threads:** Specification files (`.thread`) containing `Feature`, `Component`, `Storage`, or `Protocol` entities with documentation notes (`!Note [[ ... ]]`) and diagrams (`!Diagram ... [[ ... ]]`).
- **Fabric File:** Optional macro-system architecture blueprint (`.fabric`) defining cluster groups and macro connection topology.
- **Compiler Engine:** Core Rust CLI (`loom`) running syntactic parsing, semantic analysis, and JSON AST compilation.
- **React SPA Viewer:** Pre-compiled React production web app bundle serving as a stateless viewer for compiled JSON AST datasets.

### Dependencies

- **Rust Toolchain:** The compiler and package manager required to build the CLI (`cargo`).
- **Operating System:** Windows or Linux.

## Getting Started

Installation and Usage Guidelines are inside the `docs/begin.md` file.
