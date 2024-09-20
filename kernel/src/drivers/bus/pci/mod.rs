use alloc::vec::Vec;
use crate::cpu_ports::{port_input32, port_output32};

pub struct PciDevice {
    pub bus: u8,
    pub slot: u8,
    pub function: u8,
}
impl PciDevice {
    pub fn enumerate_all() -> Vec<PciDevice> {
        let mut devices = Vec::new();
        for bus in 0..256 {
            for slot in 0..32 {
                let device = PciDevice {
                    bus: bus as u8,
                    slot: slot as u8,
                    function: 0,
                };
                if device.is_valid() {
                    devices.push(device);
                }
            }
        }
        return devices;
    }
    pub fn is_valid(&self) -> bool {
        let vendor = self.read_config_u16(0);
        vendor != 0xFFFF
    }
    pub fn config_read_u32(&self, offset: u8) -> u32 {
        let address: u32;
        let lbus = self.bus as u32;
        let lslot = self.slot as u32;
        let lfunc = self.function as u32;
        address = (lbus << 16) | (lslot << 11) | (lfunc << 8) | ((offset & 0xFC) as u32) | (0x80000000);
        port_output32(0xCF8, address);
        return port_input32(0xCFC);
    }
    pub fn read_config_u16(&self, offset: u8) -> u16 {
        let full = self.config_read_u32(offset);
        let offset_diff = (offset % 4) * 8;
        return ((full >> offset_diff) & 0xffff) as u16;
    }
    pub fn read_config_u8(&self, offset: u8) -> u8 {
        let full = self.config_read_u32(offset);
        let offset_diff = (offset % 4) * 8;
        return ((full >> offset_diff) & 0xff) as u8;
    }

}