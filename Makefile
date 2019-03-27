all: help
##-------------------------------------------------------------------------
##									_   
##                   _ __ _   _ ___| |_ 
##                  | '__| | | / __| __|
##                  | |  | |_| \__ \ |_ 
##  Tryna' learn sum|_|   \__,_|___/\__|                 
##                       
## ------------------------------------------------------------------------
## Available commands are:
## ------------------------------------------------------------------------
USR := $(shell id -u)
GRP := $(shell id -g)
RED := '\\033[0;31m'
##	help:			Help of the project
.PHONY : help
help : Makefile
	@sed -n 's/^##//p' $<

##	up:			Brings the environment on
.PHONY : up
up:
	docker-compose -f docker-compose.yml up -d

##	down:			Turns down the environment
.PHONY : down
down:
	docker-compose down

##	shell:			Open a shell to the main project environment
.PHONY: shell
shell:
	docker-compose exec -e "TERM=xterm-256color" rust bash

##	list:			List the projects to be buildable/runnable via cargo
.PHONY: list
list:
	find . -iname "Cargo.toml" | xargs grep -E  "name = |version" - | sed 's/.*://g'

##	rs-version:		Checks out the rust version
.PHONY: rs-version
rs-version:
	docker-compose exec rust rustup --version

##	cargo:			Executes cargo inside the container
.PHONY: cargo
cargo:
	docker-compose exec rust cargo $(filter-out $@,$(MAKECMDGOALS))

##	new: 			Create a new project structure via cargo
# the triple echo in the 'new' command is ugly. I'll fix it
.PHONY: new
new:
	docker-compose exec --user $(USR) rust cargo new projects/$(filter-out $@,$(MAKECMDGOALS)) --bin
	mkdir projects/$(filter-out $@,$(MAKECMDGOALS))/target/
	touch projects/$(filter-out $@,$(MAKECMDGOALS))/target/.gitignore
	echo "/debug/" >> projects/$(filter-out $@,$(MAKECMDGOALS))/target/.gitignore
	echo "/release/" >> projects/$(filter-out $@,$(MAKECMDGOALS))/target/.gitignore
	echo ".rustc_info.json " >> projects/$(filter-out $@,$(MAKECMDGOALS))/target/.gitignore


##	fix-perms:		Fix permisisons of project
.PHONY: fix-perms
fix-perms:
	sudo chown -R $(USR):$(GRP) .

## 	run:			[make run PROJECT_NAME]: Runs a project from the ones present. If no binary is present, then it compiles it.
.PHONY: run
run:
	docker-compose exec rust cargo run  --manifest-path=projects/$(filter-out $@,$(MAKECMDGOALS))/Cargo.toml

## 	build:			[make build PROJECT_NAME]: Compiles a project from the ones present.
.PHONY: build
build:
	docker-compose exec rust cargo build  --manifest-path=projects/$(filter-out $@,$(MAKECMDGOALS))/Cargo.toml

## 	watch:			[make watch PROJECT_NAME]: Watches the project errors upon saving files.
.PHONY: watch
watch:
	docker-compose exec rust bash -c "cd projects/$(filter-out $@,$(MAKECMDGOALS)) && cargo-watch -x check -s clear"


##	exercism-download:	[make exercism-download PROJECT-NAME]: Downloads the exercism project of choice by the exercism.io CLI tool 
.PHONY: exercism-download
exercism-download:
	docker-compose exec rust exercism download --exercise=$(filter-out $@,$(MAKECMDGOALS)) --track=rust

##	exercism-test:		[make exercism-test PROJECT-NAME]: Executes the test suite for the downloaded exercism project, if any 
.PHONY: exercism-test
exercism-test:
	docker-compose exec rust cargo test --manifest-path=projects/exercism/rust/$(filter-out $@,$(MAKECMDGOALS))/Cargo.toml


##	exercism-submit:	[make exercism-submit FILE-LIST]: Submits the implemented solution via the exercism.io CLI tool 
.PHONY: exercism-submit
exercism-submit:
	docker-compose exec rust exercism submit $(filter-out $@,$(MAKECMDGOALS))
	touch projects/exercism/rust/$(filter-out $@,$(MAKECMDGOALS))/.gitignore
	echo "/.exercism/" >> projects/$(filter-out $@,$(MAKECMDGOALS))/target/.gitignore
##
##
	