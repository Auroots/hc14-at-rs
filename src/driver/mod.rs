pub use at_commands::parser::CommandParser;
use core::marker::PhantomData;
use embedded_hal::{
    blocking::delay::DelayUs,
    digital::v2::OutputPin,
    serial::{Read, Write},
};
pub use nb::*;

/// AT配置模式(AT Configuration Mode)
pub mod configure;

/// 正常模式(Normal Mode)
pub mod normal;

/// 正常模式标记(Normal Mode Flags)
#[derive(Debug)]
pub struct Normal;

/// 配置模式标记(Configuration Mode Flags)
#[derive(Debug)]
pub struct Configuration;

/// Hc14 资源：串行端口、输出引脚和延迟。Hc14 Resources: serial ports, output pins, and delays.
#[derive(Debug, Clone, Copy)]
pub struct Hc14<S, P, D, M>
where
    S: Read<u8> + Write<u8>,
    P: OutputPin,
    D: DelayUs<u32>,
{
    serial: S,
    key_pin: P,
    delay: D,
    pub(crate) mode: PhantomData<M>,
}
