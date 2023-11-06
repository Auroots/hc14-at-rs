//! 关于AT+RX指令的数据结构

use super::{baudrate::BaudRate, channel::Channel, power::TransmissionPower, speed::Speed};

/// 所有 hc12 参数
#[derive(Debug, PartialEq, Eq)]
pub struct Parameters {
    /// 波特率
    pub baud: BaudRate,
    /// 通信信道
    pub channel: Channel,
    /// 传输功率
    pub power: TransmissionPower,
    /// 工作模式
    pub speed: Speed,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            baud: BaudRate::default(),
            channel: Channel::default(),
            power: TransmissionPower::default(),
            speed: Speed::default(),
        }
    }
}
