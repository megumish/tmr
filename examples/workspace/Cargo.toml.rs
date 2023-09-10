//! ```cargo
//! [dependencies]
//! tmr-cargo = "0.4.0"
//! toml = "0.7.8"
//! toml_edit = "0.19.15"
//! semver = "1.0.18"
//! ```
use std::convert::TryFrom;
use std::convert::TryInto;
#[derive(tmr_cargo::Item)]
#[key("workspace")]
struct Workspace {
    #[values("packages/*")]
    #[values("examples/*")]
    members: tmr_cargo::workspace::Members,
    #[values("target")]
    #[values("examples/workspace")]
    exclude: tmr_cargo::workspace::Excludes,
}

#[tmr_cargo::item("workspace")]
impl Workspace {
    fn resolver(&self) -> tmr_cargo::workspace::Resolver {
        tmr_cargo::workspace::Resolver::V2
    }
}

#[derive(tmr_cargo::Item)]
#[key("workspace.package")]
struct WorkspacePackage {
    #[values("megumish <megumish@megumi.sh>")]
    authors: tmr_cargo::workspace::package::Authors,
    #[value("Too many rust")]
    description: tmr_cargo::workspace::package::Description,
    #[value("MIT")]
    license: tmr_cargo::workspace::package::License,
    #[value("./README.md")]
    readme: tmr_cargo::workspace::package::Readme,
}

impl WorkspacePackage {
    fn kinds() -> Vec<&'static str> {
        vec!["cli", "automation", "macros"]
    }

    fn project_url() -> String {
        "https://github.com/megumish/tmr_cargo".to_owned()
    }
}

#[tmr_cargo::item("workspace.package")]
impl WorkspacePackage {
    fn categories(&self) -> tmr_cargo::workspace::package::Categories {
        Self::kinds().into()
    }
    fn documentation(&self) -> tmr_cargo::workspace::package::Documentation {
        Self::project_url().into()
    }
    fn edition(&self) -> tmr_cargo::workspace::package::Edition {
        tmr_cargo::workspace::package::Edition::Edition2021
    }
    fn homepage(&self) -> tmr_cargo::workspace::package::Homepage {
        Self::project_url().into()
    }
    fn keywords(&self) -> tmr_cargo::workspace::package::Keywords {
        Self::kinds().into()
    }
    fn publish(&self) -> tmr_cargo::workspace::package::Publish {
        false.into()
    }
    fn repository(&self) -> tmr_cargo::workspace::package::Repository {
        format!("{}.git", Self::project_url()).into()
    }
    fn rust_version(&self) -> tmr_cargo::workspace::package::RustVersion {
        semver::Version::new(0, 1, 0).into()
    }
}

#[derive(tmr_cargo::Item)]
#[key("workspace.dependencies")]
struct WorkspaceDependencies {
    #[values("env-filter")]
    #[route("tracing-subscriber.features")]
    tracing_subscriber_features: tmr_cargo::dependencies::Features,

    #[values("derive")]
    #[route("serde.features")]
    serde_features: tmr_cargo::dependencies::Features,
}

#[tmr_cargo::item("workspace.dependencies")]
impl WorkspaceDependencies {
    #[route("tmr_cargo.path")]
    fn tmr_path(&self) -> tmr_cargo::dependencies::Path {
        std::path::PathBuf::try_from("./packages/tmr_cargo")
            .unwrap()
            .into()
    }
    #[route("tmr_cargo-macros.path")]
    fn tmr_macros_path(&self) -> tmr_cargo::dependencies::Path {
        std::path::PathBuf::try_from("./packages/tmr_cargo-macros")
            .unwrap()
            .into()
    }
    fn tracing(&self) -> tmr_cargo::dependencies::Version {
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
    fn tracing_subscriber_version(&self) -> tmr_cargo::dependencies::Version {
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
    #[route("serde.version")]
    fn serde_version(&self) -> tmr_cargo::dependencies::Version {
        semver::VersionReq {
            comparators: vec![semver::Comparator {
                op: semver::Op::GreaterEq,
                major: 1,
                minor: Some(0),
                patch: Some(187),
                pre: semver::Prerelease::EMPTY,
            }],
        }
        .into()
    }
    fn toml(&self) -> tmr_cargo::dependencies::Version {
        semver::VersionReq {
            comparators: vec![semver::Comparator {
                op: semver::Op::GreaterEq,
                major: 0,
                minor: Some(7),
                patch: Some(8),
                pre: semver::Prerelease::EMPTY,
            }],
        }
        .into()
    }
    fn semvar(&self) -> tmr_cargo::dependencies::Version {
        semver::VersionReq {
            comparators: vec![semver::Comparator {
                op: semver::Op::GreaterEq,
                major: 1,
                minor: Some(0),
                patch: Some(18),
                pre: semver::Prerelease::EMPTY,
            }],
        }
        .into()
    }
}

tmr_cargo::cargo_toml!(Workspace, WorkspacePackage, WorkspaceDependencies);
