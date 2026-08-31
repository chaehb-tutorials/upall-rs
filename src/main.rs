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

    // 1. APT 업데이트 및 관리
    println!("\n{}", "[/] 1. APT 시스템 패키지 업데이트 진행".yellow());
    if let Err(e) = run_command("sudo", &["apt", "update"], &[]) {
        eprintln!("{} {}", "적색경보: apt update 실패 ->".red(), e);
    }

    // NEEDRESTART_MODE=a 환경변수를 주어 대화형 인터럽트(팝업창) 방지
    if let Err(e) = run_command(
        "sudo",
        &["apt", "upgrade", "-y"],
        &[("NEEDRESTART_MODE", "a")],
    ) {
        eprintln!("{} {}", "적색경보: apt upgrade 실패 ->".red(), e);
    }

    // 💡 [통합된 부분] 안쓰는 패키지, 설정 파일 및 이전 커널 청소 (--purge 옵션으로 잔여물 제거)
    println!("-> 안 쓰는 패키지 및 구버전 커널 청소 중 (autoremove)");
    let _ = run_command("sudo", &["apt", "autoremove", "--purge", "-y"], &[]);

    println!("-> 로컬 저장소 패키지 캐시 정리 (clean)");
    let _ = run_command("sudo", &["apt", "clean"], &[]);

    // 2. MISE 업데이트 & 지난 버전 정리 (prune)
    println!("\n{}", "[/] 2. mise 및 설치된 도구 업데이트 진행".yellow());
    if command_exists("mise") {
        println!("-> mise self-update 실행 (자체 최신화)");
        let _ = run_command("mise", &["self-update", "--yes"], &[]);

        println!("-> mise 플러그인 및 개발 도구 업그레이드");
        let _ = run_command("mise", &["upgrade", "--yes"], &[]);

        println!("-> mise 미사용 과거 유산 버전 정리 (prune)");
        let _ = run_command("mise", &["prune", "--yes"], &[]);
    } else {
        println!("{}", "-> 시스템에 mise가 없어 건너뜁니다.".purple());
    }

    // 3. RUSTUP 업데이트
    println!(
        "\n{}",
        "[/] 3. rustup 및 Rust 툴체인 업데이트 진행".yellow()
    );
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
