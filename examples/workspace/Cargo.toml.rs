//! ```cargo
//! [dependencies]
//! tmr = { path = "../../packages/tmr" }
//! toml = "0.7.8"
//! toml_edit = "0.19.15"
//! semver = "1.0.18"
//! ```
use std::convert::TryFrom;
use std::convert::TryInto;
#[derive(tmr::Workspace)]
struct Workspace {
    #[values("packages/*")]
    #[values("examples/*")]
    members: tmr::workspace::Members,
    #[values("target")]
    #[values("examples/workspace")]
    exclude: tmr::workspace::Excludes,
}

#[tmr::workspace]
impl Workspace {
    fn resolver(&self) -> tmr::workspace::Resolver {
        tmr::workspace::Resolver::V2
    }
}

#[derive(tmr::WorkspacePackage)]
struct WorkspacePackage {
    #[values("megumish <megumish@megumi.sh>")]
    authors: tmr::workspace::package::Authors,
    #[value("Too many rust")]
    description: tmr::workspace::package::Description,
    #[value("MIT")]
    license: tmr::workspace::package::License,
    #[value("./README.md")]
    readme: tmr::workspace::package::Readme,
}

impl WorkspacePackage {
    fn kinds() -> Vec<&'static str> {
        vec!["cli", "automation", "macros"]
    }

    fn project_url() -> String {
        "https://github.com/megumish/tmr".to_owned()
    }
}

#[tmr::workspace_package]
impl WorkspacePackage {
    fn categories(&self) -> tmr::workspace::package::Categories {
        Self::kinds().into()
    }
    fn documentation(&self) -> tmr::workspace::package::Documentation {
        Self::project_url().into()
    }
    fn edition(&self) -> tmr::workspace::package::Edition {
        tmr::workspace::package::Edition::Edition2021
    }
    fn homepage(&self) -> tmr::workspace::package::Homepage {
        Self::project_url().into()
    }
    fn keywords(&self) -> tmr::workspace::package::Keywords {
        Self::kinds().into()
    }
    fn publish(&self) -> tmr::workspace::package::Publish {
        false.into()
    }
    fn repository(&self) -> tmr::workspace::package::Repository {
        format!("{}.git", Self::project_url()).into()
    }
    fn rust_version(&self) -> tmr::workspace::package::RustVersion {
        semver::Version::new(0, 1, 0).into()
    }
}

#[derive(tmr::WorkspaceDependencies)]
struct WorkspaceDependencies {
    #[values("env-filter")]
    #[route("tracing-subscriber.features")]
    tracing_subscriber_features: tmr::dependencies::Features,
}

#[tmr::workspace_dependencies]
impl WorkspaceDependencies {
    #[route("tmr.path")]
    fn tmr_path(&self) -> tmr::dependencies::Path {
        std::path::PathBuf::try_from("./packages/tmr")
            .unwrap()
            .into()
    }
    #[route("tmr-macros.path")]
    fn tmr_macros_path(&self) -> tmr::dependencies::Path {
        std::path::PathBuf::try_from("./packages/tmr-macros")
            .unwrap()
            .into()
    }
    fn tracing(&self) -> tmr::dependencies::Version {
        semver::VersionReq {
            comparators: vec![semver::Comparator {
                op: semver::Op::GreaterEq,
                major: 0,
                minor: Some(1),
                patch: Some(37),
                pre: semver::Prerelease::EMPTY,
            }],
        }
        .into()
    }
    #[route("tracing-subscriber.version")]
    fn tracing_subscriber_version(&self) -> tmr::dependencies::Version {
        semver::VersionReq {
            comparators: vec![semver::Comparator {
                op: semver::Op::GreaterEq,
                major: 0,
                minor: Some(3),
                patch: Some(17),
                pre: semver::Prerelease::EMPTY,
            }],
        }
        .into()
    }
}

tmr::cargo_toml!(Workspace, WorkspacePackage, WorkspaceDependencies);
