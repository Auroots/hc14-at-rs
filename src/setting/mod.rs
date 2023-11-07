/// 波特率数据结构(Baud rate data structure)
pub mod baudrate;
/// 无线通信信道数据结构(Wireless communication channel data structure)
pub mod channel;
/// 指令生成器(Command generator)
pub mod command;
/// HC-14 参数(HC-14 Parameters)
pub mod parameters;
/// 传输功率(transmission power)
pub mod power;
/// 速率模式数据结构(Speed Data Structures)
pub mod speed;

/// Generate AT command
pub trait GenerateAtCommand {
    /// 无需添加缓冲区的命令生成器，以下指令类型必须使用该生成器: **波特率、速率、功率**
    ///
    /// A command generator that does not require the addition of a buffer,
    /// which must be used for the following command types: **baudrate, speed, power**
    fn make_command<'a>(&self) -> &'a [u8] {
        todo!()
    }

    /// 将缓冲区设置为 hc14 的命令, 以下指令类型必须使用该生成器: 信道
    ///
    /// Commands that set the buffer to hc14,
    /// the following command types must use this generator：**channnel**
    fn make_command_buf<'a>(&self, _buffer: &'a mut [u8]) -> &'a [u8] {
        todo!()
    }
}
