# By default, print instructions.
all:
	@echo "Options:"
	@echo " - init: Initialize local development environment."
	@echo " - suite: Run the testing suite."
	@echo " - suite-strict: Run the testing suite as CI would. Warnings become errors."

# Initialize development environment. Run `make init` to execute.
# This sets a local developer's git hook location to the .githooks/
# directory. This allows hooks to be changed and pushed via the repository,
# as traditionally this is stored in .git/hooks, which is not under normal
# repository tracking.
init:
	git config core.hooksPath .githooks

# Run the pre-commit testing suite.
suite:
	bash .githooks/pre-commit

# Run the pre-commit testing suite, but enforce more warnings. This is applicable
# when using continuous integration.
suite-strict:
	bash .githooks/pre-commit strict
