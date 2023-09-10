use std::path::PathBuf;

use serde::Serialize;

use crate::ToToml;

#[derive(Serialize)]
pub struct Path(PathBuf);

impl From<PathBuf> for Path {
    fn from(path: PathBuf) -> Self {
        Self(path)
    }
}

impl ToToml for Path {}

pub struct Version(semver::VersionReq);

impl Serialize for Version {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.to_string().serialize(serializer)
    }
}

impl From<semver::VersionReq> for Version {
    fn from(version: semver::VersionReq) -> Self {
        Self(version)
    }
}

impl ToToml for Version {}

#[derive(Serialize)]
pub struct Features(Vec<Feature>);

impl From<Vec<&str>> for Features {
    fn from(features: Vec<&str>) -> Self {
        Self(
            features
                .into_iter()
                .map(|feature| Feature(feature.to_string()))
                .collect(),
        )
    }
}

impl From<&[&str]> for Features {
    fn from(features: &[&str]) -> Self {
        Self(
            features
                .iter()
                .map(|feature| Feature(feature.to_string()))
                .collect(),
        )
    }
}

impl ToToml for Features {}

#[derive(Serialize)]
pub struct Feature(String);

impl ToToml for Feature {}
