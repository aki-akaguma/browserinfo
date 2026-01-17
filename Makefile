
all: list

MAKEFILE_LIST = Makefile
# Self-documenting Makefile targets script from Stack Overflow
# Targets with comments on the same line will be listed.
list:
	@LC_ALL=C $(MAKE) -pRrq -f $(firstword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/(^|\n)# Files(\n|$$)/,/(^|\n)# Finished Make data base/ {if ($$1 !~ "^[#.]") {if ($$1 !~ "^[^:]*/[^:]*$$") {print $$1}}}' | sort | grep -E -v -e '^[^[:alnum:]]' -e '^$@$$'

.PHONY: list

readme: README.md

README.md: README.tpl src/lib.rs
	cargo readme > $@

test:
	cargo test --offline

test-no-default-features:
	cargo test --offline --no-default-features

clean:
	@cargo clean
	@rm -f z.*
	@rm -f *.profraw

clippy:
	cargo clippy --offline --tests --workspace

fmt:
	cargo fmt

doc:
	cargo doc

minix: assets/min/broinfo.js assets/min/user_agent.js

assets/min/broinfo.js: assets/js/broinfo.js
	minix -i assets/js/broinfo.js -o assets/min/broinfo.js

assets/min/user_agent.js: assets/js/user_agent.js
	minix -i assets/js/user_agent.js -o assets/min/user_agent.js

update-regexes: resources/regexes.yaml

resources/regexes.yaml: core/regexes.yaml
	cp -a core/regexes.yaml resources/

core/regexes.yaml:
	git submodule update --init core

#	git submodule update --remote core

