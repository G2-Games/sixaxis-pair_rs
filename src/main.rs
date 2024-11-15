mod hidapi;

use crate::hidapi::{SixaxisApi, HidDevice};
use futures_lite::future::block_on;
use macaddr::MacAddr6;
use std::{env, error::Error, process::exit, str::FromStr};

const VENDOR: u16 = 0x054c;
const PRODUCT: u16 = 0x0268;
const CONTROLLER_MAC_REPORT_ID: u8 = 0xf2;
const CONNECTED_MAC_REPORT_ID: u8 = 0xf5;

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

    // Get the controller's MAC address
    let controller_address =
        block_on(device.get_feature_report(CONTROLLER_MAC_REPORT_ID, 17)).unwrap()[4..10].to_vec();
    println!("Controller MAC: {}", mac_to_string(&controller_address));
    println!("----");

    // Get the currently paired device
    let paired_device =
            &block_on(device.get_feature_report(CONNECTED_MAC_REPORT_ID, 8)).unwrap()[2..];

    if args.len() == 1 {
        // If no arguments were passed, display the currently paired address
        println!("Current Device: {}", mac_to_string(paired_device));
    } else if args.len() == 2 {
        // If mac address provided, set it, then retrieve the paired device
        let mac_buffer = set_pairing(&device, args[1].as_str()).unwrap();
        let paired_dev = &block_on(
            device.get_feature_report(CONNECTED_MAC_REPORT_ID, 8)
        ).unwrap()[2..];

        if mac_buffer.as_slice() == paired_dev {
            println!("New Device: {}", mac_to_string(paired_dev));
        } else {
            println!(
                "Setting failed, returned MAC Address is {}, expected {}",
                mac_to_string(paired_dev),
                mac_to_string(&mac_buffer),
            );
        }
    } else {
        println!("Usage:\n\n{} [mac]", args[0]);
    }
}

/// Set the new pairing of a SixAxis controller
fn set_pairing(device: &HidDevice, address: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buffer = vec![CONNECTED_MAC_REPORT_ID, 0x0];
    let address = match MacAddr6::from_str(address) {
        Ok(buf) => buf.as_bytes().to_vec(),
        Err(e) => {
            eprintln!("Failed to parse MAC address: {}", e);
            exit(1);
        },
    };
    buffer.extend_from_slice(&address);

    block_on(device.set_feature_report(CONNECTED_MAC_REPORT_ID, &buffer))?;

    Ok(address)
}

/// Turn a MAC address into a string
fn mac_to_string(buffer: &[u8]) -> String {
    let mut output = String::new();
    for (i, byte) in buffer.iter().enumerate() {
        output.push_str(&format!("{:02X?}", byte));
        if i < 5 {
            output.push(':');
        }
    }

    output
}
