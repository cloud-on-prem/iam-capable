# Variables
TARGET = iam-capable
BUILD_DIR = ./target
RELEASE_FLAGS = --release

# Build targets
.PHONY: all
all: test build

.PHONY: test
test:
	@echo "Running tests..."
	@cargo test

.PHONY: build
build: build-macos build-macos-x86

.PHONY: build-macos-x86
build-macos-x86:
	@echo "Building binary for macOS x86..."
	@cargo build $(RELEASE_FLAGS) --target x86_64-apple-darwin
	@mkdir -p $(BUILD_DIR)/macos-x86
	@cp $(BUILD_DIR)/x86_64-apple-darwin/release/$(TARGET) $(BUILD_DIR)/macos-x86

.PHONY: build-macos
build-macos:
	@echo "Building binary for macOS..."
	@cargo build $(RELEASE_FLAGS) --target aarch64-apple-darwin
	@mkdir -p $(BUILD_DIR)/macos
	@cp $(BUILD_DIR)/aarch64-apple-darwin/release/$(TARGET) $(BUILD_DIR)/macos
