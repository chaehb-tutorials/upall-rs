# upall-rs

A lightweight, high-performance Linux system update automation CLI tool written in Rust. It consolidates package managers, environment managers, and compiler toolchains into a single execution step, preventing interactive prompt dropouts.

🌐 **Translations**: [한국어 설명서는 README-ko.md를 참고하세요.](./README-ko.md)

---

## Key Features
- **Interruption-Free APT**: Injects `NEEDRESTART_MODE=a` to bypass interactive purple prompt dialogs during `apt upgrade`.
- **Automated Purging**: Runs `apt autoremove --purge` and `apt clean` automatically to ensure the `/boot` partition or root drive stays clean from historical kernels and cached remnants.
- **`mise` Ecosystem Management**: Seamlessly self-updates `mise`, upgrades all pinned development tools (`nodejs`, `elixir`, `erlang`, etc.), and purges stale historical runtimes with `mise prune`.
- **`rustup` Continuous Delivery**: Auto-updates the global Rust compiler infrastructure and target toolchains.
- **Dynamic Localization (i18n)**: Auto-detects system `$LANG` to seamlessly switch internal CLI logs between English and Korean.
- **Single Static Binary**: Compiles down to an independent binary containing embedded localization layouts with zero shared library clutter.

## Supported Distributions
- Ubuntu (including Server 26.04 for Raspberry Pi 4B)
- Debian
- MX Linux (Fully compatible with SysVinit architectures)

## Usage

### Print Version
```bash
upall --version
# Output: upall version 0.2.0
```

### Run Full Update Pipeline
```bash
upall
```

## Manual Compilation & Installation

### Prerequisites
```bash
sudo apt update && sudo apt install -y build-essential pkg-config libssl-dev
```

### Local Compilation (Native Target)
```bash
cargo build --release
mkdir -p ~/.local/bin
cp target/release/upall-rs ~/.local/bin/upall
```

### Cross-Compilation for Raspberry Pi 4B (AArch64)
1. Add target & linker tooling on your Host PC:
   ```bash
   rustup target add aarch64-unknown-linux-gnu
   sudo apt install gcc-aarch64-linux-gnu
   ```
2. Configure `.cargo/config.toml`:
   ```toml
   [target.aarch64-unknown-linux-gnu]
   linker = "aarch64-linux-gnu-gcc"
   ```
3. Compile:
   ```bash
   cargo build --release --target aarch64-unknown-linux-gnu
   ```

## License
Distributed under the MIT License.
