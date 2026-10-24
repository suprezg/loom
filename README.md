# Loom

<p align="center">
    <img src="./assets/banner.png" alt="Loom Banner">
    <br />
    <br />
    <img src="https://img.shields.io/badge/Standard-LSS--V1-brightgreen?style=for-the-badge&logo=markdown&logoColor=white" alt="LSS-V1">
    <img src="https://img.shields.io/badge/Compiler-Rust-black?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
    <img src="https://img.shields.io/badge/License-MIT_%7C_MPL--2.0-blue?style=for-the-badge" alt="License">
    <img src="https://img.shields.io/badge/Platforms-Linux_%7C_Windows-orange?style=for-the-badge" alt="Platforms">
    <br />
    <br />
    <i>An Architecture-as-Code compiler turning readable Markdown specifications into machine-parseable ASTs under an open, feature-first standard.</i>
</p>

> [!NOTE]
> **Active Development:** This project is currently in Version 1.0 (V1) active development.

## Abstract

Loom is an **Architecture as Code** ecosystem designed to build software systems from the feature perspective outward. Instead of scattering requirements across tickets, architecture diagrams, and fragmented codebases, Loom establishes a unified, feature-first methodology centered on human-readable Markdown specifications (`.spec.md`) that compile deterministically into machine-parseable Abstract Syntax Trees (ASTs).

Traditional specification tools like Gherkin capture high-level behavioral scenarios, yet leave low-level engineering elements—such as procedure contracts, data shapes, database schemas, and protocol interfaces—unspecified. Developers are left bridging the gap between business stories and concrete architecture by hand. Loom solves this by defining the **Loom Specification Standard (LSS-V1)**, an open standard where each feature is authoritatively specified across its full lifecycle: Intent, Behaviors, Tests, Models (ALDT and ABDT), Contracts, Storage, and Protocols.

Alongside the standard, Loom delivers a high-performance **Rust compiler (`loom`)** that validates specifications, verifies structural boundaries, and translates specifications into structured AST datasets. Downstream plugins ("Yarns") consume this AST to generate boilerplate, scaffolding, and validation suites across target languages. Loom also provides dedicated **Agent Skills**, enabling AI coding assistants to follow the specification standard and author compliant, production-grade specifications alongside human engineers.

## Objective

**Feature-First Holistic Specification:** Bridge the divide between user stories and system design by expressing business intent, behavioral workflows, verification test cases, data models, functional contracts, relational storage, and protocol interfaces in a single human-readable `.spec.md` document.

**Open & Machine-Parseable Standard:** Provide an unambiguous, paradigm-agnostic specification standard (LSS-V1) that is 100% rendered in standard Markdown viewers while compiling deterministically into a normalized AST for tooling, plugins, and automation.

**High-Speed Verification & Tooling Pipeline:** Deliver a fast, reliable Rust compiler (`loom`) equipped with commands for linting and architectural boundary checking (`scout`), plugin execution and AST compilation (`weave`), visual architecture graph generation (`tapestry`), and plugin installation (`spin`).

**Agentic Specification Workflows:** Provide dedicated agent skills that allow autonomous coding agents and LLM assistants to understand the standard, follow architectural conventions, and author robust specifications collaboratively.

## Features

### Functional

- **Markdown-Native Specification Standard (`.spec.md`):** Complete, human-readable specification format covering Intent, Behaviors (Given/When/Then), Tests, Models (ALDT & ABDT), Contracts, Storage, and Protocols without proprietary DSL friction.
- **Compiler Linting & Verification (`loom scout`):** Validates specification grammar, verifies flat group folder structures, checks mandatory anchor tags, resolves cross-references, and blocks architectural anti-patterns like cross-group coupling.
- **AST Compilation & Plugin Pipeline (`loom weave`):** Compiles verified `.spec.md` specifications into a standardized JSON AST and pipes it directly into user-selected plugins ("Yarns") for code generation, migrations, and scaffolding.
- **Visual Architecture Graphing (`loom tapestry`):** Generates comprehensive visual dependency graphs connecting models, contracts, database tables, and communication channels.
- **Plugin Management (`loom spin`):** Fetches, validates, and installs external code-generation plugins ("Yarns") into the local toolchain.
- **Agent Skills:** Pre-packaged AI agent skill runbooks allowing coding assistants to navigate the LSS-V1 standard, design compliant specifications, and audit specifications against strict lint rules.
- **Rich Modeling Constructs:** Native support for Algebraic Data Types (`ALDT`) capturing product fields and sum variants, as well as Abstract Data Types (`ABDT`) defining operational interfaces.
- **VS Code Extension:** TextMate syntax highlighting, validation diagnostics, snippets, and navigation badges for `.spec.md` specifications.

### Non-Functional

- **Lightning-Fast Rust Toolchain:** Built in Rust for sub-millisecond parsing, boundary validation, and AST compilation.
- **Zero Language or Paradigm Lock-In:** Paradigm-agnostic design supporting Object-Oriented (OOP), Functional (FP), Data-Oriented (DOD), and Event-Driven architectures equally.
- **Clean Git Diffs & High Readability:** Markdown-native structured lists and tables produce clean line-by-line diffs without horizontal scrolling or alignment churn.
- **Balanced Dual Open-Source Licensing:** The Specification Standard and Agent Skills are licensed under the permissive MIT License; the core Rust Compiler engine is protected under the Mozilla Public License 2.0 (MPL 2.0).

## Prerequisites

### Requirements

To compile and build Loom from source on your local system:

- **Rust Toolchain:** Stable Rust compiler and package manager (`rustc` and `cargo`, edition 2021 or newer).
- **Build Tools:** Standard system linker and build tools (`gcc`, `clang`, or MSVC).
- **Version Control:** Git for cloning the repository.
- **Operating System:** Linux or Windows.

### Dependencies

To run and use Loom directly without building from source:

- **Pre-Compiled Binary:** Standalone `loom` CLI binary downloaded for your platform (Linux or Windows).
- **Specification Standard (LSS-V1):** `.spec.md` specification files authored according to the standard.
- **Agent Skills:** Pre-packaged Loom skills for AI assistants and agentic workflows to author and inspect specifications.
- **Markdown Environment:** Any standard text editor, IDE (such as VS Code), or Markdown viewer.

## Getting Started

Installation and Usage Guidelines are inside the [begin.md](docs/begin.md) file.