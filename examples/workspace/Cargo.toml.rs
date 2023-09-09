//! ```cargo
//! [dependencies]
//! tmr = { path = "../../packages/tmr" }
//! ```
#[derive(tmr::Workspace)]
struct Workspace {}

#[tmr::workspace]
impl Workspace {
    fn members(&self) -> Vec<String> {
        vec!["sample-package".to_string()]
    }
}

tmr::cargo_toml!(Workspace);
