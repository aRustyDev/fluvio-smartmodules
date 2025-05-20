.PHONY: build test clean

URL=`git remote -v | rg fetch | rg -o "\S+://\S+"`
REPO=$(shell basename `git rev-parse --show-toplevel`)
ORG=`git remote -v | rg fetch | sed 's|.*://[^/]*/||' | rg -o '^[^/]+'`
USER=`git config user.name`

build:
	@smdk build

test:
	@cargo test
	@smdk test

load:
	@smdk load

publish:
	@smdk publish

clean:
	@smdk clean
