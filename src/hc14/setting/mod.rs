//! 参数与数据结构

/// 波特率数据结构
pub mod baudrate;
/// 通信信道数据结构
pub mod channel;
/// 指令生成器
pub mod command;
/// HC-12 参数
pub mod parameters;
/// 传输功率
pub mod power;
/// 速率模式数据结构
pub mod speed;

/// Generate AT command
pub trait GenerateAtCommand {
    /// 无需添加缓冲区的命令生成器，以下指令类型必须使用该生成器:
    /// [**baudrate**], [**speed**], [**power**]
    fn make_command<'a>(&self) -> &'a [u8] {
        todo!()
    }

    /// 将缓冲区设置为 hc14 的命令, 以下指令类型必须使用该生成器:
    /// [**channnel**]
    fn make_command_buf<'a>(&self, _buffer: &'a mut [u8]) -> &'a [u8] {
        todo!()
    }
}
