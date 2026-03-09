use anyhow::Result;
use clap::Parser;
use parquet_peek::collect_head;

/// A tool to sneak peek parquet files
#[derive(Parser)]
#[command(about = "A tool to sneak peek parquet files", version)]
struct Cli {
    /// Parquet file to peek
    #[arg(short = 'p', long)]
    parquet_fn: String,

    /// How many lines to show
    #[arg(short = 'l', long, default_value_t = 10)]
    lines_to_show: u32,

    /// Char width to print in each column of the data frame
    #[arg(short = 'w', long, default_value_t = 250)]
    width: usize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Configure polars display formatting before reading data
    // Safety: called in main before any threads are spawned
    std::env::set_var("POLARS_FMT_STR_LEN", cli.width.to_string());
    std::env::set_var("POLARS_TABLE_WIDTH", cli.width.to_string());
    std::env::set_var("POLARS_FMT_MAX_COLS", "-1");

    let df = collect_head(&cli.parquet_fn, cli.lines_to_show)?;
    println!("{df}");

    Ok(())
}
