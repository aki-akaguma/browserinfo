
all:

update-regexes: resources/regexes.yaml

resources/regexes.yaml: core/regexes.yaml
	git submodule update --init
	cp -a core/regexes.yaml resources/
