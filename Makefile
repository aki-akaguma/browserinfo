
all: readme

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

