use anyhow::Context;
use applykit_core::types::{Baseline, GenerateInput, Track};
use applykit_core::{generate_packet, GenerateOptions};
use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "applykit")]
#[command(version)]
#[command(about = "ApplyKit local-first packet generator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Generate {
        #[arg(long)]
        company: String,
        #[arg(long)]
        role: String,
        #[arg(long)]
        source: String,
        #[arg(long)]
        baseline: String,
        #[arg(long)]
        jd: PathBuf,
        #[arg(long)]
        outdir: Option<PathBuf>,
        #[arg(long)]
        date: Option<String>,
        #[arg(long)]
        track_override: Option<String>,
        #[arg(long, default_value_t = false)]
        allow_unapproved: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cwd = std::env::current_dir().context("reading cwd")?;

    match cli.command {
        Commands::Generate {
            company,
            role,
            source,
            baseline,
            jd,
            outdir,
            date,
            track_override,
            allow_unapproved,
        } => {
            let jd_text = std::fs::read_to_string(&jd)
                .with_context(|| format!("reading JD file {}", jd.display()))?;
            let baseline = baseline.parse::<Baseline>().map_err(anyhow::Error::msg)?;
            let date = match date {
                Some(v) => Some(NaiveDate::parse_from_str(&v, "%Y-%m-%d")?),
                None => None,
            };
            let track_override = match track_override {
                Some(v) => Some(v.parse::<Track>().map_err(anyhow::Error::msg)?),
                None => None,
            };

            let result = generate_packet(
                GenerateInput {
                    company,
                    role,
                    source,
                    baseline,
                    jd_text,
                    outdir,
                    run_date: date,
                    track_override,
                    allow_unapproved,
                },
                GenerateOptions { repo_root: cwd },
            )?;

            println!("Packet generated successfully");
            println!("Track: {}", result.track.selected);
            println!("Fit Score: {}", result.fit.total);
            println!("Output Dir: {}", result.packet_dir.display());
            println!("Files:");
            for path in result.files_written {
                println!("- {}", path.display());
            }
        }
    }

    Ok(())
}
