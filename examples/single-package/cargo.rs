//! ```cargo
//! [dependencies]
//! tmr-cargo = { path = "../../packages/tmr-cargo" }
//! toml = "0"
//! toml_edit = "0"
//! semver = "1"
//! ```
use std::convert::TryInto;
#[derive(tmr_cargo::Item)]
#[key("package")]
struct Package {
    #[value("sample-package")]
    name: tmr_cargo::package::Name,
}

#[tmr_cargo::item("package")]
impl Package {
    fn version(&self) -> tmr_cargo::package::Version {
        "0.1.0".try_into().unwrap()
    }

    fn description(&self) -> tmr_cargo::package::Description {
        format!("Example of {}", Into::<String>::into(&self.name)).into()
    }
}

tmr_cargo::cargo_toml!(Package);
