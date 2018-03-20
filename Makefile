PWD:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

normal: frontend
	cargo build --release

.PHONY: frontend
frontend:
	cd react && npm install && npm run build
	rm -rf www/
	mkdir -p www/
	cp -r react/dist/* www/

docker: frontend
	docker pull clux/muslrust
	docker run -v cargo-cache:/root/.cargo -v "$(PWD):/volume" --rm -it clux/muslrust cargo build --release
	docker build -t roba1993/spe3d .

.PHONY: docker-run
docker-run:
	docker run -t -p 127.0.0.1:8000:8000 -p 127.0.0.1:8001:8001 -v $(PWD)/config:/config -v $(PWD)/out:/out --name=spe3d -t roba1993/spe3d

.PHONY: docker-rm
docker-rm:
	docker rm --force spe3d