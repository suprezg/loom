/*
File Name: thread.rs
Purpose: Pest parser implementation for .thread specification grammar.
*/

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/thread.pest"]
pub struct ThreadParser;
