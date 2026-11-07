#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_stm32::init(Default::default());
    // NUCLEO-F411RE의 사용자 LED LD2는 PA5에 연결되어 있습니다.
    let mut led = Output::new(peripherals.PA5, Level::Low, Speed::Low);

    defmt::info!("NUCLEO-F411RE LED 예제 시작");
    loop {
        led.set_high();
        defmt::info!("LD2 켜짐");
        Timer::after_secs(1).await;

        led.set_low();
        defmt::info!("LD2 꺼짐");
        Timer::after_secs(1).await;
    }
}
