use serde::Serialize;

use crate::ToToml;

#[derive(Serialize)]
pub struct Authors(Vec<Author>);

impl From<Vec<&str>> for Authors {
    fn from(authors: Vec<&str>) -> Self {
        Self(
            authors
                .into_iter()
                .map(|author| Author(author.to_string()))
                .collect(),
        )
    }
}

impl ToToml for Authors {}

#[derive(Serialize)]
pub struct Author(String);

impl ToToml for Author {}

#[derive(Serialize)]
pub struct Categories(Vec<Category>);

impl From<Vec<&str>> for Categories {
    fn from(categories: Vec<&str>) -> Self {
        Self(
            categories
                .into_iter()
                .map(|category| Category(category.to_string()))
                .collect(),
        )
    }
}

impl ToToml for Categories {}

#[derive(Serialize)]
pub struct Category(String);

impl ToToml for Category {}

#[derive(Serialize)]
pub struct Description(String);

impl From<String> for Description {
    fn from(description: String) -> Self {
        Self(description)
    }
}

impl From<&str> for Description {
    fn from(description: &str) -> Self {
        Self(description.to_string())
    }
}

impl ToToml for Description {}

#[derive(Serialize)]
pub struct Documentation(String);

impl From<&str> for Documentation {
    fn from(documentation: &str) -> Self {
        Self(documentation.to_string())
    }
}

impl ToToml for Documentation {}

#[derive(Serialize)]
pub enum Edition {
    #[serde(rename = "2015")]
    Edition2015,
    #[serde(rename = "2018")]
    Edition2018,
    #[serde(rename = "2021")]
    Edition2021,
}

impl ToToml for Edition {}

#[derive(Serialize)]
pub struct Homepage(String);

impl From<&str> for Homepage {
    fn from(homepage: &str) -> Self {
        Self(homepage.to_string())
    }
}

impl ToToml for Homepage {}

#[derive(Serialize)]
pub struct Keywords(Vec<Keyword>);

impl From<Vec<&str>> for Keywords {
    fn from(keywords: Vec<&str>) -> Self {
        Self(
            keywords
                .into_iter()
                .map(|keyword| Keyword(keyword.to_string()))
                .collect(),
        )
    }
}

impl ToToml for Keywords {}

#[derive(Serialize)]
pub struct Keyword(String);

impl ToToml for Keyword {}

#[derive(Serialize)]
pub struct License(String);

impl From<&str> for License {
    fn from(license: &str) -> Self {
        Self(license.to_string())
    }
}

impl ToToml for License {}

#[derive(Serialize)]
pub struct Publish(bool);

impl From<bool> for Publish {
    fn from(publish: bool) -> Self {
        Self(publish)
    }
}

impl ToToml for Publish {}

#[derive(Serialize)]
pub struct Readme(String);

impl From<&str> for Readme {
    fn from(readme: &str) -> Self {
        Self(readme.to_string())
    }
}

impl ToToml for Readme {}

#[derive(Serialize)]
pub struct Repository(String);

impl From<&str> for Repository {
    fn from(repository: &str) -> Self {
        Self(repository.to_string())
    }
}

impl ToToml for Repository {}

pub struct RustVersion(semver::Version);

impl Serialize for RustVersion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.to_string().serialize(serializer)
    }
}

impl TryFrom<&str> for RustVersion {
    type Error = semver::Error;

    fn try_from(rust_version: &str) -> Result<Self, Self::Error> {
        Ok(Self(semver::Version::parse(rust_version)?))
    }
}

impl ToToml for RustVersion {}

pub struct Version(semver::Version);

impl Serialize for Version {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.to_string().serialize(serializer)
    }
}

impl TryFrom<&str> for Version {
    type Error = semver::Error;

    fn try_from(version: &str) -> Result<Self, Self::Error> {
        Ok(Self(semver::Version::parse(version)?))
    }
}

impl ToToml for Version {}