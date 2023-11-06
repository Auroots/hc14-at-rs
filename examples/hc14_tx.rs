// #![feature(alloc_error_handler)]
#![no_main]
#![no_std]
#![feature(slice_range)]
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    rcc::RccExt,
    serial::{self, Serial},
    timer::SysTimerExt,
};

#[entry]
fn main() -> ! {
    // 初始化外设
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain();
    let mut gpioa = dp.GPIOA.split();

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
    let mut delay = syst.delay(&clocks);

    let set = gpioa.pa7.into_open_drain_output(&mut gpioa.crl);

    // USART1
    let tx = gpioa.pa9.into_alternate_open_drain(&mut gpioa.crh);
    let rx = gpioa.pa10;

    // ! 创建HC14实例。取得USART寄存器和tx/rx引脚的所有权。其余寄存器用于启用和配置设备。
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

    let mut hc14 = hc14_at::hc14::Hc14::new(serial, set, delay).unwrap();

    // let dev = AHT10::new(i2c, delay);
    // if let Err(err) = dev {
    //     hprintln!("I2C error: {:?}", err);
    //     loop {}
    // }
    // let mut aht10 = dev.unwrap();

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
    let mut current_char = b'a';
    loop {
        let mut buffer: [u8; 2] = [b'9', 2];

        buffer[1] = current_char;

        hc14.write_buffer(&buffer).unwrap();

        current_char = if current_char == b'z' {
            b'a'
        } else {
            current_char + 1
        };
        // delay.delay_ms(500_u16);
    }
}

// #[exception]
// unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
//     panic!("{:#?}", ef);
// }

// #[alloc_error_handler]
// fn alloc_error(_layout: Layout) -> ! {
//     hprintln!("OOM");
//     loop {}
// }
