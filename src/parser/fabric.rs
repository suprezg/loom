/*
File Name: fabric.rs
Purpose: Pest parser implementation for .fabric macro architecture grammar.
*/

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/fabric.pest"]
pub struct FabricParser;
