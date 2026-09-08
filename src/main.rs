/*
File Name: main.rs
Purpose: Main entry point for the Loom compiler CLI binary supporting weave and scout commands with configurable log levels.
*/

#![allow(non_snake_case)]

use clap::{Parser, Subcommand};
use loom::helpers::diagnostics::{initLogLevel, LogLevel};
use loom::services::compiler::{build, validate};

/*
Command-line arguments parser for the Loom compiler.
*/
#[derive(Parser, Debug)]
#[command(
    name = "loom",
    version,
    about = "A behavior and contract driven specification compiler"
)]
struct Cli
{
    #[arg(
        short = 'v',
        long = "verbose",
        global = true,
        help = "Set log level to verbose",
        conflicts_with = "quiet"
    )]
    verbose: bool,

    #[arg(
        short = 'q',
        long = "quiet",
        global = true,
        help = "Set log level to quiet",
        conflicts_with = "verbose"
    )]
    quiet: bool,

    #[command(subcommand)]
    command: Commands,
}

/*
Available CLI subcommands for Loom compiler workflows.
*/
#[derive(Subcommand, Debug)]
enum Commands
{
    #[command(about = "Weave specification files into unified JSON AST documentation")]
    Weave
    {
        #[arg(help = "Path to the input specification file or directory")]
        input: String,

        #[arg(short = 'o', long = "output", help = "Path to the output directory")]
        output: String,
    },
    #[command(about = "Scout and validate specification files for syntactic and semantic correctness")]
    Scout
    {
        #[arg(help = "Path to the input specification file or directory")]
        input: String,
    },
}

/**
 * Main entry point for the Loom CLI compiler binary.
 *
 * Takes:
 * 	None.
 *
 * Gives:
 * 	(): Unit type.
 */
fn main() -> ()
{
    let cli = Cli::parse();

    let logLevel = if cli.verbose {
        LogLevel::Verbose
    } else if cli.quiet {
        LogLevel::Quiet
    } else {
        LogLevel::Normal
    };

    initLogLevel(logLevel);

    match cli.command {
        Commands::Weave { input, output } => {
            if let Err(_) = build(&input, &output) {
                std::process::exit(1);
            }
        }
        Commands::Scout { input } => {
            if let Err(_) = validate(&input) {
                std::process::exit(1);
            }
        }
    }
}
