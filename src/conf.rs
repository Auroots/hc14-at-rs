// ! AT查询指令
/// AT查询指令：固件版本`AT+VERSION`，返回 "www.hc01.com HC-14V1.1 2022.12.1" 字数32
pub const AT_COMMAND_QUERY_VERSION: [u8; 10] = [65, 84, 43, 86, 69, 82, 83, 73, 79, 78];

/// AT查询指令：查询模块是否已经进入了AT指令模式`AT`，返回 "OK" 字数2
pub const AT_COMMAND_QUERY_MODE: [u8; 2] = *b"AT";

/// AT查询指令：查询串口波特率指令`AT+B?`
pub const AT_COMMAND_QUERY_BAUD: [u8; 5] = *b"AT+B?";

/// AT查询指令：查询模块的无线信道`AT+C?`
pub const AT_COMMAND_QUERY_CHANNEL: [u8; 5] = *b"AT+C?";

/// AT查询指令：查询模块的无线速率`AT+S?`
pub const AT_COMMAND_QUERY_SPEED: [u8; 5] = *b"AT+S?";

/// AT查询指令：模块的所有基本参数`AT+RX`
pub const AT_COMMAND_QUERY_ALL: [u8; 5] = *b"AT+RX";

/// AT查询指令：查询模块的无线发射功率`AT+P?`
pub const AT_COMMAND_QUERY_POWER: [u8; 5] = *b"AT+P?";

// ! AT设置指令
/// AT设置指令：恢复出厂默认值`AT+DEFAULT`, 返回 "OK+DEFAULT"
pub const AT_COMMAND_DEFAULT: [u8; 10] = *b"AT+DEFAULT";

/// AT设置指令：设置串口波特率指令`AT+Bxxx`
/// xxx代表你需要设置的值
pub const AT_COMMAND_SET_BAUD: [u8; 4] = *b"AT+B";

/// AT设置指令：设置模块的无线信道`AT+Cxxx`
/// xxx代表你需要设置的值
pub const AT_COMMAND_SET_CHANNEL: [u8; 4] = *b"AT+C";

/// AT设置指令：设置模块的无线速率`AT+Sxxx`
/// xxx代表你需要设置的值
pub const AT_COMMAND_SET_SPEED: [u8; 4] = *b"AT+S";

/// AT设置指令：设置模块的无线发射功率`AT+Pxxx`，设置范围为：6~20dBm，默认：20dBm
/// xxx代表你需要设置的值
pub const AT_COMMAND_SET_POWER: [u8; 4] = *b"AT+P";

// ! 响应
/// 如果成功进入AT指令模式，将返回以下信息
pub const RESPONSE_OK: [u8; 4] = *b"OK\r\n";

/// 如果成功恢复出厂默认值，将返回以下信息
pub const RESPONSE_RESET_SETTINGS: [u8; 12] = *b"OK+DEFAULT\r\n";

/// 波特率响应前缀
pub const RESPONSE_BAUD: [u8; 5] = *b"OK+B:";

/// 信道响应前缀
pub const RESPONSE_CHANNEL: [u8; 5] = *b"OK+C:";

/// 速率响应前缀
pub const RESPONSE_SPEED: [u8; 5] = *b"OK+S:";

/// 无线发射功率响应前缀
pub const RESPONSE_POWER: [u8; 6] = *b"OK+P:+";

// 待验证的指令
//
