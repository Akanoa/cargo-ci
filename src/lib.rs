use crate::parser::parse_git;
use std::fs::File;
use std::io::Read;

pub mod parser;

pub fn to_value<S: Into<String>>(path: S) -> Option<toml::Value> {
    let source_data = File::open(path.into()).ok().and_then(|mut file| {
        let mut buffer = String::new();
        let result = file.read_to_string(&mut buffer);

        match result.is_ok() {
            true => Some(buffer),
            false => None,
        }
    });

    match &source_data {
        None => None,
        Some(content) => {
            let value: Option<toml::Value> = toml::from_str(content).ok();
            value
        }
    }
}

fn process_dependency(
    ci_token_job: &String,
    table: &mut toml::value::Table,
    dependency: String,
) -> Option<()> {
    let dependency = table.get_mut(&dependency)?.as_table_mut()?;

    let git_entry = dependency.get_mut("git")?;

    let git_string = git_entry.as_str()?;

    let (path, git_url) = parse_git(git_string).ok()?;

    let https_url = format!(
        "https://gitlab-ci-token:{ci_token_job}@{}/{}{path}",
        git_url.domain, git_url.organisation
    );

    let value = toml::Value::String(https_url);

    *git_entry = value;

    None
}

fn process(
    ci_token_job: String,
    table: &mut toml::value::Table,
    dependencies: Vec<String>,
) -> Option<()> {
    for dependency in dependencies {
        process_dependency(&ci_token_job, table, dependency);
    }

    None
}

pub fn run(
    ci_token_job: String,
    mut source: toml::Value,
    dependencies_to_update: Vec<String>,
    dev_dependencies_to_update: Vec<String>,
) -> Option<toml::Value> {
    let root = source.as_table_mut()?;

    if let Some(dependencies) = root.get_mut("dependencies") {
        let dependencies = dependencies.as_table_mut()?;
        process(ci_token_job.clone(), dependencies, dependencies_to_update);
    }

    if let Some(dev_dependencies) = root.get_mut("dev-dependencies") {
        let dev_dependencies = dev_dependencies.as_table_mut()?;
        process(ci_token_job, dev_dependencies, dev_dependencies_to_update);
    }

    Some(source)
}
