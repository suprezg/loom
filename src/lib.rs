/*
File Name: lib.rs
Purpose: Root library entry point for the Loom compiler.
*/

#![allow(non_snake_case)]

/**
 * Module containing helper utilities, diagnostics loggers, path resolvers, and models.
 */
pub mod helpers;

/**
 * Module containing Pest parsers for .thread and .fabric specifications.
 */
pub mod parser;

