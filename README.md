# Bettercap UI

Bettercap을 제어하는 Linux 데스크톱 애플리케이션. Tauri v2 + React + TypeScript 기반.

## Architecture

```
┌─ Kali Linux ───────────────────────────┐
│                                         │
│  Bettercap UI (Tauri native app)        │
│    ├── Settings: 인터페이스/포트 설정    │
│    ├── bettercap 프로세스 시작/중지      │
│    │       │                            │
│    │       ▼                            │
│    │   bettercap (subprocess)           │
│    │   REST :8081 + WebSocket           │
│    │       ▲                            │
│    ├── Rust: API client (localhost)     │
│    └── React: 실시간 UI                 │
│                                         │
│  wlan1 (외장 어댑터)                     │
└─────────────────────────────────────────┘
```

앱이 bettercap 프로세스를 직접 관리합니다. 별도 터미널 실행 불필요.

## Install

```bash
curl -sSL https://raw.githubusercontent.com/LimSeongHyeon/bettercap-ui/main/install.sh | sudo bash
```

또는 [Releases](https://github.com/LimSeongHyeon/bettercap-ui/releases)에서 `.deb` 파일을 직접 다운로드:

```bash
sudo dpkg -i bettercap-ui_*.deb
sudo apt-get install -f
```

## Usage

```bash
bettercap-ui
```

1. Settings에서 네트워크 인터페이스, API 포트, 인증정보 설정
2. **Start Bettercap** 클릭
3. 스캔, 호스트 목록, 이벤트 로그 사용

설정값은 `~/.config/bettercap-ui/config.json`에 자동 저장됩니다.

## Features

- **Process Control** — 앱에서 bettercap 프로세스 시작/중지
- **Settings** — 인터페이스, API 주소/포트, 인증정보 설정 및 영구 저장
- **Network Scan** — net.probe 기반 네트워크 스캔 및 호스트 탐지
- **Host List** — 발견된 호스트 실시간 테이블 (IP, MAC, Vendor)
- **Event Log** — bettercap WebSocket 이벤트 실시간 스트림
- **ARP Spoof** — 타겟 지정 ARP 스푸핑 제어

## Development

```bash
# 시스템 의존성
sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 빌드 및 실행
git clone https://github.com/LimSeongHyeon/bettercap-ui.git
cd bettercap-ui
npm install
cargo tauri dev
```

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Runtime | Tauri v2 (Rust) |
| Frontend | React 19 + TypeScript |
| Bundler | Vite 6 |
| Package | .deb (Debian/Kali) |

## License

TBD
