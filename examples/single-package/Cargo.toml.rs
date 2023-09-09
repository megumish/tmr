//! ```cargo
//! [dependencies]
//! tmr = { path = "../../packages/tmr" }
//! toml = "0.7.8"
//! ```
use std::convert::TryInto;
#[derive(tmr::Package)]
struct Package {
    #[value("sample-package")]
    name: tmr::package::Name,
}

#[tmr::package]
impl Package {
    fn version(&self) -> tmr::package::Version {
        "0.1.0".try_into().unwrap()
    }

    fn description(&self) -> tmr::package::Description {
        format!("Example of {}", Into::<String>::into(&self.name)).into()
    }
}

tmr::cargo_toml!(Package);
