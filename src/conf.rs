// ! AT查询指令(AT query command)
/// AT查询指令：固件版本`AT+VERSION`
/// - AT query command: Firmware version AT+VERSION
pub const AT_COMMAND_QUERY_VERSION: [u8; 10] = [65, 84, 43, 86, 69, 82, 83, 73, 79, 78];

/// AT查询指令：查询模块是否已经进入了AT指令模式`AT`
/// - AT query command: Check if the module has entered AT command mode AT
pub const AT_COMMAND_QUERY_MODE: [u8; 2] = *b"AT";

/// AT查询指令：查询串口波特率指令`AT+B?`
/// - AT query command: Query serial port baud rate AT+B?
pub const AT_COMMAND_QUERY_BAUD: [u8; 5] = *b"AT+B?";

/// AT查询指令：查询模块的无线信道`AT+C?`
/// - AT query command: Query the module's wireless channel AT+C?
pub const AT_COMMAND_QUERY_CHANNEL: [u8; 5] = *b"AT+C?";

/// AT查询指令：查询模块的无线速率`AT+S?`
/// - AT query command: Query the module's wireless speed AT+S?
pub const AT_COMMAND_QUERY_SPEED: [u8; 5] = *b"AT+S?";

/// AT查询指令：模块的所有基本参数`AT+RX`
/// - AT query command: All basic parameters of the module AT+RX
pub const AT_COMMAND_QUERY_ALL: [u8; 5] = *b"AT+RX";

/// AT查询指令：查询模块的无线发射功率`AT+P?`
/// - AT query command: Query the module's wireless transmission power AT+P?
pub const AT_COMMAND_QUERY_POWER: [u8; 5] = *b"AT+P?";

// ! AT设置指令(AT setup command)
/// AT设置指令：恢复出厂默认值`AT+DEFAULT`
/// - AT setup command: Restore factory default values AT+DEFAULT
pub const AT_COMMAND_DEFAULT: [u8; 10] = *b"AT+DEFAULT";

/// AT设置指令：设置串口波特率指令`AT+B`
/// - AT setup command: Set serial port baud rate AT+B
pub const AT_COMMAND_SET_BAUD: [u8; 4] = *b"AT+B";

/// AT设置指令：设置模块的无线信道`AT+C`
/// - AT setup command: Set the module's wireless channel AT+C
pub const AT_COMMAND_SET_CHANNEL: [u8; 4] = *b"AT+C";

/// AT设置指令：设置模块的无线速率`AT+S`
/// - AT setup command: Set the module's wireless speed AT+S
pub const AT_COMMAND_SET_SPEED: [u8; 4] = *b"AT+S";

/// AT设置指令：设置模块的无线发射功率`AT+P`
/// - AT setup command: Set the module's wireless transmission power AT+P
pub const AT_COMMAND_SET_POWER: [u8; 4] = *b"AT+P";

// ! 响应(responsive)
/// 如果成功进入AT指令模式，将返回以下信息
/// - If successfully entered AT command mode, the following response will be returned
pub const RESPONSE_OK: [u8; 4] = *b"OK\r\n";

/// 如果成功恢复出厂默认值，将返回以下信息
/// - If successfully restored to factory default settings, the following response will be returned
pub const RESPONSE_RESET_SETTINGS: [u8; 12] = *b"OK+DEFAULT\r\n";

/// 波特率响应前缀
/// - Baud rate response prefix
pub const RESPONSE_BAUD: [u8; 5] = *b"OK+B:";

/// 信道响应前缀
/// - Channel response prefix
pub const RESPONSE_CHANNEL: [u8; 5] = *b"OK+C:";

/// 速率响应前缀
/// - Speed response prefix
pub const RESPONSE_SPEED: [u8; 5] = *b"OK+S:";

/// 无线发射功率响应前缀
/// - Wireless transmission power response prefix
pub const RESPONSE_POWER: [u8; 6] = *b"OK+P:+";
