/// *nix stuff
use nix::sys::signal::{raise, SIGUSR1};

#[allow(unused_must_use)]
pub fn system_error() {
  raise(SIGUSR1);
  std::process::exit(1);
}