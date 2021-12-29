MAKEFILE_DIR=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

.PHONY: test
test: certificate
	cargo test

.PHONY: certificate
certificate:
	docker volume create --name slack-rust-mkcert-data
	docker run --rm -d -e domain=localhost --name slack-rust-mkcert -v slack-rust-mkcert-data:/root/.local/share/mkcert vishnunair/docker-mkcert
	docker cp slack-rust-mkcert:/root/.local/share/mkcert/rootCA.pem $(MAKEFILE_DIR)
	docker cp slack-rust-mkcert:/root/.local/share/mkcert/localhost-key.pem $(MAKEFILE_DIR)
	docker cp slack-rust-mkcert:/root/.local/share/mkcert/localhost.pem $(MAKEFILE_DIR)
	docker stop slack-rust-mkcert && docker volume rm slack-rust-mkcert-data