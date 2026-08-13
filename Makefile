TARGETS := x86_64-apple-darwin aarch64-apple-darwin x86_64-pc-windows-gnu
RELEASE_DIR := release

.PHONY: build release clean

build:
	@for t in $(TARGETS); do \
		echo "=== $$t ==="; \
		cargo build --release --target $$t; \
	done

release: build
	@rm -rf $(RELEASE_DIR)
	@mkdir -p $(RELEASE_DIR)
	@# macOS universal (Intel + Apple Silicon)
	@mkdir -p $(RELEASE_DIR)/drifter-macos-universal
	@lipo -create -output $(RELEASE_DIR)/drifter-macos-universal/drifter \
		target/x86_64-apple-darwin/release/drifter \
		target/aarch64-apple-darwin/release/drifter
	@chmod +x $(RELEASE_DIR)/drifter-macos-universal/drifter
	@tar czf $(RELEASE_DIR)/drifter-macos-universal.tar.gz -C $(RELEASE_DIR) drifter-macos-universal
	@rm -rf $(RELEASE_DIR)/drifter-macos-universal
	@# macOS per-arch
	@for arch in x86_64 aarch64; do \
		mkdir -p $(RELEASE_DIR)/drifter-macos-$$arch; \
		cp target/$$arch-apple-darwin/release/drifter $(RELEASE_DIR)/drifter-macos-$$arch/drifter; \
		chmod +x $(RELEASE_DIR)/drifter-macos-$$arch/drifter; \
		tar czf $(RELEASE_DIR)/drifter-macos-$$arch.tar.gz -C $(RELEASE_DIR) drifter-macos-$$arch; \
		rm -rf $(RELEASE_DIR)/drifter-macos-$$arch; \
	done
	@# Windows
	@mkdir -p $(RELEASE_DIR)/drifter-windows-x86_64
	@cp target/x86_64-pc-windows-gnu/release/drifter.exe $(RELEASE_DIR)/drifter-windows-x86_64/drifter.exe
	@cd $(RELEASE_DIR) && zip -q drifter-windows-x86_64.zip drifter-windows-x86_64/drifter.exe
	@rm -rf $(RELEASE_DIR)/drifter-windows-x86_64
	@echo "=== release ==="
	@ls -lh $(RELEASE_DIR)/

clean:
	cargo clean
	rm -rf $(RELEASE_DIR) dist
