use crate::conf::AT_COMMAND_SET_CHANNEL;
use num_traits::ToPrimitive;

use super::{
    baudrate::BaudRate, channel::Channel, power::TransmissionPower, speed::Speed, GenerateAtCommand,
};

impl GenerateAtCommand for BaudRate {
    /// 指令生成器: 波特率
    /// ```rust
    /// let baudrate_9600 = BaudRate::Bps9600;
    /// let baud_command = baudrate_9600.make_command();
    /// assert_eq!(baud_command, b"AT+B9600");
    ///
    /// let baudrate_115200 = hc14::setting::baudrate::BaudRate::Bps115200;
    /// let baud_command = baudrate_115200.make_command();
    /// assert_eq!(baud_command, b"AT+B115200");
    /// ```
    fn make_command<'a>(&self) -> &'a [u8] {
        match self {
            BaudRate::Bps1200 => b"AT+B1200",
            BaudRate::Bps2400 => b"AT+B2400",
            BaudRate::Bps4800 => b"AT+B4800",
            BaudRate::Bps9600 => b"AT+B9600",
            BaudRate::Bps19200 => b"AT+B19200",
            BaudRate::Bps38400 => b"AT+B38400",
            BaudRate::Bps57600 => b"AT+B57600",
            BaudRate::Bps115200 => b"AT+B115200",
        }
    }
}

impl GenerateAtCommand for Speed {
    /// 指令生成器: 无线速率等级，范围：1-8
    /// ```rust
    /// let speed_s3 = Speed::S3;
    /// let speed_command = speed_s3.make_command();
    /// assert_eq!(speed_command, b"AT+S3");
    ///
    /// let speed_s8 = Speed::S8;
    /// let speed_command = speed_s8.make_command();
    /// assert_eq!(speed_command, b"AT+S8");
    /// ```
    ///
    fn make_command<'a>(&self) -> &'a [u8] {
        match self {
            Speed::S1 => b"AT+S1",
            Speed::S2 => b"AT+S2",
            Speed::S3 => b"AT+S3",
            Speed::S4 => b"AT+S4",
            Speed::S5 => b"AT+S5",
            Speed::S6 => b"AT+S6",
            Speed::S7 => b"AT+S7",
            Speed::S8 => b"AT+S8",
        }
    }
}

impl GenerateAtCommand for TransmissionPower {
    /// 指令生成器: 无线发射功率等级，范围：6-20(dbm)
    /// ```rust
    /// let power3 = TransmissionPower::new(6).unwrap();
    /// let power_command = power3.make_command();
    /// assert_eq!(power_command, b"AT+P6");
    ///
    /// let power20 = TransmissionPower::new(20).unwrap();
    /// let power_command = power20.make_command();
    /// assert_eq!(power_command, b"AT+P20");
    /// ```
    ///
    fn make_command<'a>(&self) -> &'a [u8] {
        let power = self.get_power_dbm();
        match power {
            6 => b"AT+P6",
            7 => b"AT+P7",
            8 => b"AT+P8",
            9 => b"AT+P9",
            10 => b"AT+P10",
            11 => b"AT+P11",
            12 => b"AT+P12",
            13 => b"AT+P13",
            14 => b"AT+P14",
            15 => b"AT+P15",
            16 => b"AT+P16",
            17 => b"AT+P17",
            18 => b"AT+P18",
            19 => b"AT+P19",
            20 => b"AT+P20",
            _ => b"AT+P20",
        }
    }
}
impl GenerateAtCommand for Channel {
    /// 指令生成器: 信道，会将指令填充到指令缓冲区中
    /// ```rust
    /// let chaneel_1 = Channel::new(1).unwrap();
    /// let mut chaneel_buffer_1 = [0u8; 7];
    /// let chaneel_command_1 = chaneel_1.make_command(&mut chaneel_buffer_1);
    /// assert_eq!(chaneel_command_1, b"AT+C001");
    ///
    /// let chaneel_26 = Channel::new(26).unwrap();
    /// let mut chaneel_buffer_26 = [0u8; 7];
    /// let chaneel_command_26 = chaneel_26.make_command(&mut chaneel_buffer_26);
    /// assert_eq!(chaneel_command_26, b"AT+C026");
    /// ```
    ///
    fn make_command_buf<'a>(&self, buffer: &'a mut [u8]) -> &'a [u8] {
        /// 基于 10 字节填充
        fn base_10_bytes_padded(mut n: u8, buf: &mut [u8]) -> &[u8] {
            if n == 0 {
                return b"0";
            }
            for i in buf.iter_mut() {
                if n > 0 {
                    *i = (n % 10) as u8 + b'0';
                    n /= 10;
                } else {
                    *i = b"0"[0];
                }
            }
            buf.reverse();
            buf
        }

        let mut buf: [u8; 3] = [0u8; 3];
        let bytes: &[u8] = base_10_bytes_padded(self.to_u8().unwrap(), &mut buf);

        buffer[..4].copy_from_slice(&AT_COMMAND_SET_CHANNEL);
        buffer[4..].copy_from_slice(bytes);
        buffer
    }
}
