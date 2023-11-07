use crate::{conf::RESPONSE_SPEED, driver::normal::format_converter};

#[derive(Debug, PartialEq, Eq)]
/// 无线速率等级：1-8，值越大，速率越高
///
/// Wireless rate class: 1-8, the higher the value, the higher the rate
pub enum Speed {
    /// 无线速率等级：1 <br/>
    /// Wireless rate class: 1
    S1,
    /// 无线速率等级：2 <br/>
    /// Wireless rate class: 2
    S2,
    /// 无线速率等级：3 <br/>
    /// Wireless rate class: 3
    S3,
    /// 无线速率等级：4 <br/>
    /// Wireless rate class: 4
    S4,
    /// 无线速率等级：5 <br/>
    /// Wireless rate class: 5
    S5,
    /// 无线速率等级：6 <br/>
    /// Wireless rate class: 6
    S6,
    /// 无线速率等级：7 <br/>
    /// Wireless rate class: 7
    S7,
    /// 无线速率等级：8 <br/>
    /// Wireless rate class: 8
    S8,
}

impl Speed {
    /// 如果给定的通道有效，则构建一个新通道 <br/>
    /// If the given channel is valid, construct a new channel
    pub fn new(ch: u8) -> Option<Self> {
        match ch {
            1 => Some(Speed::S1),
            2 => Some(Speed::S2),
            3 => Some(Speed::S3),
            4 => Some(Speed::S4),
            5 => Some(Speed::S5),
            6 => Some(Speed::S6),
            7 => Some(Speed::S7),
            8 => Some(Speed::S8),
            _ => None,
        }
    }
    /// 获取该空中波特率的无线灵敏度（单位 dbm)，接收灵敏度每下降 6 dBm，通信距离会减少一半 <br/>
    /// Obtain the radio sensitivity (in dbm) for this air baud rate.
    /// For every 6 dBm drop in receive sensitivity, the communication range is halved.
    pub fn get_wireless_sensitivity_dbm(&self) -> f32 {
        match self {
            Speed::S1 => -140.0,
            Speed::S2 => -137.5,
            Speed::S3 => -135.0,
            Speed::S4 => -132.5,
            Speed::S5 => -130.0,
            Speed::S6 => -127.5,
            Speed::S7 => -124.5,
            Speed::S8 => -121.5,
        }
    }
    /// 获取当前速率等级单个数据包的最大容量
    ///
    /// Get the maximum capacity of a single packet for the current rate class.
    ///
    /// S1：首次延迟约：5.0s，之后4.7s，First delay of approximately 5.0s, after that 4.7s <br/>
    /// S2：首次延迟约：2.8s，之后2.6s <br/>
    /// S3：首次延迟约：2.6s，之后2.3s <br/>
    /// S4：首次延迟约：1.53s，之后1.3s <br/>
    /// S5：首次延迟约：1.6s，之后1.3s <br/>
    /// S6：首次延迟约：1.1s，之后0.8s <br/>
    /// S7：首次延迟约：1.0s，之后0.6s <br/>
    /// S8：首次延迟约：0.7s，之后0.3s <br/>
    pub fn get_max_bytes_size(&self) -> usize {
        match self {
            Speed::S1 | Speed::S2 => 40,
            Speed::S3 | Speed::S4 => 80,
            Speed::S5 | Speed::S6 => 160,
            Speed::S7 | Speed::S8 => 250,
        }
    }
}

impl Default for Speed {
    fn default() -> Self {
        Self::S3
    }
}

/// 速率响应分析器
///
/// Rate Response Analyzer
impl TryFrom<&[u8]> for Speed {
    type Error = ();
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let result_speed: Result<i32, nb::Error<&str>> = format_converter(value, &RESPONSE_SPEED);
        match result_speed {
            Ok(1) => Ok(Speed::S1),
            Ok(2) => Ok(Speed::S2),
            Ok(3) => Ok(Speed::S3),
            Ok(4) => Ok(Speed::S4),
            Ok(5) => Ok(Speed::S5),
            Ok(6) => Ok(Speed::S6),
            Ok(7) => Ok(Speed::S7),
            Ok(8) => Ok(Speed::S8),
            _ => core::prelude::v1::Err(()),
        }
    }
}
