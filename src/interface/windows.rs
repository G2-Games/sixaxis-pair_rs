use std::error::Error;

use hidapi::{HidApi, HidDevice};

pub struct SixaxisApi;

pub struct SixaxisDevice {
    device: HidDevice,
}

impl SixaxisApi {
    pub fn new() -> Self {
        Self {}
    }

    pub fn open(&self, vendor_id: u16, product_id: u16) -> Result<SixaxisDevice, Box<dyn Error>> {
        let hidapi = HidApi::new()?;

        let device = hidapi.open(vendor_id, product_id)?;

        return Ok(SixaxisDevice {
            device,
        })
    }
}

impl SixaxisDevice {
    pub fn get_feature_report(&self, report_number: u8) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = [0u8; 8];
        buffer[0] = report_number;

        self.device.get_feature_report(&mut buffer)?;
        let result = &buffer[2..];

        Ok(result.into())
    }

    pub fn set_feature_report(&self, report_number: u8, out_buffer: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        let mut buffer = vec![report_number, 0x0];
        buffer.append(&mut out_buffer.clone());

        self.device.send_feature_report(&buffer)?;

        Ok(())
    }
}
