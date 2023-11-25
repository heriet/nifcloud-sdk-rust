# nifcloud-sdk-rust

Rust SDK for NIFCLOUD. Currently Experimental Project.

## Requirements

Requires [smithy-rs](https://github.com/smithy-lang/smithy-rs).

You shoud environment variable `REPO_SMITHY_RS` to smithy-rs repository path.

```sh
export REPO_SMITHY_RS=<your smithy-lang/smithy-rs repository path>

or

export REPO_SMITHY_RS=$(ghq list --full-path --exact smithy-lang/smithy-rs) 
```

Build smithy-rs and publish to maven local repository.

```sh
docker compose run --rm smithy-rs bash
cd /smithy-rs
./gradlew :aws:sdk:assemble
./gradlew publishToMavenLocal
```

Build docker image `smithy-rs-build-image:latest`

```sh
cd $REPO_SMITHY_RS/tools/ci-build
docker build -t smithy-rs-build-image:latest .
```

## Usage

```sh
docker compose run --rm smithy-rs bash
cd codegen
./gradlew :nifcloud:sdk:assemble
```

## Example

```sh
docker compose run --rm dev bash
cd examples/storage

export NIFCLOUD_ACCESS_KEY_ID=<your access key id>
export NIFCLOUD_SECRET_ACCESS_KEY=<your secret access key>

cargo run
```
