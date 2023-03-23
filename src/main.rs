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
  let mut command = Command::new("sh");
  command.args(["-c", "git status --short"]);
  let output = command.output().expect("faield to execute command");

  print_output(String::from("Git Status"), &output);
}

fn echo_path() {
  let mut command = Command::new("sh");
  command.args(["-c", "echo $HOME/workspace"]);
  let mut output = command.output().expect("failed to execute command");
  output
    .stdout
    .resize(output.stdout.len() - 1, output.stdout[0]);

  print_output(String::from("Echo path with reference"), &output);
}

fn main() {
  git_status();

  echo_path();
}
