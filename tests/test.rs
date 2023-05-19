use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn command_line_required_option() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    // 引数なし
    cmd.arg("--debug").assert().failure();

    // 実行可能パターン
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug").args(&["CQ CQ"]).assert().success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug").args(&["--pipe"]).assert().success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--input", "tests/data/sample.txt"])
        .assert()
        .success();

    // オプション追加
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&[
            "--wpm",
            "20",
            "--volume",
            "1",
            "--frequency",
            "600",
            "--power",
            "2.5",
            "--dump",
            "char",
        ])
        .args(&["CQ CQ"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&[
            "--wpm",
            "20",
            "--volume",
            "1",
            "--frequency",
            "600",
            "--power",
            "2.5",
            "--dump",
            "char",
        ])
        .args(&["--pipe"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&[
            "--wpm",
            "20",
            "--volume",
            "1",
            "--frequency",
            "600",
            "--power",
            "2.5",
            "--dump",
            "char",
        ])
        .args(&["--input", "tests/data/sample.txt"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&[
            "--wpm",
            "20",
            "--volume",
            "1",
            "--frequency",
            "600",
            "--power",
            "2.5",
            "--dump",
            "char",
        ])
        .assert()
        .failure();

    // グループ内排他確認
    cmd.args(&["--pipe", "CQ CQ"]).assert().failure();

    cmd.args(&["--input", "tests/data/sample.txt", "CQ CQ"])
        .assert()
        .failure();

    cmd.args(&["--pipe", "--input", "tests/data/sample.txt"])
        .assert()
        .failure();

    // ３つ指定
    cmd.args(&["--pipe", "--input", "tests/data/sample.txt", "CQ CQ"])
        .assert()
        .failure();

    Ok(())
}

#[test]
fn boundary_value_test_wpm() -> Result<(), Box<dyn std::error::Error>> {
    // 範囲内
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--wpm", "20"])
        .args(&["CQ CQ"])
        .assert()
        .success();

    // 境界上
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--wpm", "3"])
        .args(&["CQ CQ"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--wpm", "60"])
        .args(&["CQ CQ"])
        .assert()
        .success();

    // 範囲外
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--wpm", "2"])
        .args(&["CQ CQ"])
        .assert()
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--wpm", "61"])
        .args(&["CQ CQ"])
        .assert()
        .failure();

    Ok(())
}

#[test]
fn boundary_value_test_frequency() -> Result<(), Box<dyn std::error::Error>> {
    // 範囲内
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--frequency", "600"])
        .args(&["CQ CQ"])
        .assert()
        .success();

    // 境界上
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--frequency", "400"])
        .args(&["CQ CQ"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--frequency", "1200"])
        .args(&["CQ CQ"])
        .assert()
        .success();

    // 範囲外
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--frequency", "399"])
        .args(&["CQ CQ"])
        .assert()
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--frequency", "1201"])
        .args(&["CQ CQ"])
        .assert()
        .failure();

    Ok(())
}

#[test]
fn boundary_value_test_volume() -> Result<(), Box<dyn std::error::Error>> {
    // 範囲内
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--volume", "0.5"])
        .args(&["CQ CQ"])
        .assert()
        .success();

    // 境界上
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--volume", "0.001"])
        .args(&["CQ CQ"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--volume", "1"])
        .args(&["CQ CQ"])
        .assert()
        .success();

    // 範囲外
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--volume", "0"])
        .args(&["CQ CQ"])
        .assert()
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--volume", "1.1"])
        .args(&["CQ CQ"])
        .assert()
        .failure();

    Ok(())
}

#[test]
fn boundary_value_test_power() -> Result<(), Box<dyn std::error::Error>> {
    // 範囲内
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--power", "2.5"])
        .args(&["CQ CQ"])
        .assert()
        .success();

    // 境界上
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--power", "1"])
        .args(&["CQ CQ"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--power", "5"])
        .args(&["CQ CQ"])
        .assert()
        .success();

    // 範囲外
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--power", "0.9"])
        .args(&["CQ CQ"])
        .assert()
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--power", "5.1"])
        .args(&["CQ CQ"])
        .assert()
        .failure();

    Ok(())
}

#[test]
fn boundary_value_test_input() -> Result<(), Box<dyn std::error::Error>> {
    // 存在するファイルを指定
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--input", "tests/data/sample.txt"])
        .assert()
        .success();

    // 存在しないファイルを指定
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--input", "tests/data/sample.tx"])
        .assert()
        .failure();

    // 誤ったパス名を指定
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("--debug")
        .args(&["--input", "tests/data/sample!.txt"])
        .assert()
        .failure();

    Ok(())
}
