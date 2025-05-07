CC = luac

SRC_DIR = ./src

TARGET = main.lua

BUILD_DIR = ../../build

build:
	${CC} -o ${BUILD_DIR}/lua.luac ${SRC_DIR}/${TARGET}

run: build
	lua ${BUILD_DIR}/lua.luac