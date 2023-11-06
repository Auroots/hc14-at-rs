//!串行接口常用工具集

use core::u32;
use heapless::String;

use nb::block;
use stm32f1xx_hal::serial;
use unwrap_infallible::UnwrapInfallible;

/// 发送字节
pub fn send_byte<USART>(tx: &mut serial::Tx<USART>, word: u8)
where
    USART: serial::Instance,
{
    // tx.write(word)
    block!(tx.write(word)).unwrap_infallible()
}

/// 发送字节数组
pub fn send_bytes<USART>(tx: &mut serial::Tx<USART>, words: &[u8])
where
    USART: serial::Instance,
{
    words.iter().for_each(|w| send_byte(tx, *w));
}

/// 发送字符串
pub fn send_string<USART>(tx: &mut serial::Tx<USART>, words: &str)
where
    USART: serial::Instance,
{
    for word in words.as_bytes() {
        if *word == b'\0' {
            break;
        }
        send_byte(tx, *word);
    }
}

/// 发送数字
pub fn send_number<USART>(tx: &mut serial::Tx<USART>, number: u32)
where
    USART: serial::Instance,
{
    let mut length = 0;
    loop {
        length += 1;
        let rounding = number / (10_u32.pow(length));
        if rounding == 0 {
            break;
        }
    }

    for i in 0..length {
        let v = number / 10_u32.pow(length - i - 1) % 10 + '0' as u32;
        send_byte(tx, v as u8);
    }
}

/// 接收字节数组
/// 最大长度: 4096
pub fn receive_bytes<USART>(rx: &mut serial::Rx<USART>, buffer: &mut [u8])
where
    USART: serial::Instance,
{
    let mut widx: usize = 0;
    loop {
        let w = block!(rx.read()).unwrap();
        if w == b'\n' {
            break;
        }
        if widx < buffer.len() {
            buffer[widx] = w;
            widx += 1;
        }
    }
}

/// 接收字符串
/// 最大长度: 4096
pub fn receive_string<USART>(rx: &mut serial::Rx<USART>) -> String<64>
where
    USART: serial::Instance,
{
    let mut s: String<64> = String::new();

    loop {
        let w = block!(rx.read()).unwrap();
        if w == b'\n' {
            break;
        }
        s.push(w as char).unwrap();
    }
    s
}

// 奇校验
pub fn odd_parity(byte: u8) -> u8 {
    let mut count = 0;
    let mut data = byte;
    while data > 0 {
        count += data & 1;
        data >>= 1;
    }
    if count % 2 == 0 {
        // 如果1的数量为偶数，添加一个1来保持奇校验
        byte | 1
    } else {
        // 如果1的数量为奇数，不需要添加
        byte
    }
}

// 偶校验
pub fn even_parity(byte: u8) -> u8 {
    let mut count = 0;
    let mut data = byte;
    while data > 0 {
        count += data & 1;
        data >>= 1;
    }
    if count % 2 == 0 {
        // 如果1的数量为偶数，不需要添加
        byte
    } else {
        // 如果1的数量为奇数，添加一个1来保持偶校验
        byte | 1
    }
}

// fn main() {
//     let data: u8 = 0b11010010; // 二进制数据
//     let data_with_odd_parity = odd_parity(data);
//     let data_with_even_parity = even_parity(data);

//     println!("Data with odd parity: {:08b}", data_with_odd_parity);
//     println!("Data with even parity: {:08b}", data_with_even_parity);
// }
