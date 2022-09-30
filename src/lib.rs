use crate::parser::parse_git;
use eyre::{eyre, ContextCompat, Report, Result, WrapErr};
use std::fs::File;
use std::io::{Read, Write};

mod parser;

const DEPENDENCIES: &str = "dependencies";
const DEV_DEPENDENCIES: &str = "dev-dependencies";

pub fn to_value<S: Into<String>>(path: S) -> Result<toml::Value> {
    let path = path.into();

    let source_data = File::open(&path).ok().and_then(|mut file| {
        let mut buffer = String::new();
        let result = file.read_to_string(&mut buffer);

        match result.is_ok() {
            true => Some(buffer),
            false => None,
        }
    });

    match &source_data {
        None => Err(eyre!("{path}").wrap_err("No data to parse")),
        Some(content) => {
            let value: Result<toml::Value> = toml::from_str(content)
                .wrap_err(path)
                .wrap_err("Unable to parse file");
            value
        }
    }
}

fn process_dependency(
    ci_token_job: &String,
    table: &mut toml::value::Table,
    dependency: String,
) -> Result<Option<()>> {
    if let Some(dependency_value) = table.get_mut(&dependency) {
        let dependency_table = dependency_value
            .as_table_mut()
            .wrap_err("Unable to convert dependency value as Table")
            .wrap_err(dependency.clone())?;

        let git_entry = dependency_table.get_mut("git");

        if let Some(git_entry) = git_entry {
            let git_string = git_entry
                .as_str()
                .wrap_err("Unable to convert git entry as string")
                .wrap_err(eyre!("dependency : {}", dependency.clone()))?;

            let (path, git_url) = parse_git(git_string)
                .wrap_err("Unable to parse git entry")
                .wrap_err(eyre!("dependency : {}", dependency.clone()))?;

            let https_url = format!(
                "https://gitlab-ci-token:{ci_token_job}@{}/{}{path}",
                git_url.domain, git_url.organisation
            );

            let value = toml::Value::String(https_url);

            *git_entry = value;

            return Ok(Some(()));
        }
        return Ok(None);
    }
    Ok(None)
}

fn process(
    ci_token_job: String,
    table: &mut toml::value::Table,
    dependencies: Vec<String>,
) -> Option<(Vec<String>, Vec<Report>)> {
    let mut errors = vec![];
    let mut skipped_dependencies = vec![];

    for dependency in dependencies {
        let result = process_dependency(&ci_token_job, table, dependency.clone());

        match result {
            Ok(maybe) => match maybe {
                None => skipped_dependencies.push(dependency),
                Some(_) => {}
            },
            Err(err) => errors.push(err),
        }
    }
    if errors.is_empty() && skipped_dependencies.is_empty() {
        return None;
    }
    Some((skipped_dependencies, errors))
}

fn handle_process_return(kind: &str, result: Option<(Vec<String>, Vec<Report>)>) {
    match result {
        None => {
            log::info!("All {kind} have been processed")
        }
        Some((skipped_dependencies, errors)) => {
            for skipped in skipped_dependencies {
                log::warn!("{kind}.{skipped} have been skipped")
            }

            for error in errors {
                log::error!("{error:?}")
            }
        }
    }
}

fn convert(
    ci_token_job: String,
    mut source: toml::Value,
    dependencies_to_update: Vec<String>,
    dev_dependencies_to_update: Vec<String>,
) -> Result<toml::Value> {
    let root = source
        .as_table_mut()
        .ok_or(eyre!("Unable to convert root element as Table"))?;

    if let Some(dependencies) = root.get_mut(DEPENDENCIES) {
        if let Some(dependencies) = dependencies.as_table_mut() {
            let result = process(ci_token_job.clone(), dependencies, dependencies_to_update);
            handle_process_return(DEPENDENCIES, result);
        }
    }

    if let Some(dev_dependencies) = root.get_mut(DEV_DEPENDENCIES) {
        if let Some(dev_dependencies) = dev_dependencies.as_table_mut() {
            let result = process(ci_token_job, dev_dependencies, dev_dependencies_to_update);
            handle_process_return(DEV_DEPENDENCIES, result);
        }
    }

    Ok(source)
}

/// Takes the path of a source Cargo.toml file
/// Transforms dependencies git SSH entries to HTTPS one
/// with basic auth powered by CI token
/// The write the result to the destination file
pub fn run(
    ci_token_job: String,
    source_path: String,
    output_path: String,
    dependencies_to_update: Vec<String>,
    dev_dependencies_to_update: Vec<String>,
) -> Result<()> {
    let source = to_value(&source_path)
        .wrap_err("Unable to transform source file as TOML")
        .wrap_err(source_path.clone())?;

    let new_table = convert(
        ci_token_job,
        source,
        dependencies_to_update,
        dev_dependencies_to_update,
    )
    .wrap_err("Unable to convert source TOML tree")
    .wrap_err(source_path);

    match new_table {
        Ok(value) => {
            let mut dest_file = File::create(&output_path)
                .wrap_err(output_path.clone())
                .wrap_err("Unable to open destination path")?;

            let content = value.to_string();

            dest_file
                .write_all(content.as_bytes())
                .wrap_err(output_path)
                .wrap_err("Unable to write destination file")?;

            Ok(())
        }
        _ => Err(eyre!("No new table available")),
    }
}

#[cfg(test)]
mod tests {
    use crate::{convert, to_value};

    #[test]
    fn transform() {
        let source_data = to_value("tests/assets/source.toml").unwrap();
        let dest_data = to_value("tests/assets/dest.toml");

        let result = convert(
            "token123".to_string(),
            source_data,
            vec!["toto".to_string()],
            vec![],
        );

        assert_eq!(result.ok(), dest_data.ok());
    }

    #[test]
    fn transform_without_dev() {
        let source_data = to_value("tests/assets/source_without_dev.toml").unwrap();
        let dest_data = to_value("tests/assets/dest_without_dev.toml");

        let result = convert(
            "token123".to_string(),
            source_data,
            vec!["toto".to_string()],
            vec![],
        );

        assert_eq!(result.ok(), dest_data.ok());
    }

    #[test]
    fn transform_only_dev() {
        let source_data = to_value("tests/assets/source_only_dev.toml").unwrap();
        let dest_data = to_value("tests/assets/dest_only_dev.toml");

        let result = convert(
            "45sdfdsxf45".to_string(),
            source_data,
            vec![],
            vec!["titi".to_string()],
        );

        assert_eq!(result.ok(), dest_data.ok());
    }
}
