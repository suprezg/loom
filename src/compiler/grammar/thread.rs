/*
File Name: thread.rs
Purpose: Pest PEG grammar parser definition for Loom Thread specification files.
*/

#![allow(non_snake_case)]

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/thread.pest"]
pub struct ThreadParser;
