mod hidapi;

use crate::hidapi::{SixaxisApi, SixaxisDevice};
use macaddr::MacAddr6;
use std::{env, error::Error, process::exit, str::FromStr};

const VENDOR: u16 = 0x054c;
const PRODUCT: u16 = 0x0268;
const MAC_REPORT_ID: u8 = 0xf5;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Get HIDAPI context
    let api = SixaxisApi::new();

    // Try to get the first sixaxis controller
    let device = match api.open(VENDOR, PRODUCT) {
        Ok(device) => device,
        Err(_) => {
            eprintln!("No SixAxis device found!");
            exit(1);
        }
    };

    if args.len() == 1 {
        // If no arguments, display the currently paired address
        let paired_dev = pairing(device).unwrap();

        print!("Current Device: ");
        for (i, byte) in paired_dev.iter().enumerate() {
            print!("{:02X?}", byte);
            if i < 5 {
                print!(":")
            }
        }
        println!();
    } else if args.len() == 2 {
        // If mac address provided, set it
        set_pairing(device, args[1].as_str()).unwrap();
        println!("New Device: {}", args[1]);
    } else {
        println!("Usage:\n\n{} [mac]", args[0]);
    }
}

/// Get the current pairing of a SixAxis controller
fn pairing(device: SixaxisDevice) -> Result<Box<[u8]>, Box<dyn Error>> {
    let result = device.get_feature_report(MAC_REPORT_ID)?;

    Ok(result.into())
}

/// Set the new pairing of a SixAxis controller
fn set_pairing(device: SixaxisDevice, address: &str) -> Result<(), Box<dyn Error>> {
    let mut buffer = vec![MAC_REPORT_ID, 0x0];
    let mut address = MacAddr6::from_str(address)?.as_bytes().to_vec();
    buffer.append(&mut address);

    device.set_feature_report(MAC_REPORT_ID, &buffer)?;

    Ok(())
}
