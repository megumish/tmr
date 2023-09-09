use serde::Serialize;

use crate::ToToml;

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

impl ToToml for Members {}

#[derive(Serialize)]
pub struct Member(String);

impl ToToml for Member {}
