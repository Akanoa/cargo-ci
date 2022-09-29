use cargo_ci::parser::{parse_git, GitUrl};
use cargo_ci::{run, to_value};

#[test]
fn transform() {
    let source_data = to_value("tests/assets/source.toml").unwrap();
    let dest_data = to_value("tests/assets/dest.toml");

    let result = run(
        "token123".to_string(),
        source_data,
        vec!["toto".to_string()],
        vec![],
    );

    assert_eq!(result, dest_data);
}

#[test]
fn transform_without_dev() {
    let source_data = to_value("tests/assets/source_without_dev.toml").unwrap();
    let dest_data = to_value("tests/assets/dest_without_dev.toml");

    let result = run(
        "token123".to_string(),
        source_data,
        vec!["toto".to_string()],
        vec![],
    );

    assert_eq!(result, dest_data);
}

#[test]
fn transform_only_dev() {
    let source_data = to_value("tests/assets/source_only_dev.toml").unwrap();
    let dest_data = to_value("tests/assets/dest_only_dev.toml");

    let result = run(
        "45sdfdsxf45".to_string(),
        source_data,
        vec![],
        vec!["titi".to_string()],
    );

    assert_eq!(result, dest_data);
}

#[test]
fn parse_git_url() {
    let project_name = "project22_zefsdfgdfg_ff4-FHGF_55";
    let organisation = "4155fd.dGHFHsf_dsgsd-4245DJFVH";
    let domain = "4155fd.dGHFHsf_dsgsd-4245DJFVH.my_gitlab.co.uk";

    let url = format!("git@{domain}:{organisation}/{project_name}.git");

    let result = parse_git(url.as_str());

    assert_eq!(
        result,
        Ok((
            format!("/{project_name}.git"),
            GitUrl {
                domain: domain.to_string(),
                organisation: organisation.to_string()
            }
        ))
    )
}
