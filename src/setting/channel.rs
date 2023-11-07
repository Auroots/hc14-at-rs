use crate::{conf::RESPONSE_CHANNEL, driver::normal::format_converter};
use core::convert::TryFrom;
use num_derive::{FromPrimitive, ToPrimitive};

/// 通信信道
#[derive(Debug, ToPrimitive, FromPrimitive, PartialEq, Eq)]
pub struct Channel(u8);

impl Channel {
    /// 获取以 MHz 为单位获取信道频率
    ///
    /// Get channel frequency in MHz.
    pub fn get_freq_mhz(&self) -> Result<f32, &'static str> {
        let channel_mhz: [f32; 50] = [
            415.09, 415.70, 416.31, 416.92, 417.53, 418.14, 419.36, 420.58, 421.19, 421.80, 422.41,
            423.63, 424.24, 424.85, 425.46, 426.07, 426.68, 427.29, 427.90, 429.12, 429.73, 430.34,
            430.95, 431.56, 432.17, 432.78, 433.39, 434.00, 434.61, 435.22, 435.83, 436.44, 437.05,
            437.66, 438.27, 438.88, 440.10, 440.71, 441.32, 441.93, 442.54, 443.15, 443.76, 444.37,
            445.59, 446.20, 446.81, 447.42, 448.64, 449.86,
        ];
        if self.0 >= 51 || self.0 == 0 {
            Err("Invalid index, channel range: 1-50")
        } else {
            Ok(channel_mhz[self.0 as usize - 1])
        }
    }
}

impl Default for Channel {
    /// 默认信道为：28(434.00 MHz)
    ///
    /// Default channel is: 28 (434.00 MHz)
    fn default() -> Self {
        Channel(28)
    }
}

impl From<u8> for Channel {
    /// 如果给定的通道有效，则构建一个新通道
    fn from(value: u8) -> Self {
        match value {
            ch if !(1..=50).contains(&ch) => panic!("Invalid channel"),
            n => Channel(n),
        }
    }
}

impl TryFrom<&[u8]> for Channel {
    type Error = ();
    /// 将接收到的信道响应，返回为`Channel` 类型
    ///
    /// Returns the received channel response as a `Channel` type.
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let channel = format_converter(value, &RESPONSE_CHANNEL).unwrap() as u8;
        match channel {
            0 => Err(()),
            ch if !(1..=50).contains(&ch) => Err(()),
            n => Ok(Channel(n)),
        }
    }
}
