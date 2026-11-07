# STM32F411RE Rust 개발 환경

다른 Windows PC에서 환경을 처음부터 구성하려면 [환경 재구성 가이드](ENVIRONMENT_SETUP.md)를 참고하세요.

VS Code에서 F5를 누르면 Rust 펌웨어를 빌드하고 ST-LINK를 통해 플래시에 기록한 다음 디버깅합니다. 현재 펌웨어는 NUCLEO-F411RE의 사용자 LED LD2(PA5)를 기본으로 1초 켜고 1초 끄며 상태를 UART 시리얼 및 RTT 로그로 출력합니다. 시리얼 명령으로 LED를 켜거나 끄고 점멸을 다시 시작할 수 있습니다. 점멸 모드에서 상태가 1초마다 바뀌고 한 번의 완전한 깜빡임 주기는 2초입니다.

## 준비

- STM32F411RE와 ST-LINK를 연결합니다. NUCLEO 보드의 내장 ST-LINK를 사용하는 경우 디버거 쪽 USB 포트에 데이터 케이블을 연결합니다. 별도 디버거라면 SWDIO, SWCLK, GND, 타깃 전압 기준 및 필요 시 NRST를 연결하고 보드에 전원을 공급합니다.
- Rust와 `thumbv7em-none-eabihf` 타깃, `probe-rs` 도구, VS Code의 **rust-analyzer** 및 **probe-rs debugger** 확장이 필요합니다. 이 프로젝트 설정은 Cargo와 probe-rs가 `%USERPROFILE%\.cargo\bin`에 설치된 것을 기준으로 합니다.
- 이 PC에는 Rust GNU Windows 도구 모음과 기존 `C:\mingw64\bin` 링커를 사용하도록 설치했습니다. 새 PATH를 적용하려면 실행 중인 VS Code 창을 모두 종료하고 다시 실행합니다.
- VS Code에서 상위 폴더가 아닌 이 `261107_prj_1` 폴더 자체를 열어 주세요. 처음 열 때 권장 확장을 설치하고 VS Code를 재시작합니다.

## 실행 및 확인

1. `src/main.rs`의 `led.set_high();` 줄에 중단점을 지정합니다.
2. F5를 눌러 **STM32F411RE: 빌드, 플래시, 디버그** 구성을 실행합니다. 빌드가 완료되면 플래시 기록 후 리셋 위치에서 멈춥니다.
3. F5로 계속 실행하여 중단점에 도달하면 변수 및 레지스터를 확인할 수 있습니다.
4. LED와 로그를 연속으로 보려면 중단점을 끄고 F5로 계속 실행합니다. VS Code 터미널의 probe-rs RTT 채널에 시작 메시지와 `LD2 켜짐`, `LD2 꺼짐`이 번갈아 출력됩니다. RTT는 ST-LINK의 디버그 연결을 사용하며 COM6 시리얼 포트 출력이 아닙니다.

F5는 대상 플래시에 새 펌웨어를 기록하므로 기존 펌웨어 내용이 바뀝니다. 실행 중인 코드가 `panic!`으로 중단되면 `panic-probe`가 RTT에 오류 정보를 출력합니다. 시작 메시지를 놓치더라도 주기적인 LED 상태 메시지로 로그 동작을 확인할 수 있습니다.

디버그 연결은 보드가 저전력 대기 중에도 연결할 수 있도록 리셋 상태 연결을 사용합니다. 개발 빌드에서는 UART 수신 처리를 위해 의존성만 최적화하고 애플리케이션 코드는 최적화하지 않습니다.

## 디버거 없이 UART 로그 및 명령 사용

펌웨어를 한 번 보드에 기록한 후 디버그 세션을 종료하고 보드 전원을 다시 넣거나 RESET 버튼을 눌러 실행합니다. USB 데이터 케이블로 보드의 ST-LINK 포트를 PC에 연결한 상태에서 VS Code Serial Monitor 등의 시리얼 터미널을 엽니다.

- Windows 장치 관리자에서 **STMicroelectronics STLink Virtual COM Port**의 COM 번호를 확인합니다. 이 PC에서 확인한 번호는 COM6이지만 다른 PC나 연결 상태에서는 달라질 수 있습니다.
- VS Code에서 **Ctrl+Shift+P → Open Serial**을 실행하고 해당 포트를 선택해 연결합니다. **115200 baud, 데이터 8비트, 패리티 없음, 정지 비트 1, 흐름 제어 없음(115200 8N1)**으로 설정합니다. 새 PC에 확장이 없으면 `eclipse-cdt.serial-monitor`를 설치합니다.
- Serial Monitor의 터미널에 명령을 입력하고 **Enter**를 누릅니다. 문자들이 입력되는 즉시 전송되며, Enter로 명령을 완료합니다. 일반 ASCII 텍스트이며 CR, LF 또는 CRLF 줄바꿈을 지원합니다.
- 다른 프로그램이 포트를 사용 중이면 먼저 닫습니다. 출력이나 명령 응답이 없다면 ST-LINK USB 포트, 케이블, COM 번호와 설정을 확인합니다. 기본 보드에서 USART2의 PA2(TX)와 PA3(RX)는 ST-LINK 가상 COM에 연결되며, 솔더 브리지 설정이 변경된 보드라면 연결을 확인합니다. [ST 보드 매뉴얼](https://www.st.com/resource/en/user_manual/dm00105823.pdf)

| 명령 | 동작 | 응답 |
| --- | --- | --- |
| `on` | 점멸을 중지하고 켬 상태 유지 | `[OK] LED ON (manual)` |
| `off` | 점멸을 중지하고 끔 상태 유지 | `[OK] LED OFF (manual)` |
| `blink` | 즉시 켠 뒤 1초마다 상태 전환 | `[OK] LED ON (blink)` |
| `help` | 명령 목록 출력 | `Commands: on, off, blink, help` |

명령은 대소문자를 구분하지 않습니다. 잘못된 명령에는 `[ERROR] Unknown command. Use: on, off, blink, help`가 표시됩니다. 포트를 연 뒤 보드의 RESET을 누르면 시작 메시지를 다시 볼 수 있으며, 리셋 또는 전원 인가 시 기본 점멸로 돌아갑니다. 상태 및 명령 응답은 UART 일반 텍스트로 출력됩니다. 초기화나 전송에서 오류가 나면 `expect`가 panic을 발생시키고, panic 정보는 UART가 아닌 RTT로 출력됩니다.

## 구성 요소

- `embassy-stm32`: STM32의 핀과 주변장치를 사용하는 HAL(하드웨어 추상화 라이브러리)입니다. `stm32f411re` 기능을 선택했습니다.
- `embassy-executor`와 `embassy-time`: `async` 작업 및 타이머를 실행합니다. 현재 시간 드라이버가 타이머 하나를 사용하므로 이후 엔코더 등에서 타이머를 사용할 때 충돌하지 않도록 선택을 확인해야 합니다.
- `defmt`, `defmt-rtt`: 임베디드 로그를 만들고 ST-LINK RTT를 통해 전송합니다. `.cargo/config.toml`에서 `DEFMT_LOG`를 `info`로 설정했습니다.
- RTT는 `disable-blocking-mode`를 사용하므로 읽는 프로그램이 없거나 느리면 로그가 유실될 수 있지만 RTT 버퍼를 기다리며 멈추지 않습니다. UART 로그는 RTT와 별개입니다.
- `panic-probe`: 오류로 `panic!`이 발생했을 때 디버거와 로그에서 확인할 수 있도록 합니다.

## 점검 명령 (PowerShell)

```powershell
& "$env:USERPROFILE\.cargo\bin\rustup.exe" target list --installed
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build
& "$env:USERPROFILE\.cargo\bin\probe-rs.exe" list
```

ST-LINK가 검색되지 않으면 케이블과 연결, Windows 드라이버 및 장치 관리자 상태를 확인합니다. 여러 프로브가 연결되어 있으면 `probe-rs list`에서 확인한 식별자를 `.vscode/launch.json`의 `probe` 항목에 지정할 수 있습니다. 칩 종류가 STM32F411RE와 다르면 칩 이름과 `memory.x`를 해당 장치에 맞게 변경해야 합니다.

## GitHub 자동 검사

저장소 루트의 `.github/workflows/rust.yml`은 `push` 또는 Pull Request에서 포맷 검사, Clippy, STM32 빌드를 실행합니다. 직접 커밋하고 푸시해야 실행되며, 보드 업로드는 VS Code에서 수행합니다.

프로젝트 폴더의 PowerShell에서 다음과 같이 저장소 루트로 이동해 변경 사항을 확인하고 올릴 수 있습니다. `git add`는 지정한 프로젝트 폴더와 워크플로의 변경 사항을 스테이징하므로 먼저 `git status`로 내용을 확인하세요.

```powershell
cd ..
git status
git add -- 261107_prj_1 .github/workflows/rust.yml
git commit -m "Add Embassy logging and Rust CI"
git push
```

GitHub 저장소의 **Actions** 탭에서 `STM32 Rust checks` 실행 결과를 확인합니다.

참고: [probe-rs VS Code 디버거](https://probe.rs/docs/tools/debugger/) · [Rust 임베디드 타깃](https://doc.rust-lang.org/rustc/platform-support/thumbv7em-none-eabi.html)
