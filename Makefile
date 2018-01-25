PWD:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

normal:
	cargo build --release

docker:
	docker pull clux/muslrust
	docker run -v cargo-cache:/root/.cargo -v "$(PWD):/volume" --rm -it clux/muslrust cargo build --release
	docker build -t spe3d .