#![feature(alloc_error_handler)]
#![no_main]
#![no_std]
#![feature(slice_range)]

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::Point,
    text::{Baseline, Text},
    Drawable,
};

use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m_semihosting::hprintln;

// use nb::block;
use panic_halt as _;

use ssd1306::{
    prelude::DisplayConfig, rotation::DisplayRotation, size::DisplaySize128x32,
    I2CDisplayInterface, Ssd1306,
};
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    pac,
    prelude::*,
    rcc::RccExt,
    serial::{self, Serial},
    timer::{SysDelay, SysTimerExt},
};

// #[macro_use]
extern crate alloc;
use alloc::string::String;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;

#[global_allocator]
static HEAP: CortexMHeap = CortexMHeap::empty();

use core::mem::MaybeUninit;
const HEAP_SIZE: usize = 1024;
static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    {
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
    // 初始化外设
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    // let syst: pac::SYST = cp.SYST;
    let mut afio = dp.AFIO.constrain();
    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();

    // 封装具有自定义精度的阻塞延迟函数
    let clocks = if 1 == 1 {
        rcc.cfgr.use_hse(8.MHz()).freeze(&mut flash.acr)
    } else {
        // 我的 Blue Pill（搭载 stm32f103 克隆芯片）似乎不遵循 rcc，所以不会补偿脉冲长度。
        // 我们需要更快的时钟来补偿。
        rcc.cfgr
            .use_hse(8.MHz())
            .sysclk(108.MHz())
            .pclk1(6.MHz())
            .freeze(&mut flash.acr)
    };
    let syst = cp.SYST;
    // 具有自定义精度的阻塞延迟
    let delay: SysDelay = syst.delay(&clocks);

    let set = gpioa.pa7.into_open_drain_output(&mut gpioa.crl);
    // USART1
    let tx1 = gpioa.pa9.into_alternate_open_drain(&mut gpioa.crh);
    let rx1 = gpioa.pa10;
    // let mut led = gpiob.pb2.into_push_pull_output(&mut gpiob.crl);

    // 设置usart设备。取得USART寄存器和tx/rx引脚的所有权。其余寄存器用于启用和配置设备。
    // hprintln!("load serial...");
    let config = serial::Config::default();
    let serial = Serial::new(
        dp.USART1,
        (tx1, rx1),
        &mut afio.mapr,
        config
            .baudrate(9600.bps())
            .stopbits(serial::StopBits::STOP2)
            .wordlength_8bits()
            .parity_none(),
        &clocks,
    );

    let mut hc14 = hc14_at::hc14::Hc14::new(serial, set, delay).unwrap();

    let oled_scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let oled_sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    // 初始化第一个 I2C 总线
    let oled_i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (oled_scl, oled_sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400.kHz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        1000,
        10,
        1000,
        1000,
    );
    let interface = I2CDisplayInterface::new(oled_i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();
    // ! 重置模式参数
    // let mut hc14 = hc14.into_configuration_mode().unwrap();
    // hc14.reset_settings();
    // hprint!("{:?}", hc14.get_parameters());
    // ! 设置信道
    // let mut buffer = [0u8; 32];
    // hc14.wirte_set_channel(28, &mut buffer);

    // ! 读取缓冲区
    // for i in buffer.iter() {
    //     hprint!("{}", *i as char);
    // }
    // ! 测试all指令
    // let (mut tx, mut rx) = serial.split();
    // set.set_low();
    // get_parameters(&mut rx, &mut tx);
    // let mut clo = 0;

    loop {
        let mut buffer: [u8; 2] = [0u8; 2];
        hc14.read_buffer(&mut buffer);

        let mut result = String::new();
        for i in buffer.iter() {
            result.push(*i as char);
        }
        // hprintln!("result: {}", result);
        display.clear_buffer();
        Text::with_baseline(&result, Point::zero(), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        display.flush().unwrap();
    }
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    hprintln!("OOM");
    loop {}
}
