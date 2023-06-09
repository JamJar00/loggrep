use assert_cmd::Command;

#[test]
fn test_nginx_information() {
    // GIVEN nginx log lines
    let logs = std::fs::read_to_string("tests/files/nginx/input.txt").unwrap();

    // WHEN getting information about it
    let mut cmd = Command::cargo_bin("loggrep").unwrap();
    cmd.write_stdin(logs);

    // THEN it is detected as nginx
    cmd.assert()
        .success()
        .stdout(predicates::path::eq_file("tests/files/nginx/information.txt").utf8().unwrap());
}

#[test]
fn test_syslog_bsd_information() {
    // GIVEN syslog BSD log lines
    let logs = std::fs::read_to_string("tests/files/syslog-bsd/input.txt").unwrap();

    // WHEN getting information about it
    let mut cmd = Command::cargo_bin("loggrep").unwrap();
    cmd.write_stdin(logs);

    // THEN it is detected as syslog-bsd
    cmd.assert()
        .success()
        .stdout(predicates::path::eq_file("tests/files/syslog-bsd/information.txt").utf8().unwrap());
}
