#![allow(non_camel_case_types)]

pub const BMA421_DEVICE_ID: u8 = 0x11;
pub const BMA421_ALT_DEVICE_ID: u8 = 0x12; //TODO this is a guess

pub const BMA423_DEFAULT_DEVICE_ID: u8 = 0x18;
pub const BMA423_ALT_DEVICE_ID: u8 = 0x19;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Register {
    CHIP_ID = 0x00,
    ERROR = 0x02,
    STATUS = 0x03,
    DATA_0 = 0x0A,
    DATA_8 = 0x12,

    SENSORTIME_0 = 0x18,
    INT_STAT_0 = 0x1C,
    INT_STAT_1 = 0x1D,

    TEMPERATURE_ = 0x22,

    CMD = 0x7E,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Command {
    SOFT_RESET = 0xB6,
}
