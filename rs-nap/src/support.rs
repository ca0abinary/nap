#[cfg_attr(all(target_arch="x86_64", target_os="linux"), path="support/x86_64.rs")]
#[cfg_attr(all(target_arch="aarch64", target_os="linux"), path="support/aarch64.rs")]
mod support;
pub use support::*;

#[path="support/noarch.rs"]
mod noarch;
pub use noarch::*;
