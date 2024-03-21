use std::error::Error;

use futures_lite::future::block_on;
use nusb::{Interface, list_devices, transfer::{ControlIn, ControlType, Recipient, ControlOut}};

pub struct SixaxisApi;

pub struct SixaxisDevice {
    interface: Interface,
}

impl SixaxisApi {
    pub fn new() -> Self {
        Self {}
    }

    pub fn open(&self, vendor_id: u16, product_id: u16) -> Result<SixaxisDevice, Box<dyn Error>> {
        let device = list_devices()
            .unwrap()
            .find(|dev| dev.vendor_id() == vendor_id && dev.product_id() == product_id)
            .expect("Unable to find SixAxis device!")
            .open()?;

        let interface = device.detach_and_claim_interface(0)?;

        Ok(SixaxisDevice {
            interface,
        })
    }
}

impl SixaxisDevice {
    pub fn get_feature_report(&self, report_number: u8) -> Result<Vec<u8>, Box<dyn Error>> {
        let result = block_on(self.interface.control_in(
            ControlIn {
                control_type: ControlType::Class,
                recipient: Recipient::Interface,
                request: 1,
                value: 0x300 | report_number as u16,
                index: 0,
                length: 8
            }
        )).into_result()?;

        let result = result[2..].to_vec();

        Ok(result)
    }

    pub fn set_feature_report(&self, report_number: u8, out_buffer: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        block_on(self.interface.control_out(
            ControlOut {
                control_type: ControlType::Class,
                recipient: Recipient::Interface,
                request: 9,
                value: 0x300 | report_number as u16,
                index: 0,
                data: out_buffer,
            }
        )).into_result()?;

        Ok(())
    }
}
