use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dialoguer::{Input, Select};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "spm")]
#[command(about = "Science Package Manager - Track papers and funding", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new science project
    Init,
    /// Add a paper reference
    Add {
        /// Paper citation (e.g., "notion (2024)")
        citation: String,
    },
    /// Configure funding information
    Fund,
}

#[derive(Serialize, Deserialize, Default)]
struct FundingInfo {
    ethereum_address: Option<String>,
    optimism_address: Option<String>,
    celo_address: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
struct Paper {
    citation: String,
    doi: Option<String>,
    url: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
struct ScienceConfig {
    papers: Vec<Paper>,
}

fn init_project() -> Result<()> {
    // Initialize FUNDING.json if it doesn't exist
    if !Path::new("FUNDING.json").exists() {
        let funding = FundingInfo::default();
        fs::write(
            "FUNDING.json",
            serde_json::to_string_pretty(&funding)?,
        )?;
    }

    // Initialize SCIENCE.json if it doesn't exist
    if !Path::new("SCIENCE.json").exists() {
        let science = ScienceConfig::default();
        fs::write(
            "SCIENCE.json",
            serde_json::to_string_pretty(&science)?,
        )?;
    }

    println!("Initialized new science project");
    Ok(())
}

fn add_paper(citation: String) -> Result<()> {
    let mut science: ScienceConfig = if Path::new("SCIENCE.json").exists() {
        let content = fs::read_to_string("SCIENCE.json")?;
        serde_json::from_str(&content)?
    } else {
        ScienceConfig::default()
    };

    let doi: String = Input::new()
        .with_prompt("DOI (optional, press enter to skip)")
        .allow_empty(true)
        .interact()?;

    let url: String = Input::new()
        .with_prompt("URL (optional, press enter to skip)")
        .allow_empty(true)
        .interact()?;

    science.papers.push(Paper {
        citation,
        doi: if doi.is_empty() { None } else { Some(doi) },
        url: if url.is_empty() { None } else { Some(url) },
    });

    fs::write(
        "SCIENCE.json",
        serde_json::to_string_pretty(&science)?,
    )?;

    println!("Added paper to SCIENCE.json");
    Ok(())
}

fn configure_funding() -> Result<()> {
    let mut funding: FundingInfo = if Path::new("FUNDING.json").exists() {
        let content = fs::read_to_string("FUNDING.json")?;
        serde_json::from_str(&content)?
    } else {
        FundingInfo::default()
    };

    let options = vec!["Ethereum", "Optimism", "Celo"];
    let selection = Select::new()
        .with_prompt("Select blockchain for funding")
        .items(&options)
        .interact()?;

    let address: String = Input::new()
        .with_prompt("Enter your wallet address")
        .interact()?;

    match selection {
        0 => funding.ethereum_address = Some(address),
        1 => funding.optimism_address = Some(address),
        2 => funding.celo_address = Some(address),
        _ => unreachable!(),
    }

    fs::write(
        "FUNDING.json",
        serde_json::to_string_pretty(&funding)?,
    )?;

    println!("Updated funding information");
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init_project()?,
        Commands::Add { citation } => add_paper(citation)?,
        Commands::Fund => configure_funding()?,
    }

    Ok(())
}