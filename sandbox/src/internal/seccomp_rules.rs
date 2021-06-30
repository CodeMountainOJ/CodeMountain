use std::convert::TryInto;

use nix::libc::{SYS_execve};
use seccomp::{ Context, Rule, Compare, Action };

use crate::Sandbox;

/*
  TODO: Add all the other necessary rules :^)
*/

/// Creates and loads seccomp rules into kernel and returns true or false weather it's a success or not
pub fn set_seccomp_rules(config: &Sandbox) -> bool {
  let mut seccomp_ctx = Context::default(Action::Allow).unwrap();

  
  // no execve
  let no_execve_cmp = Compare::arg(0)
    .using(seccomp::Op::Ne)
    .with(config.executable_path.as_ptr() as u64)
    .build().unwrap();

  let no_execve_rule = Rule::new(SYS_execve.try_into().unwrap(), no_execve_cmp, Action::Kill);
  if seccomp_ctx.add_rule(no_execve_rule).is_err() {
    return false;
  }

  if seccomp_ctx.load().is_err() {
    return false;
  }

  true
}