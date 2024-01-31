use std::error::Error;

use futures_lite::future::block_on;
use nusb::list_devices;
use nusb::{Device, DeviceInfo, Interface, InterfaceInfo};
use nusb::transfer::{ControlType, ControlIn, Recipient, Control};

pub struct HidApi {

}

pub struct HidDevice {
    device: Device,
    interface: Interface,
}

impl HidApi {
    pub fn new() -> Self {
        Self {}
    }

    pub fn open(&self, vendor_id: u16, product_id: u16) -> Result<HidDevice, Box<dyn Error>> {
        let device;
        for dev_info in list_devices()? {
            if dev_info.vendor_id() == vendor_id
                && dev_info.product_id() == product_id
            {
                let temp_dev = dev_info.open()?;

                device = HidDevice::new(temp_dev, dev_info)?;
                return Ok(device)
            }
        }

        Err("Could not find requested device".into())
    }
}

impl HidDevice {
    fn new(device: Device, device_info: DeviceInfo) -> Result<Self, Box<dyn Error>> {
        let interface;
        for int_info in device_info.interfaces() {
            if int_info.class() == 3 { // Ensure it's an HID interface
                interface = device.detach_and_claim_interface(0)?;

                return Ok(Self {
                    device,
                    interface
                })
            }
        }

        Err("Could not find valid interface".into())
    }

    pub fn get_feature_report(&self, report_number: u8) -> Result<Vec<u8>, Box<dyn Error>> {
        let result = block_on(self.interface.control_in(
            ControlIn {
                control_type: ControlType::Class,
                recipient: Recipient::Interface,
                request: 0x01,
                value: (3 << 8) | report_number as u16,
                index: 0,
                length: 8
            }
        )).into_result()?;

        dbg!(result.clone());

        Ok(result)
    }
}
