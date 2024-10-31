use std::{process::exit, str::FromStr};

use clap::{Parser, Subcommand};
use enums::ValueUpdate;

pub mod consts;
pub mod device;
pub mod enums;
pub mod utils;

#[derive(Parser, Debug)]
#[command(name = "Brightctl", author, version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    list: bool,

    #[arg(long, default_value_t = false)]
    pretend: bool,

    #[arg(short = 'm', long, default_value_t = false)]
    machine_readable: bool,

    #[arg(short = 'n', long, default_value = "1", help = "N{%}{+-}")]
    min_value: String,

    #[arg(short, long, global = true)]
    device: Option<String>,

    #[arg(short, long, global = true)]
    class: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    I,
    Info,
    G,
    Get,
    M,
    Max,
    S { value: String },
    Set { value: String },
}

fn main() {
    let cli = Cli::parse();
    let devices: Vec<device::Device> = device::read_devices(&cli.class, &cli.device);
    let dev_: Option<&device::Device> = devices.first();
    let min_value = match ValueUpdate::from_str(&cli.min_value) {
        Ok(update) => update,
        Err(err) => {
            println!("Error {}", err);
            exit(1);
        }
    };

    if dev_.is_none() {
        println!(
            "No device found for class {} and device name {}",
            cli.class.unwrap_or(String::from("''")),
            cli.device.unwrap_or(String::from("''"))
        );
        std::process::exit(1)
    }
    let device: &device::Device = dev_.unwrap();

    match &cli.command {
        Some(Commands::Info) | Some(Commands::I) | None => {
            if cli.list {
                for device in devices {
                    if !cli.machine_readable {
                        print!("{}", device)
                    } else {
                        print!("{:+}", device)
                    }
                }
            } else if !cli.machine_readable {
                print!("{}", device)
            } else {
                print!("{:+}", device)
            }
        }
        Some(Commands::Get) | Some(Commands::G) => {
            println!("{}", device.get_curr_brightness())
        }
        Some(Commands::Max) | Some(Commands::M) => {
            println!("{}", device.get_max_brightness())
        }
        Some(Commands::Set { value }) | Some(Commands::S { value }) => {
            let parsed_val_update = match ValueUpdate::from_str(value) {
                Ok(update) => update,
                Err(err) => {
                    println!("Error {}", err);
                    exit(1);
                }
            };

            let computed_min_value = device.compute_min_value(&min_value);
            if device.get_max_brightness() < computed_min_value {
                println!("Invalid minimum value {}", cli.min_value)
            }
            let new_dev =
                device::write_device(device, &parsed_val_update, computed_min_value, cli.pretend);
            if let Some(dev) = new_dev {
                println!("{}", dev)
            }
        }
    }
}
