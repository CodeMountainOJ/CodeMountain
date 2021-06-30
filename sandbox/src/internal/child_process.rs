use crate::Sandbox;

// *nix stuff
use nix::libc::{RLIMIT_AS, RLIMIT_CPU, RLIMIT_STACK, STDERR_FILENO, rlimit, setrlimit};
use nix::unistd::{dup2, execve};
use nix::libc::{STDIN_FILENO, STDOUT_FILENO};
use std::ffi::CString;
use std::os::unix::prelude::AsRawFd;

use super::seccomp_rules::set_seccomp_rules;
use super::signal::system_error;

fn get_env_vars() -> Vec<CString> {
  let vars = std::env::vars();
  let mut vars_vec: Vec<CString> = Vec::new();

  for (k, v) in vars {
    vars_vec.push(CString::new(format!("{}={}", k, v)).unwrap());
  };

  return vars_vec
}

pub fn child_process(config: &Sandbox) {
  // dup2 stdin
  let executable_stdin_file = std::fs::File::open(&config.executable_stdin);

  if executable_stdin_file.is_err() {
    system_error(); // failed to open file to replace stdin with
  }
  
  let executable_stdin_fd = executable_stdin_file.unwrap().as_raw_fd();

  if dup2(STDIN_FILENO, executable_stdin_fd).is_err() {
    system_error(); // failed to replace stdin
  }

  // dup2 stdout
  let executable_stdout_file = std::fs::File::create(&config.executable_stdout);

  if executable_stdout_file.is_err() {
    system_error(); // failed to open file to replace stdout with
  }
  
  let executable_stdout_fd = executable_stdout_file.unwrap().as_raw_fd();

  if dup2(STDOUT_FILENO, executable_stdout_fd).is_err() {
    system_error(); // failed to replace stdout
  }

  // dup2 stderr
  let executable_stderr_file = std::fs::File::create(&config.executable_stderr);

  if executable_stderr_file.is_err() {
    system_error(); // failed to open file to replace stderr with
  }

  let executable_stderr_fd = executable_stderr_file.unwrap().as_raw_fd();

  if dup2(STDERR_FILENO, executable_stderr_fd).is_err() {
    system_error(); // failed to replace stderr
  }


  // Set resource limits(Unsafe)
  
  /* Memory limit */
  let mem_limit = rlimit {
    rlim_cur: (config.memory_limit * 1024) as u64,
    rlim_max: (config.memory_limit * 1024 * 2) as u64
  };

  if unsafe { setrlimit(RLIMIT_AS, &mem_limit) } != 0 {
    system_error(); // failed to set memory limit
  }

  /* Stack limit */
  let stack_limit = rlimit {
    rlim_cur: (config.stack_limit * 1024) as u64,
    rlim_max: (config.stack_limit * 1024) as u64
  };

  if unsafe { setrlimit(RLIMIT_STACK, &stack_limit) != 0 } {
    system_error(); // failed to set stack limit
  }

  /* CPU Time limit */
  let cpu_time_limit = rlimit {
    rlim_cur: (config.cpu_time_limit / 1000) as u64,
    rlim_max: (config.cpu_time_limit / 1000 * 2) as u64
  };

  if unsafe { setrlimit(RLIMIT_CPU, &cpu_time_limit) != 0 } {
    system_error(); // failed to set cpu time limit
  }

  if !set_seccomp_rules(&config) {
    system_error();
  }

  let c_executable_path = CString::new(config.executable_path.clone()).unwrap();
  let c_executable_args: Vec<CString> = config.executable_args.iter().map(|s| CString::new(s.clone()).unwrap()).collect();
  
  if execve(&c_executable_path, c_executable_args.as_slice(), get_env_vars().as_slice()).is_err() {
    system_error();
  }
}