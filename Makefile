# By default, print instructions.
all:
	@echo "Options:"
	@echo " - init: Initialize local development environment."

# Initialize development environment. Run `make init` to execute.
# This sets a local developer's git hook location to the .githooks/
# directory. This allows hooks to be changed and pushed via the repository,
# as traditionally this is stored in .git/hooks, which is not under normal
# repository tracking.
init:
	git config core.hooksPath .githooks
