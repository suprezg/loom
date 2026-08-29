/*
File Name: fabric.rs
Purpose: Pest PEG grammar parser definition for Loom Fabric blueprint files.
*/

#![allow(non_snake_case)]

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/fabric.pest"]
pub struct FabricParser;
