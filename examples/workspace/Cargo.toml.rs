//! ```cargo
//! [dependencies]
//! tmr = { path = "../../packages/tmr" }
//! toml = "0.7.8"
//! ```
use std::convert::TryInto;
#[derive(tmr::Workspace)]
struct Workspace {}

#[tmr::workspace]
impl Workspace {
    fn members(&self) -> tmr::workspace::Members {
        vec!["sample-package"].into()
    }
}

#[derive(tmr::WorkspacePackage)]
struct WorkspacePackage {
    #[value("sample-description")]
    description: tmr::workspace::package::Description,
}

#[tmr::workspace_package]
impl WorkspacePackage {}

tmr::cargo_toml!(Workspace, WorkspacePackage);
