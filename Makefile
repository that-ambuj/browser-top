dev:
	ENV=dev yarn --cwd frontend dev & cargo watch -x run

backend:
	cargo build --release

frontend:
	yarn --cwd frontend build 

build: frontend backend

start: build
	ENV=prod ./target/release/browser-top