# Sats Escrow - Development Commands

.PHONY: dev build test fmt fmt-check lint check docker-up docker-down frontend-dev frontend-test clean

dev:
	cargo run

build:
	cargo build

test:
	cargo test --workspace

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --all-targets --all-features -- -D warnings

check: fmt-check lint test

docker-up:
	docker compose up -d

docker-down:
	docker compose down

frontend-dev:
	cd frontend && npm run dev

frontend-test:
	cd frontend && npm test

clean:
	cargo clean
