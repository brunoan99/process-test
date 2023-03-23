use std::process::{Command, Output, Stdio};
use std::str;

fn print_output(title: String, o: &Output) {
  println!("{}", title);
  println!("status: {:?}", o.status);
  println!(
    "output: {:?}",
    str::from_utf8(&o.stdout).expect("failed to parse output")
  );
  println!(
    "error : {:?}",
    str::from_utf8(&o.stderr).expect("failed to parse err")
  );
  println!();
}

fn git_status() {
  let mut cmd = Command::new("sh");
  cmd.args(["-c", "git status --short"]);
  let output = cmd.output().expect("faield to execute cmd");

  print_output(String::from("Git Status"), &output);
}

fn echo_path() {
  let mut cmd = Command::new("sh");
  cmd.args(["-c", "echo $HOME/workspace"]);
  let mut output = cmd.output().expect("failed to execute cmd");
  output
    .stdout
    .resize(output.stdout.len() - 1, output.stdout[0]);

  print_output(String::from("Echo path with reference"), &output);
}

fn exec_in_another_dir() {
  let output = Command::new("ls")
    .current_dir("/home/snape")
    .output()
    .unwrap();
  print_output(String::from("Exec in other path"), &output);
}

fn exec_in_two_cmds() {
  let ls_cmd = Command::new("ls").stdout(Stdio::piped()).spawn().unwrap();

  let grep_cmd = Command::new("grep")
    .args([".rs", "--color"])
    .stdin(Stdio::from(ls_cmd.stdout.unwrap()))
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();

  let output = grep_cmd.wait_with_output().unwrap();
  print_output(String::from("Cd and exec using chield"), &output);
}

fn main() {
  git_status();
  echo_path();
  exec_in_another_dir();
  exec_in_two_cmds();
}
