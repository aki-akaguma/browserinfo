
all:

update-regexes: resources/regexes.yaml

resources/regexes.yaml: core/regexes.yaml
	cp -a core/regexes.yaml resources/

core/regexes.yaml:
	git submodule update --init core

#	git submodule update --init
