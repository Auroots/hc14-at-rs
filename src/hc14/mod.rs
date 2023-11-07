pub use at_commands::parser::CommandParser;
use core::marker::PhantomData;
pub use embedded_hal::blocking::delay::DelayUs;

pub use embedded_hal::{
    digital::v2::OutputPin,
    serial::{Read, Write},
};
pub use nb::*;

/// AT配置模式
pub mod configure;

/// 正常模式
pub mod normal;

/// 设置
pub mod setting;

/// 正常模式标记
#[derive(Debug)]
pub struct Normal;

/// 配置模式标记
#[derive(Debug)]
pub struct Configuration;

/// Hc14 资源：串行端口、输出引脚和延迟。
#[derive(Debug, Clone, Copy)]
pub struct Hc14<S, P, D, M>
where
    S: Read<u8> + Write<u8>,
    P: OutputPin,
    D: DelayUs<u32>,
{
    serial: S,
    set_pin: P,
    delay: D,
    pub(crate) mode: PhantomData<M>,
}
