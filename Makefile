help:
	@echo "run \"make run day=01\" or whatever day you like"

run:
	cargo run --bin day${day}
