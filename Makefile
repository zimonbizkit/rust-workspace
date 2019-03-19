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
.PHONY: new
new:
	docker-compose exec --user $(USR) rust cargo new projects/$(filter-out $@,$(MAKECMDGOALS)) --bin

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
	