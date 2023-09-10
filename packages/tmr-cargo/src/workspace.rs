use serde::Serialize;

use crate::ToToml;

pub mod package;

#[derive(Serialize)]
pub struct Members(Vec<Member>);

impl From<Vec<&str>> for Members {
    fn from(members: Vec<&str>) -> Self {
        Self(
            members
                .into_iter()
                .map(|member| Member(member.to_string()))
                .collect(),
        )
    }
}

impl From<&[&str]> for Members {
    fn from(members: &[&str]) -> Self {
        Self(
            members
                .iter()
                .map(|member| Member(member.to_string()))
                .collect(),
        )
    }
}

impl ToToml for Members {}

#[derive(Serialize)]
pub struct Member(String);

impl ToToml for Member {}

#[derive(Serialize)]
pub struct Excludes(Vec<Exclude>);

impl From<Vec<&str>> for Excludes {
    fn from(excludes: Vec<&str>) -> Self {
        Self(
            excludes
                .into_iter()
                .map(|exclude| Exclude(exclude.to_string()))
                .collect(),
        )
    }
}

impl From<&[&str]> for Excludes {
    fn from(excludes: &[&str]) -> Self {
        Self(
            excludes
                .iter()
                .map(|exclude| Exclude(exclude.to_string()))
                .collect(),
        )
    }
}

impl ToToml for Excludes {}

#[derive(Serialize)]
pub struct Exclude(String);

impl ToToml for Exclude {}

pub enum Resolver {
    V1,
    V2,
}

impl Serialize for Resolver {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Resolver::V1 => "1".serialize(serializer),
            Resolver::V2 => "2".serialize(serializer),
        }
    }
}

impl TryFrom<&str> for Resolver {
    type Error = ();

    fn try_from(resolver: &str) -> Result<Self, Self::Error> {
        match resolver {
            "1" => Ok(Self::V1),
            "2" => Ok(Self::V2),
            _ => Err(()),
        }
    }
}

impl ToToml for Resolver {}
