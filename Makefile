install:
	cargo install

build:
	cargo build

run:
	cargo run

watch:
	cargo watch -q -c -w src/ -x run

migrate-up:
	sqlx migrate run

migrate-down:
	sqlx migrate revert