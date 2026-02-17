use assert_cmd::Command;
use predicates::prelude::*;

fn ms() -> Command {
    Command::cargo_bin("ms").unwrap()
}

#[test]
fn test_password_default() {
    ms().arg("password").assert().success().stdout(
        predicate::str::contains("copied to clipboard")
            .or(predicate::str::is_match(".{16}").unwrap()),
    );
}

#[test]
fn test_password_custom_length() {
    ms().args(["password", "8"]).assert().success();
}

#[test]
fn test_password_zero_length() {
    ms().args(["password", "0"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("greater than 0"));
}

#[test]
fn test_base64_encode() {
    ms().args(["base64", "encode", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("aGVsbG8="));
}

#[test]
fn test_base64_decode() {
    ms().args(["base64", "decode", "aGVsbG8="])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello"));
}

#[test]
fn test_url_encode() {
    ms().args(["url", "encode", "hello world"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello+world"));
}

#[test]
fn test_url_decode() {
    ms().args(["url", "decode", "hello+world"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello world"));
}

#[test]
fn test_url_parse() {
    ms().args(["url", "parse", "https://example.com?a=1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("example.com"));
}

#[test]
fn test_branch() {
    ms().args(["branch", "Fix: urgent bug"])
        .assert()
        .success()
        .stdout(predicate::str::contains("fix-urgent-bug"));
}

#[test]
fn test_date_add() {
    ms().args(["date", "add", "01/01/2024", "10"])
        .assert()
        .success()
        .stdout(predicate::str::contains("11/01/2024"));
}

#[test]
fn test_date_sub() {
    ms().args(["date", "sub", "11/01/2024", "10"])
        .assert()
        .success()
        .stdout(predicate::str::contains("01/01/2024"));
}

#[test]
fn test_hash_sha256() {
    ms().args(["hash", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2cf24dba5fb0a30e"));
}

#[test]
fn test_hash_md5() {
    ms().args(["hash", "hello", "md5"])
        .assert()
        .success()
        .stdout(predicate::str::contains("5d41402abc4b2a76"));
}

#[test]
fn test_json_pretty() {
    ms().args(["json", "pretty", r#"{"a":1}"#])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"a\": 1"));
}

#[test]
fn test_json_minify() {
    ms().args(["json", "minify", "{\n  \"a\": 1\n}"])
        .assert()
        .success()
        .stdout(predicate::str::contains("{\"a\":1}"));
}

#[test]
fn test_case_upper() {
    ms().args(["case", "upper", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("HELLO"));
}

#[test]
fn test_case_snake() {
    ms().args(["case", "snake", "helloWorld"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello_world"));
}

#[test]
fn test_uuid() {
    ms().arg("uuid")
        .assert()
        .success()
        .stdout(predicate::str::is_match("[0-9a-f-]{36}").unwrap());
}

#[test]
fn test_flatten() {
    ms().args(["flatten", "hello\nworld"])
        .assert()
        .success()
        .stdout(predicate::str::contains("helloworld"));
}

#[test]
fn test_regex() {
    ms().args(["regex", r"\d+", "abc123def456"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2 match"));
}

#[test]
fn test_qr() {
    ms().args(["qr", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("█"));
}

#[test]
fn test_color_hex() {
    ms().args(["color", "#ff0000"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rgb(255,0,0)"));
}

#[test]
fn test_filesize() {
    ms().args(["filesize", "1048576"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1.0 MB"));
}

#[test]
fn test_jwt() {
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.\
                 eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.\
                 SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
    ms().args(["jwt", token])
        .assert()
        .success()
        .stdout(predicate::str::contains("HS256"));
}

#[test]
fn test_epoch_now() {
    ms().args(["epoch", "now"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Epoch:"));
}

#[test]
fn test_epoch_timestamp() {
    ms().args(["epoch", "1700000000"])
        .assert()
        .success()
        .stdout(predicate::str::contains("UTC:"));
}

#[test]
fn test_inspect() {
    ms().args(["inspect", "hello world"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Bytes:")
                .and(predicate::str::contains("Characters:"))
                .and(predicate::str::contains("Words:")),
        );
}

#[test]
fn test_cron() {
    ms().args(["cron", "*/5 * * * *"])
        .assert()
        .success()
        .stdout(predicate::str::contains("every 5"));
}

#[test]
fn test_port_check() {
    ms().args(["port", "check", "19999"])
        .assert()
        .success()
        .stdout(predicate::str::contains("CLOSED"));
}

#[test]
fn test_convert_json_to_yaml() {
    ms().args(["convert", "yaml", r#"{"name":"test"}"#])
        .assert()
        .success()
        .stdout(predicate::str::contains("name: test"));
}

#[test]
fn test_completions_zsh() {
    ms().args(["completions", "zsh"]).assert().success();
}

#[test]
fn test_no_subcommand() {
    ms().assert().failure();
}

#[test]
fn test_version() {
    ms().arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.2.0"));
}
