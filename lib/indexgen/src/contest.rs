use cargo_metadata::Package;
use serde::Serialize;

use crate::meta;

fn parse_contest_number(package: &Package) -> Result<String, ()> {
    if !package.name.starts_with("abc") {
        return Err(());
    }

    package
        .name
        .split('_')
        .nth(1)
        .map(|s| s.to_string())
        .ok_or(())
}

fn is_solved(package: &Package, task: char) -> bool {
    package
        .targets
        .iter()
        .any(|t| t.name == task.to_string() && t.kind.contains(&cargo_metadata::TargetKind::Bin))
}

#[derive(Debug, Clone, Serialize)]
pub struct Beginner {
    contest_number: String,
    solved_tasks: Vec<String>,
}

impl TryFrom<Package> for Beginner {
    type Error = ();

    fn try_from(pkg: Package) -> Result<Self, Self::Error> {
        let contest_number = parse_contest_number(&pkg)?;
        let mut solved_tasks = Vec::new();
        for task in 'a'..='h' {
            if is_solved(&pkg, task) {
                solved_tasks.push(task.to_string());
            }
        }

        Ok(Beginner {
            contest_number,
            solved_tasks,
        })
    }
}

impl Beginner {
    pub fn contests_in_workspace() -> Vec<Self> {
        let members = meta::workspace_members();
        members
            .into_iter()
            .filter_map(|pkg| Self::try_from(pkg).ok())
            .collect()
    }
}
