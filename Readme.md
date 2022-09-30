# Cargo Gitlab CI Helper

## Description

This tool aims to resolve a specific problem.

When you're using private dependency authenticate by an SSH key authentication in a CI environment, you'll encounter
the issue of how to explain to Cargo that you want to get the dependency though an HTTPS basic authentication instead of
an SSH one.

_Exemple_:

```toml
[package]
name = "project1"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[dependencies]
toto = { git = "git@gitlab.com:orga/project.git", tag="1.0.0" }

[dev-dependencies.titi]
git = "git@gitlab.domain.tld:myorg/sub_project/too/deep.git"
tag = "1.0.0"
default-no-features = "true"
features = [
 "feature1",
 "test"
]
```

As you can see, `toto` and `titi` dependencies are gated behind an SSH
authentication.


You'd rather prefer having this:

```toml
[dependencies.toto]
git = "https://gitlab-ci-token:token123@gitlab.com/orga/project.git"
tag = "1.0.0"

[dev-dependencies.titi]
default-no-features = "true"
features = ["feature1", "test"]
git = "https://gitlab-ci-token:token123@gitlab.domain.tld/myorg/sub_project/too/deep.git"
tag = "1.0.0"
```

Thanks to the [CI job tokens](https://docs.gitlab.com/ee/ci/jobs/ci_job_token.html) you'll be able to access to your private repositories
using the HTTPS authentication method.

## How to use

```yaml
build:
  stage: build
  script:
    - cargo-ci --path Cargo.toml --output Cargo.dest.toml --deps toto --dev-deps titi --token ${CI_TOKEN_JOB}
```

## How it works

The tool takes a `Cargo.toml` file as input, scans each dependency and development dependency.

If one of these dependencies is referenced either in `--deps` or `--dev-deps` options.

It searches for `git` key, 

If the `git` value is an SSH dependency, it'll transform it as HTTPS one using the `--token` parameter.

_Exemple_:

From

```
git@gitlab.com:orga/project.git
```

To

```
https://gitlab-ci-token:token123@gitlab.com/orga/project.git
```