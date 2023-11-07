//! 类型状态驱动 Hc14 的状态机。
use super::*;

use core::marker::PhantomData;

/// Hc14 正常模式的实施
impl<S, P, D> Hc14<S, P, D, Normal>
where
    S: Read<u8> + Write<u8>,
    P: OutputPin,
    D: DelayUs<u32>,
{
    /// !以正常模式构建 Hc14 实例
    pub fn new(serial: S, mut set_pin: P, mut delay: D) -> Result<Self, ()> {
        let at_off = set_pin.set_high();
        delay.delay_us(100_000_u32); // delay 0.1s
        match at_off {
            Ok(_) => {
                return Ok(Self {
                    serial,
                    set_pin,
                    delay,
                    mode: PhantomData::<Normal>,
                })
            }
            Err(_) => Err(nb::Error::Other(())),
        }
    }

    /// ! **"正常模式"** 切换到: "**AT配置模式**"
    pub fn into_configuration_mode(mut self) -> Result<Hc14<S, P, D, Configuration>, ()> {
        let at_on = self.set_pin.set_low();
        self.delay.delay_us(100_000_u32); // delay 0.1s

        match at_on {
            Ok(_) => Ok(Hc14 {
                serial: self.serial,
                set_pin: self.set_pin,
                delay: self.delay,
                mode: PhantomData::<Configuration>,
            }),
            Err(_) => Err(nb::Error::Other(())),
        }
    }

    /// 释放所含资源
    pub fn release(self) -> (S, P, D) {
        (self.serial, self.set_pin, self.delay)
    }

    /// **[Normal]**: 将串行端口读取到的信息，返回至整个缓冲区
    pub fn read_buffer<'a>(
        &mut self,
        buffer: &'a mut [u8],
    ) -> Result<&'a [u8], Error<crate::Error>> {
        self.delay.delay_us(100_000_u32); // delay 0.1s
        let mut count: usize = 0;
        for v in buffer.iter_mut() {
            if let Ok(ch) = block!(self.serial.read()) {
                *v = ch;
                count += 1;
                if ch == b'\n' {
                    break;
                }
            }
        }
        Ok(&buffer[..count])
    }

    /// 发送字节
    pub fn send_byte(&mut self, word: u8) -> Result<bool, ()> {
        match block!(self.serial.write(word)) {
            Ok(_) => Ok(true),
            Err(_) => Err(nb::Error::Other(())),
        }
    }

    /// 发送字符串
    pub fn send_string(&mut self, words: &str) {
        for word in words.as_bytes() {
            if *word == b'\0' {
                break;
            }
            self.send_byte(*word).unwrap();
        }
    }

    /// **[Normal]**: 将整个缓冲区写入串行端口
    pub fn send_buffer(&mut self, buffer: &[u8]) -> Result<bool, Error<crate::Error>> {
        self.delay.delay_us(100_000_u32); // delay 0.1s
        let mut verify: bool = false;
        for ch in buffer {
            verify = self.send_byte(*ch).is_ok();
        }
        Ok(verify)
    }

    /// 发送无符号数字
    pub fn send_number(&mut self, number: u32) {
        let mut length: u32 = 0;
        loop {
            length += 1;
            let rounding: u32 = number / (10_u32.pow(length));
            if rounding == 0 {
                break;
            }
        }

        for i in 0..length {
            let v: u32 = number / 10_u32.pow(length - i - 1) % 10 + 48_u32;
            self.send_byte(v as u8).unwrap();
        }
    }

    // 接收字符串, 最大长度: 40
    // pub fn read_string_40(&mut self) -> String<40> {
    //     let mut result: String<40> = String::new();
    //     loop {
    //         if let Ok(ch) = block!(self.serial.read()) {
    //             result.push(ch as char).unwrap();
    //             if ch == b'\n' {
    //                 break;
    //             }
    //         }
    //     }
    //     result
    // }
}

/// 将模块返回的参数格式化为i32
/// 可用于：baud、
/// ```rust
/// // 创建实例
/// let hc14 = hc14::Hc14::new(serial, set, delay).unwrap();
/// let mut hc14_configure = hc14.into_configuration_mode().unwrap();
///
/// // 执行AT指令
/// let baud_command = BaudRate::Bps9600.make_command();
/// let mut buffer = [0u8; 32];
/// hc14_configure.wirte_command(baud_command, &mut buffer);
///
/// // 将缓冲区中的数组格式化为i32
/// let format_baud = format_converter(&buffer, &RESPONSE_BAUD).unwrap();
/// assert_eq!(format_baud, 9600);
///
/// ![value]: 读取缓冲区
/// ![response]：响应类型
///```
pub fn format_converter<'a>(value: &'a [u8], response: &'a [u8]) -> Result<i32, &'a str> {
    let result = match response {
        &[79, 75, 43, 80, 58, 43] => CommandParser::parse(&value)
            .expect_identifier(&[79, 75, 43, 80, 58, 43])
            .expect_int_parameter()
            .expect_identifier(b"dBm\r\n")
            .finish(),
        _ => CommandParser::parse(&value)
            .expect_identifier(response)
            .expect_int_parameter()
            .expect_identifier(b"\r\n")
            .finish(),
    };

    match result {
        Ok(n) => {
            return Ok(n.0);
        }
        Err(_) => Err(nb::Error::Other("Error: Type conversion error")),
    }
}

// /// 在正常模式下对 Hc14 执行读取
// /// 这只需服从底层串行实现。
// impl<S, P, D> embedded_hal::serial::Read<u8> for Hc14<S, P, D, Normal>
// where
//     S: Read<u8> + Write<u8>,
//     P: OutputPin,
//     D: DelayUs<u16>,
// {
//     type Error = <S as Read<u8>>::Error;

//     fn read(&mut self) -> nb::Result<u8, Self::Error> {
//         self.serial.read()
//     }
// }

// /// 在正常模式下为 Hc14 执行写入操作。
// /// 这只需服从底层串行实现。
// impl<S, P, D> embedded_hal::serial::Write<u8> for Hc14<S, P, D, Normal>
// where
//     S: embedded_hal::serial::Read<u8> + embedded_hal::serial::Write<u8>,
//     P: OutputPin,
//     D: DelayUs<u16>,
// {
//     type Error = <S as Write<u8>>::Error;
//     fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
//         self.serial.write(word)
//     }

//     fn flush(&mut self) -> nb::Result<(), Self::Error> {
//         self.serial.flush()
//     }
// }
