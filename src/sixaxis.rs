use std::error::Error;

use futures_lite::future::block_on;
use macaddr::MacAddr6;
use nusb::list_devices;

use crate::hidapi::HidDevice;

const VENDOR: u16 = 0x054C;
const PRODUCT: u16 = 0x0268;
const CONTROLLER_MAC_REPORT_ID: u8 = 0xF2;
const CONNECTED_MAC_REPORT_ID: u8 = 0xF5;

pub struct SixaxisApi {
    device: HidDevice,
}

impl SixaxisApi {
    pub fn open() -> Result<SixaxisApi, Box<dyn Error>> {
        let device = list_devices()
            .unwrap()
            .find(|dev| dev.vendor_id() == VENDOR && dev.product_id() == PRODUCT)
            .ok_or("Unable to find SixAxis device!")?
            .open()?;

        let interface = device.detach_and_claim_interface(0)?;

        Ok(SixaxisApi {
            device: HidDevice::new(interface),
        })
    }

    /// Get the MAC address of the controller itself
    pub fn mac(&mut self) -> Result<MacAddr6, Box<dyn Error>> {
        let response = block_on(self.device.get_feature_report(CONTROLLER_MAC_REPORT_ID, 17))?;
        let mac_bytes: [u8; 6] = response[4..10].try_into().unwrap();

        Ok(MacAddr6::from(mac_bytes))
    }

    /// Get the MAC address of the device paired to the controller
    pub fn paired_mac(&mut self) -> Result<MacAddr6, Box<dyn Error>> {
        let response = block_on(self.device.get_feature_report(CONNECTED_MAC_REPORT_ID, 8))?;
        let mac_bytes: [u8; 6] = response[2..].try_into().unwrap();

        Ok(MacAddr6::from(mac_bytes))
    }

    /// Set the MAC address of the device paired to the controller
    pub fn set_paired_mac(&mut self, address: MacAddr6) -> Result<(), Box<dyn Error>> {
        let mut buffer = vec![CONNECTED_MAC_REPORT_ID, 0x0];
        buffer.extend_from_slice(address.as_bytes());

        block_on(self.device.set_feature_report(CONNECTED_MAC_REPORT_ID, &buffer))?;

        Ok(())
    }
}
