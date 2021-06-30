use nix::{libc::{ENOMEM, SIGKILL, SIGSEGV, SIGUSR1, WSTOPPED, WTERMSIG, kill, wait4}, unistd::Pid};

use crate::{Sandbox, SandboxResult};

pub fn parent_process(config: Sandbox, child_pid: Pid, sandbox_result: &mut SandboxResult) {
  let mut rusage = std::mem::MaybeUninit::zeroed();
  let mut status = 0;

  let start_time = std::time::SystemTime::now();
  let r = unsafe { wait4(child_pid.as_raw(), &mut status, WSTOPPED, rusage.as_mut_ptr()) };
  let end_time = start_time.elapsed().unwrap().as_secs() * 1000;


  if r < 0 {
    sandbox_result.system_error = true;
    unsafe { kill(child_pid.as_raw(), SIGKILL) };
    return;
  }

  let rusage = unsafe { rusage.assume_init() };
  sandbox_result.memory_spent = rusage.ru_maxrss;
  sandbox_result.cpu_time_spent = rusage.ru_utime.tv_sec * 1000 + rusage.ru_utime.tv_usec / 1000;
  sandbox_result.real_time_spent = end_time as i64;
  sandbox_result.signal = WTERMSIG(status);

  if WTERMSIG(status) == SIGUSR1 {
    // system error
    sandbox_result.system_error = true;
  }
  else if WTERMSIG(status) == SIGSEGV {
    if rusage.ru_maxrss == config.memory_limit {
      sandbox_result.mle = true;
    }
    else {
      // runtime error
      sandbox_result.runtime_error = true;
    }
  }
  else if WTERMSIG(status) == ENOMEM {
    sandbox_result.mle = true;
  }

  if sandbox_result.cpu_time_spent > config.cpu_time_limit {
    sandbox_result.cle = true;
  }

  if sandbox_result.real_time_spent > config.real_time_limit {
    sandbox_result.tle = true;
  }
}