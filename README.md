[![Build and unit tests](https://github.com/wang-jianliang/crane/actions/workflows/rust.yml/badge.svg)](https://github.com/wang-jianliang/crane/actions/workflows/rust.yml)
# Crane (This project is still under development)

**Crane** is an open-source tool aimed at simplifying the management of multiple code repositories in large-scale projects, developed in Rust. It allows the execution of operations across various git repositories with a single command. For example, the command ```crane sync``` fetches all sub-repositories within a project in one go.

Crane is a tool similar to the [repo](https://gerrit.googlesource.com/git-repo/+/HEAD/README.md), which is used for source code management in Android projects. However, compared to the latter, Crane adopts a design of main repository plus sub-repositories, which is well compatible with individual git repositories, and provides more consistent user commands.

The Crane tool leverages a configuration file, ".crane", to delineate the sub-repositories within the primary project. This ".crane" file, in essence a Python script, provides users with enhanced flexibility in outlining the dependency graph.

## Core concepts
[TODO]

## Installation
[TODO]

## Quick start
### 1. Create the configuration file
Create a file named ".crane" within the root directory of your project and add following code into the file:
```python
deps = {
    ".": {
        "type": "solution",
        "deps_file": ".crane_deps",
        "url": "https://github.com/wang-jianliang/crane.git",
        "branch": "main",
    }
}
```

### 2. Sync the code
Run following command to sync the code to your local directoryExecute the following command to synchronize the code to the local directory, the effect is equivalent to executing the git clone command to pull all sub-repositories to the local one by one.
```shell
$ crane sync <project root>
```

## Build

Clone this repository and run:
```shell
$ cargo build
```
