#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // 보드별 핀을 가정하지 않는 디버깅 확인용 루프입니다.
    loop {
        cortex_m::asm::nop();
    }
}
