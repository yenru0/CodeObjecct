#!/usr/bin/env python3
import os
import sys
import pathlib
from enum import Enum, unique, auto
from dataclasses import dataclass
import re
import subprocess
import yaml

import click

CFG_PATH = "./config.yml"
STATE_PATH = "./state.yml"

TC_DIR = "./_testcases"

SRC_SPACE_DIR = "./space"

STORAGE_DIR = "./storage"

BUILD_DIR = "./build"


@unique
class Language(Enum):
    PYTHON = "py"
    C = "c"
    CPP = "cpp"
    RUST = "rs"
    KOTLIN = "kt"
    LUA = "lua"

    UNDEFINED = "undefined"

    @property
    def build_command(self):
        return {
            Language.PYTHON: [
                "python",
                "./make.py",
                "build",
            ],
            Language.C: [
                "make",
                "build",
            ],
            Language.CPP: [
                "make",
                "build",
            ],
            Language.RUST: [
                "cargo",
                "build",
            ],
            Language.KOTLIN: [
                "./gradlew",
                "jar",
            ],
            Language.LUA: [
                "make",
                "build",
            ],
        }[self]

    @property
    def working_dir(self):
        return f"{SRC_SPACE_DIR}/src-{self.value}"

    @property
    def build_executable(self):
        return {
            Language.PYTHON: "py.pyc",
            Language.C: "c.out",
            Language.CPP: "cpp.out",
            Language.RUST: "rs.out",
            Language.KOTLIN: "kt.jar",
            Language.LUA: "lua.luac",
        }[self]

    def run_executable_command(self, executable_loc):
        return {
            Language.PYTHON: ["python", f"{executable_loc}"],
            Language.C: [f"{executable_loc}"],
            Language.CPP: [f"{executable_loc}"],
            Language.RUST: [f"{executable_loc}"],
            Language.KOTLIN: ["java", "-jar", f"{executable_loc}"],
            Language.LUA: ["lua", f"{executable_loc}"],
        }[self]

    @staticmethod
    def convert_ext(ext: str):
        match (ext):
            case "py":
                return Language.PYTHON
            case "c":
                return Language.C
            case "cpp":
                return Language.CPP
            case "rs":
                return Language.RUST
            case "kt":
                return Language.KOTLIN
            case "lua":
                return Language.LUA
            case _:
                return Language.UNDEFINED

    @staticmethod
    def convert_name(name: str):
        match (name.lower()):
            case "py" | "py3" | "python" | "python3":
                return Language.PYTHON
            case "c":
                return Language.C
            case "cpp" | "c++":
                return Language.CPP
            case "rust" | "rs":
                return Language.RUST
            case "kt" | "kotlin":
                return Language.KOTLIN
            case "lua":
                return Language.LUA
            case _:
                return Language.UNDEFINED


def parse_range_string(range_str):
    try:
        if range_str.startswith(".."):
            # 형식: "..N" → 1부터 N까지
            end = int(range_str[2:])
            return list(range(1, end + 1))
        else:
            # 형식: "M..N" → M부터 N까지
            parts = range_str.split("..")
            if len(parts) == 1:
                # 단일 숫자 (예: "1")
                return [int(parts[0])]
            elif len(parts) == 2:
                # 범위 (예: "3..5")
                start = int(parts[0])
                end = int(parts[1])
                return list(range(start, end + 1))
            else:
                raise ValueError("Invalid range format")
    except ValueError as e:
        print(f"Error: {e}")
        return []


def parse_range_string_list(str_list):
    result = []
    for s in str_list:
        result.extend(parse_range_string(s))
    return result


@click.group()
def cli():
    pass


@click.command(name="run")
@click.option("--from", "from_", type=str, required=True)
@click.option("--target", "-t", default=["1"], multiple=True)
@click.option("--verbose/--no-verbose", "-v/-nv", default=True)
def run(from_: str, target: str, verbose: bool):
    # Language 확인
    from_language: Language = Language.convert_name(from_)

    if from_language is Language.UNDEFINED:
        raise click.ClickException("Undefined Language Exception")
    try:
        with open("state.yml", "r", encoding="utf-8") as f:
            state = yaml.safe_load(f)
    except FileNotFoundError as e:
        raise click.ClickException(e)
    except yaml.YAMLError as e:
        raise click.ClickException(e)

    if "space" not in state:
        raise click.ClickException("State(state.yml) Exception")
    elif not state["space"]:
        click.secho("state.space has no member", fg="yellow", bold=True)
        state["space"] = {}

    lang_space_state: dict | None
    if from_language.value not in state["space"]:
        lang_space_state = None
    else:
        lang_space_state = state["space"][from_language.value]

    if lang_space_state is None or not lang_space_state:
        raise click.ClickException("Run Exception: There are no pre-defined state")

    # 테케 분석
    try:
        tcs: list[int] = parse_range_string_list(target)
    except ValueError as e:
        raise click.ClickException(e)

    # build

    try:
        s = subprocess.run(
            from_language.build_command,
            check=True,
            capture_output=True,
            text=True,
            cwd=from_language.working_dir,
        )
        print(s.stderr)
        
    except subprocess.CalledProcessError as e:
        click.echo(">>>>>> [Error while BUILD process] >>>>>>")
        raise click.ClickException(e.stderr)

    build_executable = from_language.build_executable

    for tc in tcs:
        if not (
            os.path.isfile(f"{TC_DIR}/{tc}.in") and os.path.isfile(f"{TC_DIR}/{tc}.out")
        ):
            click.echo(f"{tc:02d}: {None}")
        else:
            f_in_path = f"{TC_DIR}/{tc}.in"
            f_out_path = f"{TC_DIR}/{tc}.out"
            with open(f_in_path, "r", encoding="utf-8") as f_in:
                try:
                    stdout = subprocess.run(
                        from_language.run_executable_command(
                            f"{BUILD_DIR}/{build_executable}"
                        ),
                        check=True,
                        capture_output=True,
                        text=True,
                        stdin=f_in,
                    ).stdout
                except subprocess.CalledProcessError as e:
                    click.echo(">>>>>> [Error while RUNNING process] >>>>>>")
                    click.echo(f"{e.stderr}")
                    click.echo(f"returncode: {e.returncode}")
                    stdout = ""
                    continue
            flag: bool = False
            with open(f_out_path, "r", encoding="utf-8") as f_out:
                flag = stdout.strip() == f_out.read().strip()
            click.echo(f"{tc:02d}: {flag}")

            if verbose:
                click.echo("===" * 3)
                click.echo(stdout)
                click.echo("===" * 3)


@click.command(name="load")
@click.argument("file", type=str, nargs=1, required=True)
@click.option("--to", type=str, default="")  # language value
@click.option("--from", "from_", type=str, required=True)
@click.option("--force", "-f", is_flag=True)
def load(file: str, to: str, from_: str, force: bool):
    """
    load specific file into corresponding src-space
    """

    # 파일의 src-space가 어딘지 확인
    to_language: Language
    file_name: str
    file_sep_ext = file.split(".")
    if len(file_sep_ext) == 1:
        file_name = file_sep_ext[0]
        to_language = Language.convert_name(to)  # if blank, undefined
    elif len(file_sep_ext) == 2:
        file_name = file_sep_ext[0]
        to_language = Language.convert_name(file_sep_ext[1])
    else:
        file_name = ""
        to_language = Language.UNDEFINED

    if to_language is Language.UNDEFINED:
        raise click.UsageError("LoadError: `to` or `file` argument is wrong")

    try:
        with open(STATE_PATH, "r", encoding="utf-8") as f_state:
            state = yaml.safe_load(f_state)
    except FileNotFoundError as e:
        raise click.ClickException(e)
    except yaml.YAMLError as e:
        raise click.ClickException(e)

    if "space" not in state:
        raise click.ClickException("state.yml is wrong")
    elif not state["space"]:
        click.secho("state.space has no member", fg="yellow", bold=True)
        state["space"] = {}

    lang_space_state: dict | None
    if to_language.value not in state["space"]:
        lang_space_state = None
    else:
        lang_space_state = state["space"][to_language.value]

    if not force:
        if lang_space_state is None or not lang_space_state:
            pass
        elif lang_space_state["file"] is None:
            pass
        else:
            raise click.ClickException(
                f"File {lang_space_state['file']}.{to_language.value} already in source space {to_language.value}"
            )

    # 파일이 존재하는지 확인
    # uncompleted
    file_is_exist: bool = False
    file_is_completed: bool = False
    file_loc: str = None
    if os.path.isfile(
        f"{STORAGE_DIR}/{from_}/{to_language.value}/{file_name}.{to_language.value}"
    ):
        file_is_exist = True
        file_is_completed = False
        file_loc = (
            f"{STORAGE_DIR}/{from_}/{to_language.value}/{file_name}.{to_language.value}"
        )
    elif os.path.isfile(
        f"{STORAGE_DIR}/{from_}/{to_language.value}/completed/{file_name}.{to_language.value}"
    ):
        file_is_exist = True
        file_is_completed = True
        file_loc = f"{STORAGE_DIR}/{from_}/{to_language.value}/completed/{file_name}.{to_language.value}"
    # 존재하면 move
    # 존재하지 않으면 빈 파일을 생성하고
    # state에 기록함
    if file_is_exist:
        with (
            open(file_loc, "rb") as f_src,
            open(
                f"{SRC_SPACE_DIR}/src-{to_language.value}/src/main.{to_language.value}",
                "wb",
            ) as f_dst,
        ):
            f_dst.write(f_src.read())
        os.remove(file_loc)
    else:
        with open(
            f"{SRC_SPACE_DIR}/src-{to_language.value}/src/main.{to_language.value}",
            "wb",
        ) as f_dst:
            pass

    with open("state.yml", "w", encoding="utf-8") as f:
        if lang_space_state is None:
            state["space"][to_language.value] = {}
        state["space"][to_language.value]["file"] = file_name
        state["space"][to_language.value]["location"] = from_
        state["space"][to_language.value]["is_completed"] = file_is_completed

        yaml.safe_dump(state, f)
    if file_is_exist:
        click.echo(
            f"File {file_name}.{to_language.value} from {from_}{'/completed' if file_is_completed else ''} is successfully loaded"
        )
    else:
        click.echo(f"File {file_name}.{to_language.value} is successfully created")


@click.command(name="export")
@click.option("--from", "from_", type=str, required=True)
@click.option(
    "--completed/--no-completed", "-c/-nc", is_flag=True, flag_value=False, default=None
)
@click.option("--copy", is_flag=True, help="is copy from state or move")
def export(from_: str, completed: bool, copy: bool):
    # Language 알기
    from_language = Language.convert_name(from_)

    if from_language is Language.UNDEFINED:
        raise click.ClickException("Undefined Language Exception")
    try:
        with open("state.yml", "r", encoding="utf-8") as f:
            state = yaml.safe_load(f)
    except FileNotFoundError as e:
        raise click.ClickException(e)
    except yaml.YAMLError as e:
        raise click.ClickException(e)

    if "space" not in state:
        raise click.ClickException("State(state.yml) Exception")
    elif not state["space"]:
        click.secho("state.space has no member", fg="yellow", bold=True)
        state["space"] = {}

    lang_space_state: dict | None
    if from_language.value not in state["space"]:
        lang_space_state = None
    else:
        lang_space_state = state["space"][from_language.value]

    if lang_space_state is None or not lang_space_state:
        raise click.ClickException("Export Exception: There are no pre-defined state")

    s_file = lang_space_state["file"]
    s_loc = lang_space_state["location"]
    s_is_completed = lang_space_state["is_completed"]

    if not s_file:
        raise click.ClickException(
            f"Export Exception: There is no file in {from_language.value} space"
        )

    if not s_loc:
        raise click.ClickException(f"Export Exception: Location {s_loc} is not defined")

    if completed:
        s_is_completed = True

    dest_path: str = (
        f"{STORAGE_DIR}/{s_loc}/{from_language.value}{'/completed' if s_is_completed else ''}/{s_file}.{from_language.value}"
    )
    source_path: str = (
        f"{SRC_SPACE_DIR}/src-{from_language.value}/src/main.{from_language.value}"
    )

    with open(source_path, "rb") as f_src, open(dest_path, "wb") as f_dst:
        f_dst.write(f_src.read())

    click.echo(f"File {s_file}.{from_language.value} successfully ")

    if not copy:
        with open("state.yml", "w", encoding="utf-8") as f:
            state["space"][from_language.value] = {}
            yaml.safe_dump(state, f)

        os.remove(source_path)


@click.command(name="state")
def state():
    try:
        with open(STATE_PATH, "r", encoding="utf-8") as f_state:
            state = yaml.safe_load(f_state)
    except FileNotFoundError as e:
        raise click.ClickException(e)
    except yaml.YAMLError as e:
        raise click.ClickException(e)

    if "space" not in state:
        raise click.ClickException("state.yml is wrong")
    elif not state["space"]:
        click.secho("state.space has no member", fg="yellow", bold=True)
        state["space"] = {}

    for k, v in state["space"].items():
        if not v:
            print(f"{k}: _")
        else:
            print(
                f"{k}: {v['file']}.{k} @ {v['location']}{'/completed' if v['is_completed'] else ''}"
            )


@click.command(name="init")
@click.argument("count", type=int)
def init(count: int):
    """
    init testcases with initial count and state.yml and config.yml
    """
    if count < 1:
        raise click.UsageError("var count should be greater than 1.")

    if not os.path.exists("./" + TC_DIR):
        os.mkdir(TC_DIR)
    for i in range(1, count + 1):
        default_in_path = "./" + TC_DIR + f"/{i}.in"
        default_out_path = "./" + TC_DIR + f"/{i}.out"
        if not os.path.exists(default_in_path):
            with open(default_in_path, "w", encoding="utf-8") as _:
                pass
        if not os.path.exists(default_out_path):
            with open(default_out_path, "w", encoding="utf-8") as _:
                pass

    click.echo(f"1. Testcases was made with count {count} successfully!")

    if not os.path.isfile("./state.yml"):
        click.echo("2-1. state.yml does not exist, creating...")
        with open("state.yml", "w", encoding="utf-8") as f:
            f.write("space:\n")
        click.echo("2-1. state.yml is created successfully!")
    else:
        click.echo("2-1. state.yml exists")
    if not os.path.isfile("./config.yml"):
        click.echo("2-2. config.yml does not exist, creating...")
        with open("config.yml", "w", encoding="utf-8") as f:
            f.write("space:\n")
        click.echo("2-2. config.yml is created successfully!")
    else:
        click.echo("2-2. config.yml exists")


cli.add_command(run)
cli.add_command(load)
cli.add_command(init)
cli.add_command(export)
cli.add_command(state)

if __name__ == "__main__":
    cli()
