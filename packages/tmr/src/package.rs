use serde::Serialize;

use crate::ToToml;

#[derive(Serialize)]
pub struct Name(String);

impl From<&str> for Name {
    fn from(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl Into<String> for &Name {
    fn into(self) -> String {
        self.0.to_string()
    }
}

impl ToToml for Name {}

pub struct Version(semver::Version);

impl Serialize for Version {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.to_string().serialize(serializer)
    }
}

impl ToToml for Version {}

impl TryFrom<&str> for Version {
    type Error = semver::Error;

    fn try_from(version: &str) -> Result<Self, Self::Error> {
        Ok(Self(semver::Version::parse(version)?))
    }
}

#[derive(Serialize)]
pub struct Description(String);

impl From<String> for Description {
    fn from(description: String) -> Self {
        Self(description)
    }
}

impl Into<String> for Description {
    fn into(self) -> String {
        self.0
    }
}

impl ToToml for Description {}
