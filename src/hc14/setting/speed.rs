// use at_commands::parser::CommandParser;

use crate::{conf::RESPONSE_SPEED, hc14::normal::format_converter};

/// 速率等级
#[derive(Debug, PartialEq, Eq)]
/// 无线速率等级：1-8，值越大，速率越高
pub enum Speed {
    /// 无线速率等级：1
    /// 1 字节 延迟约: 2.2s; 10 字节 延迟约: 2.9s
    /// 20 字节 延迟约: 3.6s; 40 字节 延迟约: 5.0s
    S1,
    /// 无线速率等级：2
    /// 1 字节，延迟约: 1.4s; 10 字节 延迟约: 1.8s;
    /// 20 字节，延迟约: 2.1s; 40 字节 延迟约: 2.8s
    S2,
    /// 无线速率等级：3
    /// 1 字节，延迟约: 0.8s; 10 字节，延迟约: 1.0s;
    /// 40 字节，延迟约: 1.7s; 发送80个字节，延迟约: 2.6s
    S3,
    /// 无线速率等级：4
    /// 1 字节，延迟约: 0.44s; 10 字节，延迟约: 0.6s;
    /// 40 字节，延迟约: 1.0s; 发送80个字节，延迟约: 1.53s
    S4,
    /// 无线速率等级：5
    /// 1 字节，延迟约: 0.3s; 10 字节，延迟约: 0.4s;
    /// 发送80个字节，延迟约: 1.0s; 发送160个字节，延迟约: 1.6s
    S5,
    /// 无线速率等级：6
    /// 1 字节，延迟约: 0.2s; 10 字节，延迟约: 0.26s;
    /// 发送80个字节，延迟约: 0.66s; 发送160个字节，延迟约: 1.1s
    S6,
    /// 无线速率等级：7
    /// 1 字节，延迟约: 0.15s; 10 字节，延迟约: 0.2s;
    /// 发送160个字节，延迟约: 0.7s; 发送250个字节，延迟约: 1s
    S7,
    /// 无线速率等级：8
    /// 1 字节，延迟约: 0.1s; 10 字节，延迟约: 0.13s;
    /// 发送160个字节，延迟约: 0.55s; 发送250个字节，延迟约: 0.7s
    S8,
    // /// 无线速率未找到
    // None,
}

impl Speed {
    /// 如果给定的通道有效，则构建一个新通道
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
    /// 获取该空中波特率的无线灵敏度（单位 dbm)
    /// 接收灵敏度每下降 6 dBm，通信距离会减少一半。
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
    /// S1：首次延迟约：5.0s，之后4.7s
    /// S2：首次延迟约：2.8s，之后2.6s
    /// S3：首次延迟约：2.6s，之后2.3s
    /// S4：首次延迟约：1.53s，之后1.3s
    /// S5：首次延迟约：1.6s，之后1.3s
    /// S6：首次延迟约：1.1s，之后0.8s
    /// S7：首次延迟约：1.0s，之后0.6s
    /// S8：首次延迟约：0.7s，之后0.3s
    pub fn get_max_字节_size(&self) -> usize {
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
impl TryFrom<&[u8]> for Speed {
    type Error = ();
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let result_speed = format_converter(value, &RESPONSE_SPEED);
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
