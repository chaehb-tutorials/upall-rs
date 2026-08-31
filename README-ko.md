# upall-rs (한국어 설명서)

Rust로 작성된 가볍고 고성능의 리눅스 시스템 업데이트 자동화 CLI 도구입니다. 패키지 관리자, 환경 관리자, 컴파일러 툴체인의 업데이트를 하나의 단계로 통합하고 대화형 팝업 창으로 인한 작업 중단을 방지합니다.

🌐 **English**: [For English documentation, please refer to README.md](./README.md)

---

## 주요 기능
- **인터럽트 없는 APT 업데이트**: `NEEDRESTART_MODE=a` 환경 변수를 주입하여 `apt upgrade` 진행 시 보라색 대화형 서비스 재시작 팝업창이 뜨는 것을 원천 차단합니다.
- **자동 디스크 청소**: `apt autoremove --purge` 및 `apt clean`을 순서대로 실행하여 오래된 리눅스 커널 유산 파일과 캐시를 정리하고 `/boot` 디렉토리 용량을 안전하게 확보합니다.
- **`mise` 에코시스템 최신화**: `mise` 자체 self-update를 수행한 뒤, 연동된 모든 개발 도구(`nodejs`, `elixir`, `erlang` 등)를 업그레이드하고 `mise prune`으로 불필요한 구버전 찌꺼기 파일까지 일괄 청소합니다.
- **`rustup` 인프라 최신화**: 글로벌 Rust 컴파일러와 타깃 툴체인을 최신 상태로 원격 업데이트합니다.
- **동적 다국어 지원 (i18n)**: 시스템 환경 변수(`$LANG`)를 자동으로 추적하여 터미널 출력 로그를 한국어와 영어로 자동 전환합니다.
- **단일 독립 바이너리**: 다국어 로케일 파일(`yml`)이 빌드 시점에 바이너리 내부로 내장(Embed)되므로, 파일 한 개만 들고 다니며 즉시 실행할 수 있습니다.

## 지원 배포판
- Ubuntu (라즈베리파이 4B용 Server 26.04 버전 포함)
- Debian
- MX Linux (SysVinit 시스템 환경에서도 완벽 호환)

## 사용 방법

### 버전 확인
```bash
upall --version
# 출력: upall version 0.2.0
```

### 통합 업데이트 실행
```bash
upall
```

## 수동 컴파일 및 설치 방법

### 필수 의존성 설치
```bash
sudo apt update && sudo apt install -y build-essential pkg-config libssl-dev
```

### 로컬 컴파일 (현재 PC용)
```bash
cargo build --release
mkdir -p ~/.local/bin
cp target/release/upall-rs ~/.local/bin/upall
```

### 라즈베리파이 4B (AArch64)용 교차 컴파일
1. 빌드를 진행할 호스트 PC에 타깃 및 링커 설치:
   ```bash
   rustup target add aarch64-unknown-linux-gnu
   sudo apt install gcc-aarch64-linux-gnu
   ```
2. `.cargo/config.toml` 설정 추가:
   ```toml
   [target.aarch64-unknown-linux-gnu]
   linker = "aarch64-linux-gnu-gcc"
   ```
3. 컴파일 명령어 실행:
   ```bash
   cargo build --release --target aarch64-unknown-linux-gnu
   ```

## 라이선스
MIT 라이선스에 따라 배포됩니다.
