use cargo_ci::{run, to_value};
use clap::Parser;

#[derive(Parser, clap::ValueEnum, Clone, Debug)]
enum CiKind {
    Gitlab,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Convert a Cargo.toml private SSH dependencies to HTTPS dependenciecies
///
/// ```toml
///[dependencies]
///toto = {git = "git@gitlab.com:orga/project.git", tag="1.0.0"}
///
/// [dev-dependencies.titi]
/// git = "git@gitlab.domain.tld:myorg/sub_project/too/deep.git"
/// tag = "1.0.0"
/// default-no-features = "true"
/// features = [
///     "feature1",
///     "test"
/// ]
/// ```
///
/// to
///
///```toml
///[dependencies]
///toto = {git = "https://gitlab-ci-token:token123@gitlab.com/orga/project.git", tag="1.0.0"}
///
/// [dev-dependencies.titi]
/// git = "https://gitlab-ci-token:token123gitlab.domain.tld/myorg/sub_project/data.git"
/// tag = "1.0.0"
/// default-no-features = "true"
/// features = [
///     "feature1",
///     "test"
/// ]
///```
struct Args {
    /// CI provider
    #[arg(short, long, default_value = "CiKind::Gitlab")]
    ci: CiKind,
    /// Path to Cargo.toml file
    #[arg(short, long, default_value = "Cargo.toml")]
    path: String,
    /// List of dependencies to convert
    #[arg(long = "deps", default_value = "vec![]", value_delimiter = ',')]
    dependencies: Vec<String>,
    /// List of dev dependencies to convert
    #[arg(long = "dev-deps", default_value = "vec![]", value_delimiter = ',')]
    dev_dependencies: Vec<String>,
    /// Job CI token
    #[arg(long)]
    token: String,
}

fn main() {
    let args = Args::parse();

    let source = to_value(args.path).unwrap();

    run(args.token, source, args.dependencies, args.dev_dependencies);
}
