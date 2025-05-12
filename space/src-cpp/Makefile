CC = g++

SRC_DIR = ./src

TARGET = main.cpp

BUILD_DIR = ../../build

OUTPUT = cpp.out

all: build

build:
	$(CC) -o $(BUILD_DIR)/$(OUTPUT) -O2 -Wall -lm --std=c++17 -fsanitize=address,leak,undefined $(SRC_DIR)/$(TARGET)

run: build
	$(BUILD_DIR)/$(OUTPUT)
