use std::error::Error;

use nusb::{
    Interface,
    transfer::{ControlIn, ControlType, Recipient, ControlOut}
};

pub struct HidDevice {
    interface: Interface,
}

impl HidDevice {
    pub fn new(interface: Interface) -> Self {
        Self { interface  }
    }

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
