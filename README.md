# Bettercap UI

Bettercap을 제어하는 Linux 데스크톱 애플리케이션. Tauri v2 + React + TypeScript 기반.

## Architecture

```
┌─ Kali Linux ───────────────────────────┐
│                                         │
│  wlan1 (외장 어댑터)                     │
│    │                                    │
│  bettercap (REST :8081 + WebSocket)     │
│    ▲                                    │
│    │ localhost                           │
│    ▼                                    │
│  Bettercap UI (Tauri native app)        │
│    ├── Rust: bettercap API client       │
│    └── React: 실시간 UI                  │
└─────────────────────────────────────────┘
```

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
# 1. bettercap API 서버 실행
sudo bettercap -iface wlan1 \
  -api-rest-address 127.0.0.1 \
  -api-rest-port 8081 \
  -api-rest-username admin \
  -api-rest-password admin

# 2. 앱 실행
bettercap-ui
```

## Features

- **Network Scan** — net.probe 기반 네트워크 스캔 및 호스트 탐지
- **Host List** — 발견된 호스트 실시간 테이블 (IP, MAC, Vendor)
- **Event Log** — bettercap WebSocket 이벤트 실시간 스트림
- **ARP Spoof** — 타겟 지정 ARP 스푸핑 제어

## Development

```bash
# 의존성 설치
sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 프로젝트 빌드
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
