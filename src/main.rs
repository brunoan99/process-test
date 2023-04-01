use std::process::{Command, Output, Stdio};
use std::str;

fn print_output(title: &str, o: &Output) {
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
  let mut cmd = Command::new("/bin/git");
  cmd.args(["status", "--short"]);
  let output = cmd.output().expect("faield to execute cmd");

  print_output("Git Status", &output);
}

fn echo_path() {
  let mut cmd = Command::new("sh");
  cmd.args(["-c", "/bin/echo $HOME/workspace"]);
  let mut output = cmd.output().expect("failed to execute cmd");
  output
    .stdout
    .resize(output.stdout.len() - 1, output.stdout[0]);

  print_output("Echo path with reference", &output);
}

fn exec_in_another_dir() {
  let output = Command::new("ls")
    .current_dir("/home/snape")
    .output()
    .unwrap();
  print_output("Exec in other path", &output);
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
  print_output("Cd and exec using child", &output);
}

fn exec_in_evaluated_path() {
  let mut path_cmd_out = Command::new("sh")
    .args(["-c", "echo $PWD"])
    .stdout(Stdio::piped())
    .spawn()
    .unwrap()
    .wait_with_output()
    .unwrap();
  path_cmd_out
    .stdout
    .resize(path_cmd_out.stdout.len() - 1, path_cmd_out.stdout[0]);
  let path = str::from_utf8(&path_cmd_out.stdout).expect("failed to parse output");

  let output = Command::new("/bin/ls").current_dir(path).output().unwrap();
  print_output("Exec status in evaluated path", &output);
}

fn git_status_evaluated_path() {
  let mut path_cmd_out = Command::new("sh")
    .arg("-c")
    .arg("/bin/echo $PWD")
    .stdout(Stdio::piped())
    .spawn()
    .unwrap()
    .wait_with_output()
    .unwrap();
  path_cmd_out
    .stdout
    .resize(path_cmd_out.stdout.len() - 1, path_cmd_out.stdout[0]);
  let path = str::from_utf8(&path_cmd_out.stdout).expect("failed to parse output");

  let output = Command::new("/bin/git")
    .args(["status", "--short"])
    .current_dir(path)
    .output()
    .unwrap();
  print_output("Git status in evaluated path", &output);
}

fn git_status_other_path() {
  let output = Command::new("/bin/git")
    .args(["-C", "$PWD", "status", "--short"])
    .output()
    .unwrap();
  print_output("Git other path using -C", &output)
}

fn git_log() {
  let output = Command::new("/bin/git")
    .args(["log", "--oneline"])
    .output()
    .expect("faield to execute cmd");

  print_output("Git Log", &output);
}

fn main() {
  git_status();
  echo_path();
  exec_in_another_dir();
  exec_in_two_cmds();
  exec_in_evaluated_path();
  git_status_evaluated_path();
  git_status_other_path();
  git_log();
}
