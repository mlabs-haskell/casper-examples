prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cd contract && cargo build --release --target wasm32-unknown-unknown
	wasm-strip contract/target/wasm32-unknown-unknown/release/contract.wasm 2>/dev/null | true

test: build-contract
	mkdir -p tests/wasm
	cp contract/target/wasm32-unknown-unknown/release/contract.wasm tests/wasm
	cd tests && cargo test -- --nocapture

clippy:
	cd contract && cargo clippy --all-targets -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd contract && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd contract && cargo fmt
	cd tests && cargo fmt

clean:
	cd contract && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm

nctl-up:
	cd nctl-docker && docker compose up

nctl-up-detach:
	cd nctl-docker && docker compose up -d

nctl-down:
	cd nctl-docker && docker compose down

nctl-restart:
	cd nctl-docker && docker compose exec -d mynctl /bin/bash "-c" "chmod +x /home/casper/restart.sh && /home/casper/restart.sh"

nctl-copy-keys:
	cd nctl-docker && docker compose cp mynctl:/home/casper/casper-node/utils/nctl/assets/net-1/users .

cp-wasm-to-client:
	cd contract && cargo build --release --target wasm32-unknown-unknown
	wasm-strip contract/target/wasm32-unknown-unknown/release/contract.wasm 2>/dev/null | true
	cp contract/target/wasm32-unknown-unknown/release/contract.wasm client/wasm