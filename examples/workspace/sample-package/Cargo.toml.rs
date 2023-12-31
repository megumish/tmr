//! ```cargo
//! [dependencies]
//! tmr = { path = "../../../packages/tmr" }
//! ```
#[derive(tmr::Package)]
struct Package {
    #[value("sample-package")]
    name: String,
}

#[tmr::package]
impl Package {
    fn version(&self) -> String {
        "0.1.0".to_string()
    }

    fn description(&self) -> String {
        format!("Example of {}", &self.name)
    }
}

tmr::cargo_toml!(Package);
