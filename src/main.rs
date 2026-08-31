use colored::*;
use std::process::{Command, Stdio};

/// 시스템 명령어를 실행하고 결과를 터미널에 실시간으로 출력하는 함수
fn run_command(cmd: &str, args: &[&str], envs: &[(&str, &str)]) -> Result<(), String> {
    let mut command = Command::new(cmd);
    command.args(args);

    // 환경 변수 세팅 (예: NEEDRESTART_MODE)
    for (key, val) in envs {
        command.env(key, val);
    }

    // 표준 입출력을 현재 터미널에 그대로 연결
    command.stdin(Stdio::inherit());
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());

    match command.status() {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(format!("명령어가 에러 코드를 반환했습니다: {}", status)),
        Err(e) => Err(format!("명령어 실행 실패 (설치 여부 확인 필요): {}", e)),
    }
}

/// 특정 명령어가 시스템에 설치되어 있는지 확인하는 함수
fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn main() {
    println!("{}", "==========================================".cyan());
    println!(
        "{}",
        "    Rust 기반 통합 시스템 업데이트 시작     ".bold().cyan()
    );
    println!("{}", "==========================================".cyan());

    // 1. APT 업데이트
    println!("\n{}", "[/] APT 시스템 패키지 업데이트 중...".yellow());
    // 주입할 환경변수가 없을 때는 빈 슬라이스 참조 &[] 를 명시해야 합니다.
    if let Err(e) = run_command("sudo", &["apt", "update"], &[]) {
        eprintln!("{} {}", "적색경보: apt update 실패 ->".red(), e);
    }

    // NEEDRESTART_MODE=a 환경변수를 주어 인터럽트(팝업) 방지
    if let Err(e) = run_command(
        "sudo",
        &["apt", "upgrade", "-y"],
        &[("NEEDRESTART_MODE", "a")],
    ) {
        eprintln!("{} {}", "적색경보: apt upgrade 실패 ->".red(), e);
    }

    // 안쓰는 패키지 정리
    let _ = run_command("sudo", &["apt", "autoremove", "--purge", "-y"], &[]);
    let _ = run_command("sudo", &["apt", "clean"], &[]);

    // 2. MISE 업데이트
    println!("\n{}", "[/] mise 및 설치된 도구 업데이트 중...".yellow());
    if command_exists("mise") {
        println!("-> mise self-update 실행");
        let _ = run_command("mise", &["self-update", "--yes"], &[]);

        println!("-> mise 플러그인 및 도구 업그레이드");
        let _ = run_command("mise", &["upgrade", "--yes"], &[]);
    } else {
        println!("{}", "-> 시스템에 mise가 없어 건너뜁니다.".purple());
    }

    // 3. RUSTUP 업데이트
    println!("\n{}", "[/] rustup 및 Rust 툴체인 업데이트 중...".yellow());
    if command_exists("rustup") {
        let _ = run_command("rustup", &["update"], &[]);
    } else {
        println!("{}", "-> 시스템에 rustup이 없어 건너뜁니다.".purple());
    }

    println!("\n{}", "==========================================".green());
    println!(
        "{}",
        "     모든 업데이트가 완료되었습니다!      ".bold().green()
    );
    println!("{}", "==========================================".green());
}
