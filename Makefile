main:
	cargo build

run:
	cargo run

just_run:
	./target/debug/mp3-modify-rs.exe -f tests/test.mp3

test:
	cargo build
	./target/debug/mp3-modify-rs.exe -f tests/test.mp3