#![feature(alloc_error_handler)]
#![no_main]
#![no_std]
#![feature(slice_range)]

use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m_semihosting::hprintln;
use hc14_at_rs::{self, driver::Hc14};
use panic_halt as _;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    serial::{self, Serial},
    timer::{SysDelay, SysTimerExt},
};

// #[macro_use]
extern crate alloc;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;

// 配置全局堆分配器
// Configuring the Global Heap Allocator
#[global_allocator]
static HEAP: CortexMHeap = CortexMHeap::empty();

use core::mem::MaybeUninit;
const HEAP_SIZE: usize = 1024;
static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

// 程序入口
// Entry point of the program
#[entry]
fn main() -> ! {
    // 初始化分配器，务必在使用前初始化
    // Initialize the allocator; make sure to initialize before use
    {
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    // 获取 cortex_m 和 HAL 的 Peripheral 设备
    // Obtain cortex_m and HAL peripheral devices
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let mut afio = dp.AFIO.constrain();
    let mut gpioa = dp.GPIOA.split();

    // 使用配置的时钟配置创建一个精准延迟函数
    // Create a custom delay function with precise timing
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze(&mut flash.acr);

    // 创建一个具有自定义精度的阻塞延迟函数
    // Create a custom precise delay function
    let syst = cp.SYST;
    let delay: SysDelay = syst.delay(&clocks);

    // 初始化串行通信引脚及串口配置，然后创建串口设备
    // Initialize serial communication pins and configure the serial port, then create the serial device
    let key = gpioa.pa7.into_open_drain_output(&mut gpioa.crl);
    let tx = gpioa.pa9.into_alternate_open_drain(&mut gpioa.crh);
    let rx = gpioa.pa10;
    let config = serial::Config::default();
    let serial = Serial::new(
        dp.USART1,
        (tx, rx),
        &mut afio.mapr,
        config
            .baudrate(9600.bps())
            .stopbits(serial::StopBits::STOP2)
            .wordlength_8bits()
            .parity_none(),
        &clocks,
    );

    // 创建 Hc14 实例
    // Create an Hc14 instance
    let hc14 = Hc14::new(serial, key, delay).unwrap();

    // 重置模式参数
    // Reset mode parameters
    let mut hc14_configure = hc14.into_configuration_mode().unwrap();
    hc14_configure.reset_settings();

    // 获取 Hc14 的参数
    // Get Hc14 parameters
    let hc14_parameters = hc14_configure.get_parameters().unwrap();
    hprintln!("{:#?}", hc14_parameters);

    // 设置信道
    // Set channel
    let mut buffer = [0u8; 32];
    hc14_configure.wirte_set_channel(28, &mut buffer);

    // 切换至普通模式
    // Switch to normal mode
    let mut hc14_normal = hc14_configure.into_normal_mode().unwrap();

    loop {
        // 发送缓冲区
        // Send buffer
        let buf = b"hc14";
        hc14_normal.send_buffer(buf).unwrap();

        // 发送字符串
        // Send a string
        let buf_str = "hc14";
        hc14_normal.send_string(buf_str);

        // 读取信息并存入缓冲区，然后输出每个字符
        // Read information into a buffer and print each character
        let mut buf_read: [u8; 32] = [0u8; 32];
        hc14_normal.read_buffer(&mut buf_read).unwrap();
        for i in buf_read.iter() {
            hprintln!("{}", *i as char);
        }
    }
}

// 处理 HardFault 异常
// Handle the HardFault exception
#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

// 自定义堆分配错误处理
// Custom heap allocation error handling
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    hprintln!("OOM");
    loop {}
}
