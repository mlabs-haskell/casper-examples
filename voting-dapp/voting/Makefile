build-casper:
	cd contracts && cargo odra build -b casper

deploy-via-livenet:
	cd contracts \
	&& cargo odra build -b casper \
	&& cargo run --bin deploy --features "casper-livenet" --no-default-features

livenet-debug:
	cd contracts \
	&& cargo odra build -b casper \
	&& cargo run --bin livenet-debug --features casper-livenet --no-default-features

test-w-mock-vm:
	cd contracts && cargo odra test

test-w-casper:
	cd contracts && cargo odra test -b casper

run-query-service:
	cp contracts/.env query-service/ \
	&& cd query-service && cargo run

build-query-service:
	cd query-service && cargo build

clean:
	cargo clean && cd contracts && cargo odra clean

