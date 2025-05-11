CC = luac

SRC_DIR = ./src

TARGET = main.lua

BUILD_DIR = ../../build

OUTPUT = lua.luac

build:
	${CC} -o ${BUILD_DIR}/$(OUTPUT) ${SRC_DIR}/${TARGET}

run: build
	lua ${BUILD_DIR}/$(OUTPUT)