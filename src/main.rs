use dist_plugin::backend::{CDistFactory, RustDistFactory};
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
            nih_export_standalone::<DistPlugin<RustDistFactory>>();
        }
        "2" => {
            nih_export_standalone::<DistPlugin<CDistFactory>>();
        }
        _ => {
            eprintln!("Invalid choice, defaulting to RustDist.");
            nih_export_standalone::<DistPlugin<RustDistFactory>>();
        }
    }
}
