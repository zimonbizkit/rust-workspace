## Rust project workspace with docker environment

This repo aims to implement a workspace to follow the [Rust programming language](https://doc.rust-lang.org/book/) book to implement a series of projects that are put as examples in the book. The aim is that it can serve as a boilerplate workspace for more ambitious projects in the future.

### Requirements
- Docker 
- Docker compose
- `make` command

### Installation
- Clone the repo
- Execute `make up`

### Available commands
 Execute `make` for a list of available commands

## First example project
The workspace uses `rustc` and `cargo` that are inside the container to build, run, and manage projects and its dependencies. To check the available projects and its versions, execute `make list`.

You can, then, either build or run the project excecuting `make build [project_name]` and `make run [project_name]` respectively.

### TODO
- [x] Implement the watch command to check the project compile errors while programming
- [x] Put proper .gitignore file when creating a new project
- [ ] Make exercism-submit command automatically check the potential files changed of the project and submit them accordingly.  
- [ ] Finetune the list command