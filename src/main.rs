use clap::{Parser, Subcommand};

pub mod consts;
pub mod enums;
pub mod device;
pub mod utils;
pub mod structs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    list: bool,

    #[arg(short, long, default_value_t = false)]
    quiet: bool,

    #[arg(long, default_value_t = false)]
    pretend: bool,

    #[arg(short = 'm', long, default_value_t = false)]
    machine_readable: bool,

    #[arg(short = 'n', long, default_value_t = false)]
    min_value: bool,

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
    let devices: Vec<device::Device> = device::read_devices();
    let dev_: Option<&device::Device> = {
        match (&cli.class, &cli.device) {
            (Some(class), Some(dev)) => devices
                .iter()
                .find(|d| (d.get_id() == dev) & (d.get_class() == class)),
            (Some(class), None) => devices.iter().find(|d| d.get_class() == class),
            (None, Some(dev)) => devices.iter().find(|d| d.get_id() == dev),
            (None, None) => Some(&devices[0]),
        }
    };

    if !dev_.is_some() {
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
            } else {
                if !cli.machine_readable {
                    print!("{}", device)
                } else {
                    print!("{:+}", device)
                }
            }
        }
        Some(Commands::Get) | Some(Commands::G) => {
            println!("{}", device.get_curr_brightness())
        }
        Some(Commands::Max) | Some(Commands::M) => {
            println!("{}", device.get_max_brightness())
        }
        Some(Commands::Set { value }) | Some(Commands::S { value }) => {
            let parsed_val: structs::Value = structs::parse_value(&value);
            let new_dev = device::write_device(device, &parsed_val);
            match new_dev {
                Some(dev) => println!("{}", dev),
                _ => ()
            }
        }
    }
}
