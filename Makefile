CC=cargo b

default: wasm32-wasi

wasm32-wasi: FORCE
	$(CC) --release \
		--target wasm32-wasi

clean: FORCE
	-rm -r target

FORCE:
