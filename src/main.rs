use std::process::Command;
use std::str;

fn main() {
  let mut echo_hello = Command::new("sh");
  echo_hello.arg("-c").arg("git status --short");
  let hello_1 = echo_hello.output().expect("faield to execute process");

  println!("first_status: {:?}", hello_1.status);
  println!();

  println!(
    "first_output: {:?}",
    str::from_utf8(&hello_1.stdout).expect("failed to parse output")
  );
  println!();

  println!(
    "first_error: {:?}",
    str::from_utf8(&hello_1.stderr).expect("failed to parse err")
  );
}
