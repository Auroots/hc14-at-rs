//! Hc14 driver
//! This driver implements normal, config and sleep functionality of the hc12 module.
#![no_std]
#![deny(unsafe_code)]
#![deny(missing_docs)]

/// Hc14 驱动程序
pub mod hc14;

/// Hc14 指令集
pub mod conf;

/// Crate 错误
#[derive(Debug)]
pub enum Error {
    /// 读取错误
    Read,
    /// 写入错误
    Write,
    /// 波特率无效
    InvalidBaudRate,
    /// 无效信道
    InvalidChannel,
}
