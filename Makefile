PWD:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

normal: frontend
	cargo build --release

.PHONY: frontend
frontend:
	cd frontend && npm install && npm run build
	rm -rf www/*
	cp -r frontend/build/* www/

docker: frontend
	docker pull clux/muslrust
	docker run -v cargo-cache:/root/.cargo -v "$(PWD):/volume" --rm -it clux/muslrust cargo build --release
	docker build -t roba1993/spe3d .