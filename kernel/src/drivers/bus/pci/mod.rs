use alloc::vec::Vec;
use crate::cpu_ports::{port_input32, port_output32};

pub struct PciDevice {
    pub bus: u8,
    pub slot: u8,
    pub function: u8,
}
pub struct PciDeviceConfig {
    pub vendor_id: u16,
    pub device_id: u16,
    pub command: u16,
    pub status: u16,
    pub revision: u8,
    pub prog_if: u8,
    pub subclass: u8,
    pub class_code: u8,
    pub cache_line_size: u8,
    pub latency_timer: u8,
    pub header_type: u8,
    pub bist: u8,
    pub bar0: u32,
    pub bar1: u32,
}
impl PciDevice {
    pub(crate) fn get_config(&self) -> PciDeviceConfig {
        return PciDeviceConfig {
            vendor_id: self.read_config_u16(0),
            device_id: self.read_config_u16(2),
            command: self.read_config_u16(4),
            status: self.read_config_u16(6),
            revision: self.read_config_u8(8),
            prog_if: self.read_config_u8(9),
            subclass: self.read_config_u8(10),
            class_code: self.read_config_u8(11),
            cache_line_size: self.read_config_u8(12),
            latency_timer: self.read_config_u8(13),
            header_type: self.read_config_u8(14),
            bist: self.read_config_u8(15),
            bar0: self.read_config_u32(0x10),
            bar1: self.read_config_u32(0x14),
        };
    }
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
    pub fn read_config_u32(&self, offset: u8) -> u32 {
        let address: u32;
        let lbus = self.bus as u32;
        let lslot = self.slot as u32;
        let lfunc = self.function as u32;
        address = (lbus << 16) | (lslot << 11) | (lfunc << 8) | ((offset & 0xFC) as u32) | (0x80000000);
        port_output32(0xCF8, address);
        return port_input32(0xCFC);
    }
    pub fn read_config_u16(&self, offset: u8) -> u16 {
        let full = self.read_config_u32(offset);
        let offset_diff = (offset % 4) * 8;
        return ((full >> offset_diff) & 0xffff) as u16;
    }
    pub fn read_config_u8(&self, offset: u8) -> u8 {
        let full = self.read_config_u32(offset);
        let offset_diff = (offset % 4) * 8;
        return ((full >> offset_diff) & 0xff) as u8;
    }
}