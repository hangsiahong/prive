install:
	cargo build --release
	sudo cp target/release/prive /usr/bin/prive

