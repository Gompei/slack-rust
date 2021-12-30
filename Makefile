MAKEFILE_DIR=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

.PHONY: test
test: mkcert
	cargo test

.PHONY: mkcert
mkcert:
	docker run --rm -d -e domain=localhost --name slack-rust-mkcert vishnunair/docker-mkcert && sleep 1
	docker cp slack-rust-mkcert:/root/.local/share/mkcert/rootCA.pem $(MAKEFILE_DIR)
	docker cp slack-rust-mkcert:/root/.local/share/mkcert/localhost-key.pem $(MAKEFILE_DIR)
	docker cp slack-rust-mkcert:/root/.local/share/mkcert/localhost.pem $(MAKEFILE_DIR)
	docker stop slack-rust-mkcert