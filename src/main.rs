#![feature(const_option_ext)]
use clap::{Parser, ValueEnum};
use json::object;
use std::fs;
use tlb_simulator::{Simulation, SimulationResult};

use tabled::{builder::Builder, Style};

const TLB_SIZE: usize = parse_usize(option_env!("TLB_SIZE").unwrap_or("2"));

// 🙇
const fn parse_usize(string: &str) -> usize {
    let mut res: usize = 0;
    let mut bytes = string.as_bytes();
    while let [byte, rest @ ..] = bytes {
        bytes = rest;
        if let b'0'..=b'9' = byte {
            res *= 10;
            res += (*byte - b'0') as usize;
        } else {
            panic!("not a number")
        }
    }
    res
}

#[derive(Parser, Debug)]
#[command(author = "Pablo Alessandro Santos Hugen", version = "0.1.0", about = "A command line tool for TLB simulations", long_about = None)]
struct Cli {
    #[arg(long, default_value = "1.0")]
    hit_cycles: f64,
    #[arg(long, default_value = "30.0")]
    mem_cycles: f64,
    #[arg(long, default_value = "text")]
    output: Output,
    #[arg(default_value = "/dev/stdin")]
    trace: String,
}

#[derive(ValueEnum, Debug, Clone)]
enum Output {
    Text,
    Json,
    Table,
}

impl Cli {
    fn simulate(&self) -> Result<SimulationResult, Box<dyn std::error::Error>> {
        let mut simulation: Simulation<32, 12, TLB_SIZE> =
            Simulation::new(&fs::read_to_string(&self.trace)?)?;

        Ok(simulation.run()?)
    }

    fn to_text(&self, result: SimulationResult) -> String {
        format!("Hits{}\nMisses{}\nTotal{}\nHit rate: {:.2}% Miss rate: {:.2}%\nEffective Memory Cycle Rate: {:.2} Cycles/Memory Access",
                result.hits,
                result.misses,
                result.total,
                result.hit_rate * 100.0,
                result.miss_rate * 100.0,
                (result.effective_memory_cycle_rate)(self.mem_cycles, self.hit_cycles)
        )
    }

    fn to_json(&self, result: SimulationResult) -> String {
        json::stringify_pretty(
            object! {
                hits: result.hits,
                misses: result.misses,
                total: result.total,
                hit_rate: result.hit_rate,
                miss_rate: result.miss_rate,
                effective_memory_cycle_rate: (result.effective_memory_cycle_rate)(self.mem_cycles, self.hit_cycles)
            },
            4,
        )
    }

    fn to_table(&self, result: SimulationResult) -> String {
        let mut builder = Builder::default();
        builder
            .set_columns([
                "Hits".to_string(),
                "Misses".to_string(),
                "Total".to_string(),
                "Hit rate (%)".to_string(),
                "Miss rate (%)".to_string(),
                "(Cicle/Access)".to_string(),
            ])
            .add_record([
                result.hits.to_string(),
                result.misses.to_string(),
                result.total.to_string(),
                format!("{:.2}%", result.hit_rate * 100.0),
                format!("{:.2}%", result.miss_rate * 100.0),
                format!(
                    "{:.2}",
                    (result.effective_memory_cycle_rate)(self.mem_cycles, self.hit_cycles)
                ),
            ]);

        let mut table = builder.build();
        table.with(Style::modern()).to_string()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let result = cli.simulate()?;

    println!(
        "{}",
        match cli.output {
            Output::Text => cli.to_text(result),
            Output::Json => cli.to_json(result),
            Output::Table => cli.to_table(result),
        }
    );

    Ok(())
}
