/*
File Name: lib.rs
Purpose: Root library entry point for the Loom compiler.
*/

#![allow(non_snake_case)]

/**
 * Module containing helper utilities and engines like diagnostics and loggers.
 */
pub mod helpers;

/**
 * Module containing structural models and enums representing compiler ASTs and diagnostics.
 */
pub mod models;
