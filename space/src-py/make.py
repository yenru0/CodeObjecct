#!/usr/bin/env python3
import py_compile

import click

SRC_DIR = r"./src"

TARGET = r"main.py"

OUTPUT = r"py.pyc"

BUILD_DIR = r"../../build"


@click.group()
def cli():
    pass


@click.command(name="build")
def build():
    try:
        py_compile.compile(
            f"{SRC_DIR}/{TARGET}", f"{BUILD_DIR}/{OUTPUT}", optimize=2, doraise=True
        )
    except Exception as e:
        raise click.ClickException(e)


cli.add_command(build)

if __name__ == "__main__":
    cli()
