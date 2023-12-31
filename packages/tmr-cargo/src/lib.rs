use serde::Serialize;
pub use tmr_cargo_macros::*;

pub mod dependencies;
pub mod package;
pub mod workspace;

pub trait ToToml: Serialize {
    fn to_toml(&self) -> String {
        let mut buffer = String::new();
        self.serialize(toml::ser::ValueSerializer::new(&mut buffer))
            .unwrap();
        buffer
    }
}
