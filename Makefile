main:
	cargo build

run:
	cargo run

just_run:
	./target/debug/audio-metadata-rs.exe -f tests/test.mp3

test:
	cargo build
	./target/debug/audio-metadata-rs.exe -f tests/test.mp3