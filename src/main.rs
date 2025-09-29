use dist_plugin::builder::{CBuilder, RustBuilder};
use dist_plugin::plugin::DistPlugin;
use nih_plug::nih_export_standalone;
use std::io::{self, Write};

fn main() {
    println!("Select backend:");
    println!("1 = RustDist");
    println!("2 = CDist");
    print!("Choice: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => {
            nih_export_standalone::<DistPlugin<RustBuilder>>();
        }
        "2" => {
            nih_export_standalone::<DistPlugin<CBuilder>>();
        }
        _ => {
            eprintln!("Invalid choice, defaulting to RustDist.");
            nih_export_standalone::<DistPlugin<RustBuilder>>();
        }
    }
}
