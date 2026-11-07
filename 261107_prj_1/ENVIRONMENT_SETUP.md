# NUCLEO-F411RE Rust 개발 환경 재구성

이 문서는 **64비트 Windows PC**, ST **NUCLEO-F411RE**, 보드의 내장 ST-LINK와 데이터 USB 케이블을 기준으로 한다. VS Code에서 F5로 Embassy 펌웨어를 빌드·업로드·디버깅하고 RTT 로그를 확인하는 방법을 설명한다. Linux, macOS, WSL은 여기의 Windows 실행 파일 경로와 드라이버가 그대로 맞지 않으므로 별도 설정이 필요하다.

예제는 기본적으로 LD2(PA5)를 1초 켜고 1초 끄며, 시리얼 명령으로 LED를 제어할 수 있다. 점멸 상태가 1초마다 바뀌며 완전한 주기는 2초다. F5 업로드 시 보드에 있던 펌웨어는 덮어쓰인다.

## 기준 구성

| 항목 | 이 프로젝트의 구성 |
| --- | --- |
| MCU / 보드 | STM32F411RE / NUCLEO-F411RE |
| Rust 및 호스트 | Rust 1.99.1, `x86_64-pc-windows-gnu` |
| MCU 빌드 대상 | `thumbv7em-none-eabihf` |
| 디버그 도구 | probe-rs 0.32.0, VS Code probe-rs debugger 0.32.0 |
| 에디터 확장 | rust-analyzer 0.3.3105 |
| 펌웨어 | Embassy, defmt/RTT, panic-probe |
| 자동 검사 | GitHub Actions: rustfmt, Clippy, MCU 빌드 |

컴파일러와 의존성은 각각 Rust 1.99.1 및 저장소의 `Cargo.lock`을 기준으로 한다. CI도 Rust 1.99.1을 지정한다. 이 PC는 GNU Rust와 기존 `C:\mingw64\bin`을 사용했으며, 아래에서는 다른 PC에 MinGW를 준비하는 한 가지 방법으로 MSYS2를 사용한다. 도구나 확장 업데이트 후에는 동일 동작을 다시 확인해야 한다.

## 1. VS Code, Git, ST-LINK 드라이버

1. [VS Code](https://code.visualstudio.com/)와 [Git for Windows](https://git-scm.com/downloads/win)를 설치한다.
2. Windows에서 [ST 공식 ST-LINK 드라이버](https://www.st.com/en/development-tools/stsw-link009.html)를 설치한다. 보드를 ST-LINK 쪽 USB 포트에 데이터 케이블로 연결한다.
3. VS Code의 Extensions에서 **rust-analyzer** (`rust-lang.rust-analyzer`)와 **probe-rs debugger** (`probe-rs.probe-rs-debugger`)를 설치한다. 필요하면 각 확장 메뉴의 **Install Another Version**에서 위 기준 버전을 선택한다.

기준 버전의 확장이 설치 가능한 경우 새 PowerShell에서 다음 명령도 사용할 수 있다.

```powershell
code --install-extension rust-lang.rust-analyzer@0.3.3105
code --install-extension probe-rs.probe-rs-debugger@0.32.0
```

`code` 명령을 찾을 수 없다면 VS Code 설치 시 PATH 추가 옵션을 확인하거나 확장 화면에서 설치한다.

## 2. Windows GNU 링커 준비

Rust 크레이트에는 PC에서 먼저 실행되는 빌드 스크립트와 매크로가 있다. 따라서 MCU용 코드만 빌드하더라도 Windows 호스트용 링커가 필요하다. 아래는 Windows GNU 호스트 도구 모음을 사용하는 설정이다.

1. [MSYS2](https://www.msys2.org/)를 기본 위치 `C:\msys64`에 설치한다.
2. 시작 메뉴에서 **MSYS2 MINGW64**를 연다. 아래 명령은 PowerShell이 아니라 **MSYS2 터미널**에서 실행한다.

```bash
pacman -Syu
```

업데이트 중 터미널 종료를 요청하면 종료한 뒤 MSYS2 MINGW64를 다시 열어 `pacman -Syu`를 완료한다. 그다음 같은 터미널에서 다음 패키지를 설치한다.

```bash
pacman -S mingw-w64-x86_64-gcc
```

3. Windows의 **사용자 환경 변수 Path**에 `C:\msys64\mingw64\bin`을 추가한다. 다른 위치에 설치했다면 그 위치의 `mingw64\bin`을 넣는다. 기존 PATH 항목은 삭제하지 않는다.
4. PowerShell과 VS Code를 모두 종료하고 다시 열어 `gcc -dumpmachine`을 실행한다. `x86_64-w64-mingw32`가 나와야 한다. 기존 GCC가 먼저 검색된다면 `Get-Command gcc`로 경로를 확인한다.

참고: [MSYS2 GCC 패키지](https://packages.msys2.org/packages/mingw-w64-x86_64-gcc) · [Rust Windows GNU 도구 모음](https://rust-lang.github.io/rustup/installation/windows.html)

## 3. Rust와 STM32 타깃 설치

**Rust가 아직 없는 PC**에서는 [Rust 공식 설치 페이지](https://www.rust-lang.org/tools/install)에서 64비트 Windows용 `rustup-init.exe`를 내려받는다. 다운로드 폴더의 PowerShell에서 GNU 호스트로 설치한다.

```powershell
cd "$env:USERPROFILE\Downloads"
.\rustup-init.exe -y --default-host x86_64-pc-windows-gnu --default-toolchain 1.99.1 --profile minimal
```

설치 후 PowerShell을 새로 연다. 이미 Rust가 있다면 기존 기본 도구 모음을 바꾸지 않고 아래 단계에서 이 프로젝트에만 GNU 1.99.1을 지정할 수 있다.

```powershell
rustup toolchain install 1.99.1-x86_64-pc-windows-gnu --profile minimal --component rustfmt --component clippy --target thumbv7em-none-eabihf
```

Rust 실행 파일은 기본적으로 `%USERPROFILE%\.cargo\bin`에 설치된다. 이 프로젝트의 VS Code 설정은 해당 위치를 사용하므로 `CARGO_HOME`을 별도로 바꾸었다면 `.vscode/tasks.json`과 `.vscode/launch.json`의 경로를 수정해야 한다.

## 4. probe-rs 설치

새 PowerShell에서 [공식 probe-rs 0.32.0 릴리스](https://github.com/probe-rs/probe-rs/releases/tag/v0.32.0)의 Windows 설치 명령을 실행한다.

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/probe-rs/probe-rs/releases/download/v0.32.0/probe-rs-tools-installer.ps1 | iex"
```

설치 도구는 `probe-rs.exe`를 기본적으로 `%USERPROFILE%\.cargo\bin`에 둔다. 설치 후 새 PowerShell에서 확인한다.

```powershell
probe-rs --version
probe-rs list
```

`probe-rs list`에 ST-LINK가 나오면 USB 디버거가 인식된 것이다. 장치 인식과 MCU 통신은 별개이므로 이후 F5 테스트도 확인해야 한다.

## 5. 저장소 받기와 빌드

원본 PC의 변경 사항은 먼저 GitHub에 커밋·푸시되어야 다른 PC에서 받을 수 있다. 아래는 저장소가 아직 없는 PC의 예이며, GitHub 접근 권한이 필요하다.

```powershell
cd $env:USERPROFILE
git clone --branch develop https://github.com/red-crypto/Red_STM32_Project.git STM32_Project
cd STM32_Project\261107_prj_1
rustup override set 1.99.1-x86_64-pc-windows-gnu
rustup show active-toolchain
rustup target list --installed
rustc --version
gcc -dumpmachine
cargo build --locked
```

`rustup override set`은 이 프로젝트 경로에만 도구 모음을 지정한다. `rustup show active-toolchain`은 GNU 1.99.1을, 설치된 타깃 목록은 `thumbv7em-none-eabihf`를 표시해야 한다. `cargo build --locked`는 `Cargo.lock`을 변경하지 않고 의존성을 확인해 빌드한다. 첫 빌드에는 인터넷이 필요하다.

다른 PC에서 이미 저장소를 복제했다면 해당 저장소의 `develop`을 업데이트한 뒤 프로젝트 폴더에서 같은 override와 빌드 명령을 실행한다. 다른 PC에 복사하는 경우에도 `.cargo`, `.vscode`, `Cargo.lock`, `memory.x`와 저장소 루트의 `.github`를 포함한다. `target` 빌드 결과물은 복사할 필요가 없다.

## 6. VS Code F5와 RTT 로그

1. 실행 중인 VS Code를 모두 닫고 다시 연다. **File → Open Folder**에서 저장소 전체가 아니라 `STM32_Project\261107_prj_1` 폴더를 연다. 설정에서 `${workspaceFolder}`를 이 폴더로 사용한다.
2. 보드를 연결한 상태에서 **F5**를 눌러 `STM32F411RE: 빌드, 플래시, 디버그`를 실행한다. 빌드 후 플래시에 기록하고 리셋 위치에서 멈춘다.
3. **F5를 다시 눌러 계속 실행**한다. LD2가 1초 켜지고 1초 꺼지는지 본다. VS Code의 **Terminal → defmt** 탭에서 `LD2 켜짐`과 `LD2 꺼짐` 로그가 번갈아 표시된다. RTT 로그는 ST-LINK 디버그 연결을 사용하며 COM 포트의 UART 로그가 아니다.
4. 중단점은 `src/main.rs`의 `led.set_high();` 줄 왼쪽 여백을 클릭해 설정한다. 중단점에서 멈추면 F10으로 단계 실행하고, F5로 계속한다. 로그와 LED가 계속 동작하게 보려면 중단점을 해제하거나 비활성화한다. **Shift+F5**는 디버그 세션을 종료한다.

### F5 없이 UART 로그와 LED 명령 사용

이 예제는 RTT 외에도 USART2의 **PA2(TX)**에서 일반 텍스트 로그를 보내고 **PA3(RX)**로 PC 명령을 받는다. NUCLEO-F411RE의 기본 하드웨어 설정에서는 두 신호가 내장 ST-LINK의 가상 COM 포트로 연결된다. PC에는 ST-LINK USB 데이터 케이블 하나를 연결하면 되고 별도의 USB-UART 장치는 필요하지 않다. 이 프로젝트는 **115200 baud, 데이터 8비트, 패리티 없음, 정지 비트 1개(8N1), 흐름 제어 없음**을 사용한다. 보드의 기본 연결이 바뀌었을 때는 [ST Nucleo-64 매뉴얼](https://www.st.com/resource/en/user_manual/dm00105823.pdf)의 USART 연결과 솔더 브리지 설명을 확인한다.

1. 펌웨어를 한 번 업로드한 뒤 디버거를 종료한다. 보드가 디버거에서 멈춘 상태라면 보드의 **RESET** 버튼을 누르거나 전원을 다시 넣어 독립 실행한다.
2. 새 PC에서는 VS Code Extensions에서 **Web and Desktop Serial Monitor** (`eclipse-cdt.serial-monitor`)를 설치한다. 명령줄에서는 `code --install-extension eclipse-cdt.serial-monitor`를 사용할 수 있다.
3. Windows 장치 관리자 **포트(COM 및 LPT)**에서 `STMicroelectronics STLink Virtual COM Port (COMn)`의 번호를 확인한다. 이 PC에서는 확인 당시 COM6이었지만 PC나 USB 연결에 따라 번호가 바뀔 수 있다.
4. VS Code에서 **Ctrl+Shift+P → Open Serial**을 실행하고 해당 COM 포트를 선택해 **Connect**를 누른 뒤 **115200**을 선택한다. 시리얼 모니터 터미널에 LED 상태 로그가 출력되는지 확인한다.
5. 터미널에 `on`, `off`, `blink`, `help` 중 하나를 입력하고 **Enter**를 누른다. 입력 문자는 즉시 전송되고 Enter가 명령을 완료한다. CR, LF 및 CRLF 줄바꿈을 지원한다.
6. 시작 메시지는 터미널을 열기 전에 지나갈 수 있다. 처음 메시지부터 보려면 모니터를 연 뒤 **RESET** 버튼을 누른다.

`on`은 LED를 켠 채 유지하고 `off`는 끈 채 유지한다. `blink`는 즉시 켜고 1초마다 상태를 전환하며, `help`는 명령 목록을 출력한다. `on`/`off`의 응답은 각각 `[OK] LED ON (manual)` / `[OK] LED OFF (manual)`이며, 잘못된 명령은 오류를 출력한다. 명령은 대소문자를 구분하지 않는다. RESET 또는 전원 재인가 시 기본 점멸로 돌아간다.

COM 포트는 보통 한 프로그램만 열 수 있다. 다른 시리얼 터미널이 사용 중이라면 먼저 닫는다. RTT의 `defmt` 터미널과 COM UART 터미널은 별개의 출력이다. 외부 전원만 연결하여 PC가 없으면 LED는 동작할 수 있지만 COM 로그를 표시할 PC도 없다. UART 출력은 모니터 연결 전후의 로그를 장기간 저장하지 않는다.

## 7. 로컬 검사와 GitHub Actions

프로젝트 폴더에서 아래 세 검사를 실행할 수 있다.

```powershell
cargo fmt --all -- --check
cargo clippy --locked
cargo build --locked
```

저장소 **루트**의 `.github/workflows/rust.yml`은 push와 Pull Request에서 같은 검사를 실행한다. 자동 커밋·푸시는 하지 않으며 GitHub에 보드가 연결된 것도 아니므로 실제 보드 확인은 이 PC에서 한다. 변경 내용을 확인한 뒤 직접 올린다.

```powershell
cd ..
git status
git add -- 261107_prj_1 .github/workflows/rust.yml
git commit -m "Update STM32 firmware"
git push
```

`git add`는 지정한 폴더의 모든 변경을 스테이징하므로 먼저 `git status`를 검토한다. 프로젝트 하위 폴더에서 `git add .`만 실행하면 상위 `.github` 파일은 포함되지 않는다. 푸시 후 GitHub 저장소의 **Actions → STM32 Rust checks**에서 결과를 확인한다. 새 브랜치에 upstream이 없다면 최초 푸시 시 `git push -u origin develop`처럼 실제 브랜치 이름을 지정한다.

## 문제가 생기면

- **`cargo`, `probe-rs`, `gcc` 명령을 찾지 못함**: 설치 경로와 사용자 PATH를 확인하고 PowerShell과 VS Code를 완전히 다시 연다.
- **`link.exe`를 찾지 못함**: MSVC 호스트가 선택됐을 수 있다. 프로젝트 폴더에서 `rustup show active-toolchain`을 확인하고 위의 GNU override를 적용한다. GNU 설정에서는 MSYS2의 `gcc`도 PATH에 있어야 한다.
- **ST-LINK가 검색되지 않음**: 충전 전용이 아닌 데이터 케이블, 보드의 ST-LINK USB 포트, ST 공식 드라이버와 장치 관리자를 확인한다. 다른 디버거가 점유 중이면 해당 세션을 종료한다. [probe-rs ST-LINK 안내](https://probe.rs/docs/getting-started/probe-setup/)
- **로그가 없거나 LED가 멈춤**: 플래시 후 리셋 위치나 중단점에서 일시 정지됐는지 보고 F5로 계속한다. 필요하면 중단점을 비활성화하고 `defmt` 터미널을 선택한다.
- **COM 포트에 UART 로그가 없음**: 올바른 ST-LINK 가상 COM 포트와 115200 baud를 선택했는지, 다른 프로그램이 포트를 사용 중이지 않은지 확인한다. 펌웨어가 멈춘 경우 RESET을 누르고, 보드의 USART2 기본 연결이 변경됐다면 ST 매뉴얼의 솔더 브리지를 확인한다.
- **다른 보드를 사용함**: STM32F411RE 칩이라도 LED 핀은 보드마다 다를 수 있으므로 회로도를 확인하고 `src/main.rs`의 핀과 LED 극성을 수정한다. MCU가 다르면 타깃, 칩 설정, 메모리 크기, Embassy 칩 feature도 다시 확인한다.

Embassy의 `time-driver-any`는 MCU 타이머 하나를 사용한다. 나중에 엔코더 등에서 하드웨어 타이머가 필요해지면 시간 드라이버와 중복 사용하지 않도록 배정을 확인한다.
