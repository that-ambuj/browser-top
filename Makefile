dev:
	ENV=dev yarn --cwd frontend dev & cargo watch -i frontend -c -x run

dev-backend:
	cargo watch -i frontend -c -x run

backend:
	cargo build --release

frontend: 
	cd frontend && yarn build

build: frontend backend

start: build
	ENV=prod ./target/release/browser-top

.PHONY: frontend