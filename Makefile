file ?=

install:
	@echo "🔍 Checking system dependencies..."
	@if ! command -v git > /dev/null; then 
		echo "📦 Installing Git..."; 
		sudo apt-get update && sudo apt-get install -y git; 
	else 
		echo "✅ Git already installed."; 
	fi

	@if ! command -v rustc > /dev/null || ! command -v cargo > /dev/null; then 
		echo "📦 Installing Rust & Cargo..."; 
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; 
		source $$HOME/.cargo/env; 
	else 
		echo "✅ Rust & Cargo already installed."; 
	fi

	@echo "✅ Installation complete."

build:
	cargo build --release

run:
	@if [ -z "$(file)" ]; then 
		echo "❌ Error: No file provided. Use 'make run file=yourfile.ion'"; 
		exit 1; 
	fi
	./target/release/Ion $(file)

clean:
	cargo clean

