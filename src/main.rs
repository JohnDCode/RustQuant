/*

RustQuant
/src/main.rs
JohnDavid Abe

*/



// Modules
mod commands;
mod fetch;
mod greeks;
mod binomial;
mod black_scholes;

// Packages
use clap::{Parser, Subcommand};



// CLI Parser
#[derive(Parser)]
#[command(
    name = "RustQuant",
    about = "A CLI tool used to evaluate and price American and European financial options",
    author = "JohnDavid Abe --> https://www.johndcode.com/",
    version = "0.4",
    arg_required_else_help = true
)]

// Struct to dispatch commands
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Commands and arguments
#[derive(Subcommand)]
enum Commands {

    // Price options using automatically pulled data from live options chain
    Auto {

        #[arg(short = 's', long = "symbol", required = true)]
        symbol: String,

        #[arg(short = 'k', long = "strike", required = true)]
        strike: f64,

        #[arg(short = 'n', long = "steps", default_value_t = 100)]
        steps: u32,

        #[arg(short = 'c', long = "call", default_value_t = false)]
        call: bool,

        #[arg(short = 'p', long = "put", default_value_t = false)]
        put: bool,

        #[arg(short = 'g', long = "greeks", default_value_t = false)]
        greeks: bool,
    },

    // Price options using manually inputted data
    Manual {

        // Command argument list
        #[arg(short = 's', long = "spot", required = true)]
        spot: f64,

        #[arg(short = 'k', long = "strike", required = true)]
        strike: f64,

        #[arg(short = 't', long = "time", required = true)]
        time: f64,

        #[arg(short = 'r', long = "rate", required = true)]
        rate: f64,

        #[arg(short = 'v', long = "volatility", required = true)]
        volatility: f64,

        #[arg(short = 'n', long = "steps", default_value_t = 100)]
        steps: u32,

        #[arg(short = 'c', long = "call", default_value_t = false)]
        call: bool,

        #[arg(short = 'p', long = "put", default_value_t = false)]
        put: bool,

        #[arg(short = 'a', long = "american", default_value_t = false)]
        american: bool,

        #[arg(short = 'e', long = "european", default_value_t = false)]
        european: bool,

        #[arg(short = 'g', long = "greeks", default_value_t = false)]
        greeks: bool,

    }
}



// Entry point for the binary
#[tokio::main]
async fn main() {

    // Attempt to populate cli struct
    let cli = Cli::parse();

    // Dispatch proper command
    match cli.command {

        // Price options automatically
        Commands::Auto { symbol, strike, steps, call, put, greeks } => {
            commands::auto::run(symbol, strike, steps, call, put, greeks).await;
        }

        // Price symbols manually
        Commands::Manual { spot, strike, time, rate, volatility, steps, call, put, american, european, greeks } => {
            commands::manual::run(spot, strike, time, rate, volatility, steps, call, put, american, european, greeks);
        }
    }
}

