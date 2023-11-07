use crate::{conf::RESPONSE_POWER, driver::normal::format_converter};
use core::convert::TryFrom;
use num_derive::{FromPrimitive, ToPrimitive};

/// 无线发射功率，单位: dbm
///
/// Wireless Transmit Power in dbm
#[derive(Debug, ToPrimitive, FromPrimitive, PartialEq, Eq)]
pub struct TransmissionPower(u8);

impl TransmissionPower {
    /// 构建一个新的 TransmissionPower
    ///
    /// Build a new TransmissionPower
    pub fn new(dbm: u8) -> Option<Self> {
        match dbm {
            dbm if dbm <= 5 || dbm >= 21 => None,
            _ => Some(TransmissionPower(dbm)),
        }
    }
    /// 获取以 dbm 为单位的传输功率
    ///
    /// Getting Transmission Power in dbm
    pub fn get_power_dbm(&self) -> u8 {
        self.0
    }
}

impl Default for TransmissionPower {
    fn default() -> Self {
        TransmissionPower(20)
    }
}

impl TryFrom<&[u8]> for TransmissionPower {
    type Error = ();
    /// 对获取到无线发射功率进行格式化匹配
    ///
    /// Format matching of acquired wireless transmit power
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let dbm = format_converter(value, &RESPONSE_POWER).unwrap() as u8;

        match TransmissionPower::new(dbm) {
            Some(v) => Ok(TransmissionPower(v.get_power_dbm())),
            None => Err(()),
        }
    }
}
