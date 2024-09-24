use clap::{Parser, Subcommand};

pub mod consts;
pub mod device;
pub mod enums;
pub mod structs;
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

    #[arg(short = 'n', long, default_value_t = 1)]
    min_value: u64,

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

    if dev_.is_none() {
        println!(
            "No device found for class {} and device name {}",
            cli.class.unwrap_or(String::from("''")),
            cli.device.unwrap_or(String::from("''"))
        );
        std::process::exit(1)
    }
    let device: &device::Device = dev_.unwrap();

    let min_value = structs::Value {
        val: cli.min_value,
        v_type: enums::ValueType::ABSOLUTE,
        d_type: enums::DeltaType::DIRECT,
        sign: enums::Sign::PLUS,
    };

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
            let parsed_val: structs::Value = structs::parse_value(value);
            if device.get_max_brightness() < min_value.val {
                println!("Invalid minimum value {}", min_value.val)
            }
            let new_dev = device::write_device(device, &parsed_val, &min_value, cli.pretend);
            if let Some(dev) = new_dev {
                println!("{}", dev)
            }
        }
    }
}
