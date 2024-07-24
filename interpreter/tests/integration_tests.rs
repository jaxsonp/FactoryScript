use assert_cmd::Command;
use assert_fs::{prelude::*, NamedTempFile};

static BIN_NAME: &str = "factory";

#[test]
fn test_file_not_found() {
    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    cmd.arg("this_file_does_not_exist.factory");
    cmd.assert().failure();
}

#[test]
fn test_hello_world() {
    let file = NamedTempFile::new("tmp").unwrap();
    file.write_str("[start]═─{\"hello world\"}═─[println]")
        .unwrap();
    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    cmd.arg(file.path());
    cmd.assert().success().stdout("hello world\n");
}

/// for robustness :D
#[test]
fn test_hello_world_fancy() {
    let file = NamedTempFile::new("tmp").unwrap();
    file.write_str(
        "
[start]═─{\"hello world\"} [println]
┌────────╝                 └─────────────────────────────────┐
│  ┌┐  ┌┐       ┌┐ ┌┐          ┌┐  ┌┐              ┌┐    ┌┐  │
│  ││  ││ ┌───┐ ││ ││          ││  ││        ┌┐    ││    ││  │
│  │└──┘│ │ # │ ││ ││ ┌────┐   ││┌┐││ ┌────┐ │└──┐ ││ ┌──┘│  │
│  │┌──┐│ │┌──┘ ││ ││ │ /\\ │   ││││││ │ /\\ │ │┌─┐│ ││ │| |│  │
│  ││  ││ │└──┐ ││ ││ │ \\/ │   │└┘└┘│ │ \\/ │ ││ └┘ ││ │|_|│  │
│  ││  └┘ └┐┌─┘ ││ ││ └┐┌──┘   └┐┌──┘ └┐┌──┘ ││    ││ └┐┌─┘  │
└──┘└──────┘└───┘└─┘└──┘└───────┘└─────┘└────┘└────┘└──┘└────┘",
    )
    .unwrap();
    let mut cmd = Command::cargo_bin("factory").unwrap();
    cmd.arg(file.path());
    cmd.assert().success().stdout("hello world\n");
}

#[test]
fn test_hello_world_reverse() {
    let file = NamedTempFile::new("tmp").unwrap();
    file.write_str("[println]─═{\"hello world\"}─═[start]")
        .unwrap();
    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    cmd.arg(file.path());
    cmd.assert().success().stdout("hello world\n");
}

#[test]
fn test_int_arithmetic() {
    let file = NamedTempFile::new("tmp").unwrap();
    file.write_str(
        "
[start]═─{1}═─[]═─[+]═─[]═──[-]═─[]═──[*]═─[]═────[/]═─[println]
              ╚───┘    ║╚─{6}═┘  ║╚─{2}═┘  ║╚─{128}═┘
                       │         │         │
                      [println] [println] [println]",
    )
    .unwrap();
    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    cmd.arg(file.path());
    cmd.assert().stdout("2\n4\n8\n16\n");
}

#[test]
fn test_float_arithmetic() {
    let file = NamedTempFile::new("tmp").unwrap();
    file.write_str(
        "
[start]═─{1.25}═─[]═─[+]═─[]═───[-]═─[]═───[*]═─[]═─────[/]═─[println]
                  ╚───┘   ║╚─{6f}═┘  ║╚─{2f}═┘  ║╚─{24.5}═┘
                          │          │          │
                         [println]  [println]  [println]",
    )
    .unwrap();
    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    cmd.arg(file.path());
    cmd.assert().success().stdout("2.5\n3.5\n7\n3.5\n");
}

#[test]
fn test_for_loop() {
    let file = NamedTempFile::new("tmp").unwrap();
    file.write_str(
        "
[start]      ┌─═[gate]─┐
     ╚──{1}  │   ╔─┘   ║
         ╚──[]═─[++]═─[>=]
 [println]──╝╚──{10}═──┘",
    )
    .unwrap();
    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    cmd.arg(file.path());
    cmd.assert()
        .success()
        .stdout("1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n");
}

#[test]
fn test_greeting() {
    let file = NamedTempFile::new("tmp").unwrap();
    file.write_str(
        "
[start]═─{\"What is your name? \"}═─┐
             ╚─[print]    ╚─┐     │
                       {\"Hello \"} │
                          ┌─╝     │
           [print]─═[]──═[+]─═[readln]
   [println]─═{'!'}─╝",
    )
    .unwrap();
    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    cmd.arg(file.path());
    cmd.write_stdin("Jaxson");
    cmd.assert()
        .success()
        .stdout("What is your name? Hello Jaxson!\n");
}
