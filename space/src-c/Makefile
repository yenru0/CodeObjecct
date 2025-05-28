CC = gcc

SRC_DIR = ./src

TARGET = main.c

BUILD_DIR = ../../build

OUTPUT = c.out

all: build

build:
	$(CC) -o $(BUILD_DIR)/$(OUTPUT) -O2 -Wall -lm --std=c11 -fsanitize=address,leak,undefined $(SRC_DIR)/$(TARGET)

run: build
	$(BUILD_DIR)/$(OUTPUT)
