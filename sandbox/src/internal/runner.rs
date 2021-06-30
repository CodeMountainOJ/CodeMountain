use crate::{Sandbox, SandboxResult};

// Unix stuff
use nix::unistd::{fork, ForkResult};

use super::{child_process::child_process, parent_process::parent_process};

pub fn internal_run(config: Sandbox) -> SandboxResult {
  let mut sandbox_result = SandboxResult::default();

  match unsafe{fork()} {
    Ok(ForkResult::Parent { child }) => {
      parent_process(config, child, &mut sandbox_result)
    },
    Ok(ForkResult::Child) => {
      child_process(&config)
    },
    Err(_) => sandbox_result.system_error = true
  }

  sandbox_result
}