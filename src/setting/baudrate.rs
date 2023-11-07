use super::{parameters::Parameters, speed::Speed};
use crate::{conf::RESPONSE_BAUD, driver::normal::format_converter, Error};
use core::convert::TryFrom;
use core::result::Result::*;
use num_derive::{FromPrimitive, ToPrimitive};

/// HC-14 的波特率
/// Baud rate of HC-14
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum BaudRate {
    /// 1200 波特/秒, 1200 bauds per second
    Bps1200 = 1200,
    /// 2400 波特/秒, 2400 bauds per second
    Bps2400 = 2400,
    /// 4800 波特/秒, 4800 bauds per second
    Bps4800 = 4800,
    /// 9600 波特/秒, 9600 bauds per second
    Bps9600 = 9600,
    /// 19200 波特/秒, 19200 bauds per second
    Bps19200 = 19200,
    /// 38400 波特/秒, 38400 bauds per second
    Bps38400 = 38400,
    /// 57600 波特/秒, 57600 bauds per second
    Bps57600 = 57600,
    /// 115200 波特/秒, 115200 bauds per second
    Bps115200 = 115200,
}

/// 空中波特率, Baud rate in the air
#[derive(Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum AirBaudRate {
    /// S4模式下，500 波特/秒, 5000 bauds per second
    Bps500 = 500,
    /// 5000 波特/秒, 5000 bauds per second
    Bps5000 = 5000,
    /// 15000 波特/秒, 15000 bauds per second
    Bps15000 = 15000,
    /// 58000 波特/秒, 58000 bauds per second
    Bps58000 = 58000,
    /// 236000 波特/秒, 236000 bauds per second
    Bps236000 = 236000,
    /// 250000 波特/秒, 250000 bauds per second
    Bps250000 = 250000,
}

impl Default for BaudRate {
    /// 默认波特率:9600
    fn default() -> Self {
        BaudRate::Bps9600
    }
}

impl From<u32> for BaudRate {
    /// 将从数值匹配对应波特率，请确保你输入的波特率是模块支持的，否则会`panic`
    ///
    /// will match the corresponding baud rate from the value,
    /// please make sure the baud rate you enter is supported by the module,
    /// otherwise it will be `panic`.
    fn from(value: u32) -> Self {
        match value {
            1200 => BaudRate::Bps1200,
            2400 => BaudRate::Bps2400,
            4800 => BaudRate::Bps4800,
            9600 => BaudRate::Bps9600,
            19200 => BaudRate::Bps19200,
            38400 => BaudRate::Bps38400,
            57600 => BaudRate::Bps57600,
            115200 => BaudRate::Bps115200,
            _ => panic!("Invalid BaudRate"),
        }
    }
}
impl TryFrom<&[u8]> for BaudRate {
    type Error = ();
    /// 将接收到的波特率响应，返回为`BaudRate` 类型
    ///
    ///  Returns the received baud rate response as a `BaudRate` type.
    /// ```rust
    /// let buffer = b"OK+B:9600\r\n";
    /// assert_eq!(
    ///     BaudRate::try_from(buffer as &[u8]).unwrap(),
    ///     BaudRate::Bps9600
    ///  );
    /// ````
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match format_converter(value, &RESPONSE_BAUD) {
            Ok(1200) => Ok(BaudRate::Bps1200),
            Ok(2400) => Ok(BaudRate::Bps2400),
            Ok(4800) => Ok(BaudRate::Bps4800),
            Ok(9600) => Ok(BaudRate::Bps9600),
            Ok(19200) => Ok(BaudRate::Bps19200),
            Ok(38400) => Ok(BaudRate::Bps38400),
            Ok(57600) => Ok(BaudRate::Bps57600),
            Ok(115200) => Ok(BaudRate::Bps115200),
            _ => Err(()),
        }
    }
}

impl Parameters {
    /// 设置参数的波特率，官方没有详细说明所有模式的波特率参数
    ///
    /// Set the baud rate of the parameter,
    /// the official baud rate parameter for all modes is not specified in detail
    pub fn set_baud(&mut self, rate: BaudRate) -> Result<(), Error> {
        match self.speed {
            Speed::S1 => {
                self.baud = rate;
                Ok(())
            }
            Speed::S2 => match rate {
                BaudRate::Bps1200 | BaudRate::Bps2400 | BaudRate::Bps4800 => {
                    self.baud = rate;
                    Ok(())
                }
                _ => Err(Error::InvalidBaudRate),
            },
            Speed::S3 => match rate {
                BaudRate::Bps2400 | BaudRate::Bps4800 | BaudRate::Bps9600 => {
                    self.baud = rate;
                    Ok(())
                }
                _ => Err(Error::InvalidBaudRate),
            },
            Speed::S4 => match rate {
                BaudRate::Bps1200 => {
                    self.baud = rate;
                    Ok(())
                }
                _ => Err(Error::InvalidBaudRate),
            },
            Speed::S5 => match rate {
                BaudRate::Bps4800 | BaudRate::Bps9600 | BaudRate::Bps19200 => {
                    self.baud = rate;
                    Ok(())
                }
                _ => Err(Error::InvalidBaudRate),
            },
            Speed::S6 => match rate {
                BaudRate::Bps9600 | BaudRate::Bps19200 | BaudRate::Bps38400 => {
                    self.baud = rate;
                    Ok(())
                }
                _ => Err(Error::InvalidBaudRate),
            },
            Speed::S7 => match rate {
                BaudRate::Bps19200 | BaudRate::Bps38400 | BaudRate::Bps57600 => {
                    self.baud = rate;
                    Ok(())
                }
                _ => Err(Error::InvalidBaudRate),
            },
            Speed::S8 => match rate {
                BaudRate::Bps38400 | BaudRate::Bps57600 | BaudRate::Bps115200 => {
                    self.baud = rate;
                    Ok(())
                }
                _ => Err(Error::InvalidBaudRate),
            },
        }
    }

    /// 尝试获取空中波特率（取决于串行波特率和数据表中的信息），官方没有详细说明
    /// Attempts to obtain the air baud rate (depending on the serial baud rate and information in the datasheet),
    /// which is not officially detailed
    pub fn get_air_baud(&self) -> AirBaudRate {
        match self.speed {
            Speed::S1 => AirBaudRate::Bps250000,
            Speed::S2 => AirBaudRate::Bps250000,
            Speed::S3 => match self.baud {
                BaudRate::Bps1200 => AirBaudRate::Bps5000,
                BaudRate::Bps2400 => AirBaudRate::Bps5000,
                BaudRate::Bps4800 => AirBaudRate::Bps15000,
                BaudRate::Bps9600 => AirBaudRate::Bps15000,
                BaudRate::Bps19200 => AirBaudRate::Bps58000,
                BaudRate::Bps38400 => AirBaudRate::Bps58000,
                BaudRate::Bps57600 => AirBaudRate::Bps236000,
                BaudRate::Bps115200 => AirBaudRate::Bps236000,
            },
            Speed::S4 => AirBaudRate::Bps500,
            Speed::S5 => match self.baud {
                BaudRate::Bps19200 => AirBaudRate::Bps58000,
                _ => AirBaudRate::Bps15000,
            },
            Speed::S6 => match self.baud {
                BaudRate::Bps19200 => AirBaudRate::Bps58000,
                BaudRate::Bps38400 => AirBaudRate::Bps58000,
                _ => AirBaudRate::Bps15000,
            },
            Speed::S7 => match self.baud {
                BaudRate::Bps57600 => AirBaudRate::Bps236000,
                _ => AirBaudRate::Bps58000,
            },
            Speed::S8 => match self.baud {
                BaudRate::Bps57600 => AirBaudRate::Bps236000,
                BaudRate::Bps115200 => AirBaudRate::Bps236000,
                _ => AirBaudRate::Bps58000,
            },
        }
    }
}

impl AirBaudRate {
    /// 获取该空中波特率的无线灵敏度（单位 dbm)，接收灵敏度每下降 6 dBm，通信距离会减少一半。
    ///
    /// Obtain the radio sensitivity (in dbm) for this air baud rate.
    /// For every 6 dBm drop in receive sensitivity, the communication range is halved.
    pub fn get_wireless_sensitivity_dbm(&self) -> i32 {
        match self {
            AirBaudRate::Bps500 => -124,
            AirBaudRate::Bps5000 => -116,
            AirBaudRate::Bps15000 => -111,
            AirBaudRate::Bps58000 => -106,
            AirBaudRate::Bps236000 => -100,
            AirBaudRate::Bps250000 => -100,
        }
    }
}
