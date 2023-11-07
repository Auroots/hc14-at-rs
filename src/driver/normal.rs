use super::*;
use core::marker::PhantomData;

/// Normal mode
impl<S, P, D> Hc14<S, P, D, Normal>
where
    S: Read<u8> + Write<u8>,
    P: OutputPin,
    D: DelayUs<u32>,
{
    /// !以正常模式构建 Hc14 实例
    /// - Building Hc14 Instances in Normal Mode
    pub fn new(serial: S, mut key_pin: P, mut delay: D) -> Result<Self, ()> {
        let at_off = key_pin.set_high();
        delay.delay_us(100_000_u32); // delay 0.1s
        match at_off {
            Ok(_) => Ok(Self {
                serial,
                key_pin,
                delay,
                mode: PhantomData::<Normal>,
            }),
            Err(_) => Err(nb::Error::Other(())),
        }
    }

    /// ! **"正常模式"** 切换到: "**AT配置模式**"
    /// - **"Normal Mode "** Switch to: "**AT Configuration Mode**"
    pub fn into_configuration_mode(mut self) -> Result<Hc14<S, P, D, Configuration>, ()> {
        let at_on = self.key_pin.set_low();
        self.delay.delay_us(100_000_u32); // delay 0.1s

        match at_on {
            Ok(_) => Ok(Hc14 {
                serial: self.serial,
                key_pin: self.key_pin,
                delay: self.delay,
                mode: PhantomData::<Configuration>,
            }),
            Err(_) => Err(nb::Error::Other(())),
        }
    }

    /// 释放所含资源
    /// - Release of included resources
    pub fn release(self) -> (S, P, D) {
        (self.serial, self.key_pin, self.delay)
    }

    /// **[Normal]**: 将串行端口读取到的信息，返回至整个缓冲区
    /// - Returns the information read from the serial port to the entire buffer.
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

    /// 发送字节 send byte (computing)
    pub fn send_byte(&mut self, word: u8) -> Result<bool, ()> {
        match block!(self.serial.write(word)) {
            Ok(_) => Ok(true),
            Err(_) => Err(nb::Error::Other(())),
        }
    }

    /// 发送字符串(Send String)
    pub fn send_string(&mut self, words: &str) {
        for word in words.as_bytes() {
            if *word == b'\0' {
                break;
            }
            self.send_byte(*word).unwrap();
        }
    }

    /// **[Normal]**: 将整个缓冲区写入串行端口
    ///  - Write the entire buffer to the serial port
    pub fn send_buffer(&mut self, buffer: &[u8]) -> Result<bool, Error<crate::Error>> {
        self.delay.delay_us(100_000_u32); // delay 0.1s
        let mut verify: bool = false;
        for ch in buffer {
            verify = self.send_byte(*ch).is_ok();
        }
        Ok(verify)
    }

    /// 发送无符号数字
    /// -  Send unsigned numbers
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
    // Receive String, Maximum Length: 40
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
/// - Format the parameters returned by the module as i32
///
/// # Examples
/// ```rust
/// let hc14 = hc14::Hc14::new(serial, set, delay).unwrap();
/// let mut hc14_configure = hc14.into_configuration_mode().unwrap();
///
/// // 执行AT指令(Execution of AT commands)
/// let baud_command = BaudRate::Bps9600.make_command();
/// let mut buffer = [0u8; 32];
/// hc14_configure.wirte_command(baud_command, &mut buffer);
///
/// // 将缓冲区中的数组格式化为i32(Format the array in the buffer as i32)
/// let format_baud = format_converter(&buffer, &RESPONSE_BAUD).unwrap();
/// assert_eq!(format_baud, 9600);
///```
/// ![value]: 读取缓冲区
/// ![response]：响应类型
pub fn format_converter<'a>(value: &'a [u8], response: &'a [u8]) -> Result<i32, &'a str> {
    let result = match response {
        &[79, 75, 43, 80, 58, 43] => CommandParser::parse(value)
            .expect_identifier(&[79, 75, 43, 80, 58, 43])
            .expect_int_parameter()
            .expect_identifier(b"dBm\r\n")
            .finish(),
        _ => CommandParser::parse(value)
            .expect_identifier(response)
            .expect_int_parameter()
            .expect_identifier(b"\r\n")
            .finish(),
    };

    match result {
        Ok(n) => Ok(n.0),
        Err(_) => Err(nb::Error::Other("Error: Type conversion error")),
    }
}
