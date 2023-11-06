use super::{
    setting::{
        baudrate::BaudRate, channel::Channel, parameters::Parameters, power::TransmissionPower,
        speed::Speed,
    },
    *,
};
use crate::conf::{
    AT_COMMAND_DEFAULT, AT_COMMAND_QUERY_ALL, AT_COMMAND_QUERY_MODE, RESPONSE_OK,
    RESPONSE_RESET_SETTINGS,
};
use core::{marker::PhantomData, result::Result::*};
use embedded_hal::digital::v2::OutputPin;

/// 配置模式
impl<S, P, D> Hc14<S, P, D, Configuration>
where
    S: Read<u8> + Write<u8>,
    P: OutputPin + embedded_hal::digital::v2::InputPin,
    D: DelayUs<u32>,
{
    /// ! **"AT配置模式"** 切换到 "**正常模式**"。
    pub fn into_normal_mode(mut self) -> Result<Hc14<S, P, D, Normal>, <P as InputPin>::Error> {
        let _ = self.set_pin.set_high();
        self.delay.delay_us(100_000_u32); // delay 0.1s

        match self.set_pin.is_high() {
            Ok(_) => Ok(Hc14 {
                serial: self.serial,
                set_pin: self.set_pin,
                delay: self.delay,
                mode: PhantomData::<Normal>,
            }),
            Err(e) => Err(nb::Error::Other(e)),
        }
    }

    /// ! 使用 "AT" 指令检查，当前是否为: **AT配置模式**
    /// ```rust
    /// let hc14 = hc14::Hc14::new(serial, set, delay).unwrap();
    /// let mut hc14_configure = hc14.into_configuration_mode().unwrap();
    /// assert!(hc14_configure.is_at_mode());
    /// ```
    pub fn is_at_mode(&mut self) -> bool {
        for ch in &AT_COMMAND_QUERY_MODE {
            let _ = block!(self.serial.write(*ch));
        }
        let mut n = 0;
        let mut buffer = [0u8; 4];
        while n < 4 {
            if let Ok(ch) = block!(self.serial.read()) {
                buffer[n] = ch;
                n += 1;
            }
        }
        buffer == RESPONSE_OK
    }

    /// **[Configuration]**: 将串行端口读取到的指令信息，返回至整个缓冲区
    pub fn read_buffer<'a>(&mut self, buffer: &'a mut [u8]) -> &'a [u8] {
        let mut count = 0;
        for v in buffer.iter_mut() {
            if let Ok(ch) = block!(self.serial.read()) {
                *v = ch;
                count += 1;
                if ch == b'\n' {
                    break;
                }
            }
        }
        &buffer[..count]
    }

    /// 发送字节
    pub fn send_byte(&mut self, word: u8) -> Result<bool, ()> {
        match block!(self.serial.write(word)) {
            Ok(_) => Ok(true),
            Err(_) => Err(nb::Error::Other(())),
        }
    }

    /// **[Configuration]**: 将整个缓冲区写入串行端口
    pub fn send_buffer<'a>(&mut self, buffer: &'a [u8]) -> Result<bool, Error<crate::Error>> {
        let mut verify = false;
        for ch in buffer {
            verify = self.send_byte(*ch).is_ok();
        }
        Ok(verify)
    }

    /// 写入并读取指令
    /// 可用该方法写入的指令有：power、speed、baudrate、get_version
    ///
    /// ### power指令 - 设置模块的发射功率等级
    /// 出厂默认设置为 3，发射功率最大，通信距离最远。
    /// 发射功率等级设置为 1，发射功率最小。一般来说，发射 功率每下降 6dB，通信距离会减少一半
    ///
    /// ### speed指令 - 设置模块的无线速率，串口透传模式
    /// 模块有8种无线速率，不同速率是不能互传数据的。S1是最低速率，此时通信速度最慢、无线接收
    /// 灵敏度最高、通信距离最远。速率越高，通信距离越近，用户可以根据实际情况选择最优速率。
    ///
    /// ### baudrate指令 - 设置模块的波特率(bauds per second)
    ///  1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200
    ///
    ///
    /// ### 例程
    /// ```rust
    /// // 创建hc14实例
    /// let hc14 = hc14::Hc14::new(serial, set, delay).unwrap();
    /// // 将模块切换至 "AT配置模式"
    /// let mut hc14_configure = hc14.into_configuration_mode().unwrap();
    /// // 获取指令
    /// let baud_command = BaudRate::Bps9600.make_command();
    /// // 创建缓冲区变量
    /// let mut buffer = [0u8; 32];
    /// // 写入指令，并将返回信息输出到缓冲区
    /// hc14_configure.wirte_command(baud_command, &mut buffer);
    /// //读取缓冲区
    /// assert_eq!(&buffer, b"OK+B:9600\r\n");
    ///
    /// ```
    /// 或者你想通过查询指令，获取的信息
    /// ```rust
    ///  hc14_configure.wirte_command(&AT_COMMAND_QUERY_VERSION, &mut buffer);
    ///
    pub fn wirte_command<'a>(&mut self, command: &[u8], buffer: &'a mut [u8]) -> &'a [u8] {
        self.send_buffer(&command).unwrap();
        self.read_buffer(buffer)
    }

    /// 将 Hc14 重置为默认设置。
    pub fn reset_settings(&mut self) -> bool {
        self.send_buffer(&AT_COMMAND_DEFAULT).unwrap();

        let mut response = [0u8; 12];
        let mut count = 0;
        for v in &mut response {
            if let Ok(ch) = block!(self.serial.read()) {
                *v = ch;
                count += 1;
                if ch == b'\n' {
                    break;
                }
            }
        }
        count == RESPONSE_RESET_SETTINGS.len()
            && response[..count] == RESPONSE_RESET_SETTINGS[..count]
    }

    /// 获取 Hc14 的参数
    /// ```rust
    /// let hc14 = hc14::Hc14::new(serial, set, delay).unwrap();
    /// let mut hc14_configure = hc14.into_configuration_mode().unwrap();
    /// hprintln!("{:#?}", hc14.get_parameters());
    /// ```
    pub fn get_parameters(&mut self) -> Option<Parameters> {
        for ch in &AT_COMMAND_QUERY_ALL {
            let _ = block!(self.serial.write(*ch));
        }
        let mut params = [[0u8; 16]; 4];
        let mut param_slices: [&[u8]; 4] = Default::default();
        for (pi, p) in &mut params.iter_mut().enumerate() {
            for (i, v) in p.iter_mut().enumerate() {
                match block!(self.serial.read()) {
                    Ok(ch) => {
                        *v = ch;
                        if ch == b'\n' {
                            param_slices[pi] = &p[..=i];
                            break;
                        }
                    }
                    Err(_) => (),
                }
            }
        }
        let baud = BaudRate::try_from(param_slices[0]).ok()?;
        let channel = Channel::try_from(param_slices[1]).ok()?;
        let speed = Speed::try_from(param_slices[2]).ok()?;
        let power = TransmissionPower::try_from(param_slices[3]).ok()?;

        Some(Parameters {
            baud,
            channel,
            power,
            speed,
        })
    }

    /// 设置无线信道, 信道范围从1-50
    /// 该设置方法有两个，都是可用的
    /// ```rust
    /// let hc14 = hc14::Hc14::new(serial, set, delay).unwrap();
    /// // 进入配置模式
    /// let mut hc14_configure = hc14.into_configuration_mode().unwrap();
    /// // 创建缓冲区
    /// let mut buffer = [0u8; 16];
    /// // 执行信道设置指令
    /// hc14_configure.wirte_set_channel(2, &mut buffer);
    /// ```
    ///
    pub fn wirte_set_channel<'a>(
        &mut self,
        mut channel_number: i32,
        buffer: &'a mut [u8],
    ) -> &'a [u8] {
        let mut channel_command = [65, 84, 43, 67, 48, 48, 48];

        let mut buf = [0u8; 2];
        {
            let mut count: usize = 0;
            while channel_number > 0 {
                buf[count] = (channel_number % 10) as u8;
                channel_number /= 10;
                count += 1;
            }
        }
        // 使用迭代器输出分离的数字
        {
            for (index, digit) in buf.iter().rev().enumerate() {
                let ascii_value: u8 = (digit + 48) as u8;
                channel_command[5 + index] = ascii_value;
            }
        }
        self.wirte_command(&channel_command, buffer)
    }
}
