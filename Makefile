npm: node_modules/
	npm install

build: ./src npm
	docker run --rm -v $$(pwd)\:/wd -v ~/.cargo/registry\:/root/.cargo/registry iai-compare-action
