#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use embassy_stm32::bind_interrupts;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::peripherals::USART2;
use embassy_stm32::usart::{BufferedInterruptHandler, BufferedUart, Config};
use embassy_time::{Duration, Instant, Timer};
use embedded_io_async::{Read, Write};
use {defmt_rtt as _, panic_probe as _};

// USART2 인터럽트가 입력을 버퍼에 저장하므로, 타이머를 기다리는 중에도 수신합니다.
bind_interrupts!(struct Irqs {
    USART2 => BufferedInterruptHandler<USART2>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_stm32::init(Default::default());
    // NUCLEO-F411RE의 사용자 LED LD2는 PA5에 연결되어 있습니다.
    let mut led = Output::new(peripherals.PA5, Level::High, Speed::Low);

    // PA2: TX(PC 방향), PA3: RX(PC에서 수신). ST-LINK 가상 COM을 사용합니다.
    let mut uart_config = Config::default();
    uart_config.baudrate = 115_200;
    uart_config.eager_reads = Some(1);
    let mut tx_buffer = [0u8; 128];
    let mut rx_buffer = [0u8; 128];
    let mut uart = BufferedUart::new(
        peripherals.USART2,
        peripherals.PA3,
        peripherals.PA2,
        &mut tx_buffer,
        &mut rx_buffer,
        Irqs,
        uart_config,
    )
    .expect("USART2 설정 실패");

    defmt::info!("NUCLEO-F411RE LED 명령 예제 시작");
    send(
        &mut uart,
        b"[INFO] LED ON (blink)\r\nCommands: on, off, blink, status, help\r\n",
    )
    .await;
    let mut blinking = true;
    let mut led_on = true;
    let mut deadline = Instant::now() + Duration::from_secs(1);
    let mut command = [0u8; 32];
    let mut len = 0usize;
    let mut overflow = false;
    let mut received = [0u8; 1];

    loop {
        // 각 입력 문자에서 1초 타이머를 재시작하지 않도록 절대 시각을 사용합니다.
        match select(uart.read(&mut received), Timer::at(deadline)).await {
            Either::First(Ok(1)) => match received[0] {
                b'\r' | b'\n' => {
                    if overflow {
                        send(&mut uart, b"[ERROR] Command too long\r\n").await;
                    } else if len != 0 {
                        let line = command[..len].trim_ascii();
                        if line.eq_ignore_ascii_case(b"on") {
                            blinking = false;
                            led_on = true;
                            led.set_high();
                            send(&mut uart, b"[OK] LED ON (manual)\r\n").await;
                            defmt::info!("LD2 켜짐 (수동)");
                        } else if line.eq_ignore_ascii_case(b"off") {
                            blinking = false;
                            led_on = false;
                            led.set_low();
                            send(&mut uart, b"[OK] LED OFF (manual)\r\n").await;
                            defmt::info!("LD2 꺼짐 (수동)");
                        } else if line.eq_ignore_ascii_case(b"blink") {
                            blinking = true;
                            led_on = true;
                            led.set_high();
                            deadline = Instant::now() + Duration::from_secs(1);
                            send(&mut uart, b"[OK] LED ON (blink)\r\n").await;
                            defmt::info!("LD2 점멸 시작");
                        } else if line.eq_ignore_ascii_case(b"status") {
                            // 상태는 출력을 설정한 시점의 소프트웨어 값입니다.
                            let response: &[u8] = match (blinking, led_on) {
                                (true, true) => b"[STATUS] Mode: blink, LED: ON\r\n",
                                (true, false) => b"[STATUS] Mode: blink, LED: OFF\r\n",
                                (false, true) => b"[STATUS] Mode: manual, LED: ON\r\n",
                                (false, false) => b"[STATUS] Mode: manual, LED: OFF\r\n",
                            };
                            send(&mut uart, response).await;
                        } else if line.eq_ignore_ascii_case(b"help") {
                            send(&mut uart, b"Commands: on, off, blink, status, help\r\n").await;
                        } else if !line.is_empty() {
                            send(
                                &mut uart,
                                b"[ERROR] Unknown command. Use: on, off, blink, status, help\r\n",
                            )
                            .await;
                        }
                    }
                    len = 0;
                    overflow = false;
                }
                8 | 127 if !overflow => len = len.saturating_sub(1),
                byte if !overflow => {
                    if len < command.len() {
                        command[len] = byte;
                        len += 1;
                    } else {
                        // 넘친 입력은 개행까지 버리고 잘린 명령을 실행하지 않습니다.
                        overflow = true;
                    }
                }
                _ => {}
            },
            Either::First(Ok(_)) => {}
            Either::First(Err(_)) => {
                // 손상된 명령이 실행되지 않도록 나머지 입력을 개행까지 버립니다.
                overflow = true;
                len = 0;
                defmt::warn!("UART 수신 오류: 줄 끝까지 입력을 무시합니다");
            }
            Either::Second(()) => {
                deadline = Instant::now() + Duration::from_secs(1);
                if blinking {
                    led_on = !led_on;
                    led.set_level(if led_on { Level::High } else { Level::Low });
                    if led_on {
                        send(&mut uart, b"[INFO] LED ON (blink)\r\n").await;
                        defmt::info!("LD2 켜짐");
                    } else {
                        send(&mut uart, b"[INFO] LED OFF (blink)\r\n").await;
                        defmt::info!("LD2 꺼짐");
                    }
                }
            }
        }
    }
}

async fn send(uart: &mut BufferedUart<'_>, message: &[u8]) {
    uart.write_all(message).await.expect("UART 송신 실패");
}
