//! ```cargo
//! [dependencies]
//! tmr = { path = "../../packages/tmr" }
//! toml = "0.7.8"
//! ```
#[derive(tmr::Workspace)]
struct Workspace {}

#[tmr::workspace]
impl Workspace {
    fn members(&self) -> tmr::workspace::Members {
        vec!["sample-package"].into()
    }
}

tmr::cargo_toml!(Workspace);
