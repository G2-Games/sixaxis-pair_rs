mod hidapi;
mod sixaxis;

use macaddr::MacAddr6;
use owo_colors::{OwoColorize as _, Stream::Stdout};
use sixaxis::SixaxisApi;
use std::{env, path::PathBuf, process::exit, str::FromStr};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--version".into()) | args.contains(&"-V".into()) {
        println!("{}", version_string());
        exit(0);
    }
    if args.contains(&"--help".into()) | args.contains(&"-h".into()) {
        print_help(&args[0]);
        exit(0);
    }

    // Open a sixaxis device
    let mut device = match SixaxisApi::open() {
        Ok(d) => d,
        Err(e) => print_err_exit(&e.to_string()),
    };

    if args.len() == 1 {
        // If no arguments were passed, display the currently paired address
        let paired_device = match device.paired_mac() {
            Ok(m) => m,
            Err(e) => print_err_exit(&e.to_string()),
        };

        let controller_address = match device.mac() {
            Ok(m) => m,
            Err(e) => print_err_exit(&e.to_string()),
        };

        println!("Controller MAC: {}", controller_address);
        println!("Current Device: {}", paired_device);
    } else if args.len() == 2 {
        // If mac address provided, set it, then retrieve the paired device
        let replacement_addr = match MacAddr6::from_str(&args[1]) {
            Ok(m) => m,
            Err(e) => print_err_exit(&format!(
                "Provided MAC address invalid: {} <- {}",
                args[1].if_supports_color(Stdout, |t| t.bright_yellow()),
                e
            )),
        };
        device.set_paired_mac(replacement_addr).unwrap();
        let paired_addr = device.paired_mac().unwrap();

        if replacement_addr == paired_addr {
            println!(
                "New Device: {}",
                paired_addr.if_supports_color(Stdout, |t| t.bright_green())
            );
        } else {
            print_err_exit(&format!(
                "Setting failed, returned MAC Address is {}, expected {}",
                paired_addr.if_supports_color(Stdout, |t| t.bright_yellow()),
                replacement_addr.if_supports_color(Stdout, |t| t.green()),
            ));
        }
    } else {
        print_help(&args[0])
    }
}

fn version_string() -> String {
    format!("SixAxis pair tool v{}", env!("CARGO_PKG_VERSION"))
}

fn print_help(bin_path: &str) {
    let path = PathBuf::from(bin_path);
    let bin_name = path.file_name().unwrap_or_default();
    println!("{}", version_string());
    println!("Usage:\n\n{} <MAC>", bin_name.to_string_lossy());
}

fn print_err_exit(message: &str) -> ! {
    eprintln!(
        "{} {}",
        "Error:".if_supports_color(Stdout, |t| t.red()),
        message,
    );

    exit(1)
}
