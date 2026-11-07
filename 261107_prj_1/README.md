# STM32F411RE Rust 개발 환경

VS Code에서 F5를 누르면 Rust 펌웨어를 빌드하고 ST-LINK를 통해 플래시에 기록한 다음 디버깅합니다. 현재 펌웨어는 주변장치를 설정하지 않고 `nop` 루프를 반복하는 환경 확인용 코드입니다.

## 준비

- STM32F411RE와 ST-LINK를 연결합니다. NUCLEO 보드의 내장 ST-LINK를 사용하는 경우 디버거 쪽 USB 포트에 데이터 케이블을 연결합니다. 별도 디버거라면 SWDIO, SWCLK, GND, 타깃 전압 기준 및 필요 시 NRST를 연결하고 보드에 전원을 공급합니다.
- Rust와 `thumbv7em-none-eabihf` 타깃, `probe-rs` 도구, VS Code의 **rust-analyzer** 및 **probe-rs debugger** 확장이 필요합니다. 이 프로젝트 설정은 Cargo와 probe-rs가 `%USERPROFILE%\.cargo\bin`에 설치된 것을 기준으로 합니다.
- VS Code에서 상위 폴더가 아닌 이 `261107_prj_1` 폴더 자체를 열어 주세요. 처음 열 때 권장 확장을 설치하고 VS Code를 재시작합니다.

## 실행 및 확인

1. `src/main.rs`의 `cortex_m::asm::nop();` 줄에 중단점을 지정합니다.
2. F5를 눌러 **STM32F411RE: 빌드, 플래시, 디버그** 구성을 실행합니다. 빌드가 완료되면 플래시 기록 후 디버거가 연결됩니다.
3. 중단점에 멈추면 계속(F5), 한 단계 실행(F10), 변수 및 레지스터 확인을 사용할 수 있습니다.

F5는 대상 플래시에 새 펌웨어를 기록하므로 기존 펌웨어 내용이 바뀝니다. 코드에 LED 또는 UART 동작은 없으며, 성공 여부는 디버거의 중단점으로 확인합니다.

## 점검 명령 (PowerShell)

```powershell
& "$env:USERPROFILE\.cargo\bin\rustup.exe" target list --installed
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build
& "$env:USERPROFILE\.cargo\bin\probe-rs.exe" list
```

ST-LINK가 검색되지 않으면 케이블과 연결, Windows 드라이버 및 장치 관리자 상태를 확인합니다. 여러 프로브가 연결되어 있으면 `probe-rs list`에서 확인한 식별자를 `.vscode/launch.json`의 `probe` 항목에 지정할 수 있습니다. 칩 종류가 STM32F411RE와 다르면 칩 이름과 `memory.x`를 해당 장치에 맞게 변경해야 합니다.

참고: [probe-rs VS Code 디버거](https://probe.rs/docs/tools/debugger/) · [Rust 임베디드 타깃](https://doc.rust-lang.org/rustc/platform-support/thumbv7em-none-eabi.html)
