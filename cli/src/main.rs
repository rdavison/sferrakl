use clap::{Parser, Subcommand, ValueEnum};
use core::percentage::T;
use sferrakl_model::keyboard::{Keyboard, Layout, Src};
use sferrakl_model::tier::assign_tier;
use std::collections::HashMap;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(ValueEnum, Clone, Debug)]
enum LayoutSelection {
    Qwerty,
    Dvorak,
    Colemak,
}

#[derive(Subcommand)]
enum Commands {
    /// Prints "Hello, world!"
    Hello,
    /// Generates a histogram of 3-stroke tiers
    Trigrams {
        #[arg(short, long, value_enum, default_value_t = LayoutSelection::Qwerty)]
        layout: LayoutSelection,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Hello => {
            println!("Hello, world!");
        }
        Commands::Trigrams { layout } => {
            let keymap = Src::Ansi.keymap();
            println!("Keyboard Layout:\n{}\n", keymap);

            let mut keyboard = Keyboard::new(&keymap);
            let model_layout = match layout {
                LayoutSelection::Qwerty => Layout::Qwerty,
                LayoutSelection::Dvorak => Layout::Dvorak,
                LayoutSelection::Colemak => Layout::Colemak,
            };
            keyboard.set_layout(&model_layout);

            let trigrams_iter = keyboard.nstrokes(3);
            let mut tier_histogram: HashMap<String, u32> = HashMap::new();
            let mut all_tiers: Vec<f64> = Vec::new();

            for trigram in trigrams_iter {
                if let Some(tier) = assign_tier(&trigram) {
                    let tier_value = T::as_f64(&tier);
                    all_tiers.push(tier_value);
                    let tier_str = format!("{:.0}%", tier_value * 100.0);
                    let entry = tier_histogram.entry(tier_str).or_insert(0);
                    *entry += 1;
                }
            }

            println!("--- Stroke Tier Histogram ---");
            let mut sorted_histogram: Vec<_> = tier_histogram.iter().collect();
            sorted_histogram
                .sort_by_key(|(tier, _)| tier.trim_end_matches('%').parse::<u32>().unwrap_or(0));

            for (tier, count) in sorted_histogram {
                println!("{}: {}", tier, count);
            }

            println!("\n--- Statistics ---");
            // Mean
            let mean = all_tiers.iter().sum::<f64>() / all_tiers.len() as f64;
            println!("Mean: {:.2}%", mean * 100.0);

            // Median
            all_tiers.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mid = all_tiers.len() / 2;
            let median = if all_tiers.len() % 2 == 0 {
                (all_tiers[mid - 1] + all_tiers[mid]) / 2.0
            } else {
                all_tiers[mid]
            };
            println!("Median: {:.2}%", median * 100.0);

            // Mode
            let mode = tier_histogram
                .into_iter()
                .max_by_key(|&(_, count)| count)
                .map(|(tier, _)| tier)
                .unwrap_or_else(|| "N/A".to_string());
            println!("Mode: {}", mode);
        }
    }
}
