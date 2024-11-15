use std::error::Error;

use nusb::{
    Interface,
    list_devices,
    transfer::{ControlIn, ControlType, Recipient, ControlOut}
};

pub struct SixaxisApi;

pub struct HidDevice {
    interface: Interface,
}

impl SixaxisApi {
    pub fn new() -> Self {
        Self {}
    }

    pub fn open(&self, vendor_id: u16, product_id: u16) -> Result<HidDevice, Box<dyn Error>> {
        let device = list_devices()
            .unwrap()
            .find(|dev| dev.vendor_id() == vendor_id && dev.product_id() == product_id)
            .expect("Unable to find SixAxis device!")
            .open()?;

        let interface = device.detach_and_claim_interface(0)?;

        Ok(HidDevice {
            interface,
        })
    }
}

impl HidDevice {
    /// Get a feature report from an interface
    pub async fn get_feature_report(&self, report_number: u8, length: u16) -> Result<Vec<u8>, Box<dyn Error>> {
        let result = self.interface.control_in(
            ControlIn {
                control_type: ControlType::Class,
                recipient: Recipient::Interface,
                request: 1,
                value: 0x300 | report_number as u16,
                index: 0,
                length
            }
        ).await.into_result()?;

        Ok(result)
    }

    /// Set a feature report on an interface
    pub async fn set_feature_report(&self, report_number: u8, out_buffer: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        self.interface.control_out(
            ControlOut {
                control_type: ControlType::Class,
                recipient: Recipient::Interface,
                request: 9,
                value: 0x300 | report_number as u16,
                index: 0,
                data: out_buffer,
            }
        ).await.into_result()?;

        Ok(())
    }
}
