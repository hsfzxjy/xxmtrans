PROJECT_NAME=xxmtrans

.PHONY: release
release:
	wasm-pack build --target no-modules --no-typescript --no-pack --mode no-install --release --out-dir web/pkg
	ls web/pkg -halt

.PHONY: dopt
dopt:
	wasm2wat pkg/$(PROJECT_NAME).wasm | less

.PHONY: test
test:
	wasm-pack test --node