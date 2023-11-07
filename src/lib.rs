//! HC-14 driver
//! This driver implements normal, config and sleep functionality of the hc12 module.
//! 
//! When the Key pin is pulled low, you can configure the module using AT commands. 
//! This driver program receives output pins, a serial port, and a delay time from `embedded-hal` and provides a convenient interface to interact with the HC-14 module.
//! 
//! The HC-14 can operate in two modes: Normal mode and AT configuration mode.
//! 
//! For more details, refer to the official documentation available [here](https://www.hc01.com/downloads).
//! 
//! Building upon the [barafael/hc12-at-rs](https://github.com/barafael/hc12-at-rs) project, 
//! this driver has been restructured for the HC-14 module. It's worth noting that, compared to the HC-12, 
//! the HC-14 module has additional features. However, the documentation only provides an overview of the module, 
//! and there are no example programs. As of now, I have tested this driver using an STM32F103 with default settings. 
//! In an urban environment, with the transmitting module placed on the 7th floor, the communication range is approximately 600-700 meters.
//! 
//! 当 Key 引脚被拉低时，可以使用 AT 指令对该模块进行配置。该驱动程序从 `embedded-hal` 中获取输出引脚、
//! 串行端口和延迟时间，提供了一个与 hc14 模块交互的简便接口。
//! 
//! HC-14 的运行状态：正常模式、AT配置配置。
//! 
//! 更多详情，请参阅此处提供的[官方文档](https://www.hc01.com/downloads)，
//! 
//! 在[barafael/hc12-at-rs](https://github.com/barafael/hc12-at-rs)基础上对hc14模块进行重构，不得不说，相比hc12，
//! hc14模块被很多功能有没有了，文档对模块的介绍也就停留在介绍而已，例程也没有，我暂时只使用了STM32F103进行了测试，使用默认设置，
//! 在城市中，发送模块放在7楼的情况下，通信距离大约：600-700米的。
//! 
//! # Example
//!
//!```rust
//!let hc14 = Hc14::new(serial, key, delay).unwrap();
//!
//! // Reset mode parameters
//! let mut hc14_configure = hc14.into_configuration_mode().unwrap();
//! hc14_configure.reset_settings();
//! 
//! // Retrieve Hc14 parameters
//! hc14_configure.get_parameters().unwrap();
//! 
//! // Set channel
//! let mut buffer = [0u8; 32];
//! hc14_configure.wirte_set_channel(28, &mut buffer);
//! 
//! // Switch to normal mode
//! let mut hc14_normal = hc14_configure.into_normal_mode().unwrap();
//! 
//! // Send buffer
//! let buf = b"hc14";
//! hc14_normal.send_buffer(buf).unwrap();
//! 
//! // Send a string
//! let buf_str = "hc14";
//! hc14_normal.send_string(buf_str);
//! ```
//! 


#![no_std]
#![deny(unsafe_code)]
#![deny(missing_docs)]

/// HC-14 驱动程序(Driver)
pub mod driver;

/// HC-14 指令集(Instruction Set)
pub mod conf;

/// HC-14 Settings
pub mod setting;

/// Crate 错误(Error)
#[derive(Debug)]
pub enum Error {
    /// Read error
    Read,
    /// Write error
    Write,
    /// 无效波特率(invalid baud rate)
    InvalidBaudRate,
    /// 无效信道(invalid channel)
    InvalidChannel,
}
