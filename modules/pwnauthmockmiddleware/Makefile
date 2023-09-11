# Define the directory names
PKG_DIR = pkg
TARGET_DIR = target

.PHONY: build-and-publish

build-and-publish:
	# Run 'wasm-pack build --target bundler'
	wasm-pack build --target bundler
	# Change to the 'pkg' directory, check w/ 'pwd ; ls' -> 'publish'
	cd $(PKG_DIR); pwd; ls ; npm publish

clean:
	# Remove the pkg and target directories if they exist
	rm -rf $(PKG_DIR) $(TARGET_DIR)

