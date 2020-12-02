build:
	@wasm-pack build --release

fmt:
	@cargo fix --allow-staged --allow-dirty
	@cargo fmt

bench:
	@cargo bench

WASM_BARCODE_READER_ZIP = https://github.com/mkazutaka/rust-github-action-experiment/releases/download/test/wasm-barcode-reader-images.zip

.PHONY: images
images:
	@curl -L $(WASM_BARCODE_READER_ZIP) -o images.zip
	@unzip -o images.zip
	@mkdir -p ./www/static/images
	@mv -f wasm-barcode-reader-images/* ./www/static/images
	@rm -rf wasm-barcode-reader-images images.zip
