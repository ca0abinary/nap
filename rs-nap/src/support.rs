#[cfg(all(target_arch="x86_64", target_os="linux"))]
#[path="support/x86_64.rs"]
mod arch_support;

#[cfg(all(target_arch="aarch64", target_os="linux"))]
#[path="support/aarch64.rs"]
mod arch_support;

#[cfg(not(target_os="linux"))]
#[path="support/generic.rs"]
mod arch_support;

pub use arch_support::*;

#[path="support/noarch.rs"]
mod noarch;
pub use noarch::*;
