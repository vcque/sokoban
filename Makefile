all: build test
clean:
	rm -rf $(BUILD_DIR)

BUILD=rustc
TEST=rustc --test
BUILD_DIR=bin
SRC_DIR=src
RUSTARGS=--out-dir $(BUILD_DIR)

sokoban: 
	$(BUILD) $(RUSTARGS) $(SRC_DIR)/sokoban.rs

test: 
	$(TEST) $(RUSTARGS) $(SRC_DIR)/sokoban.rs
	./$(BUILD_DIR)/sokoban

build: sokoban
