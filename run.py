import os
import sys
import pathlib
from enum import Enum, unique, auto
from dataclasses import dataclass
import re
import subprocess

import click

TC_TARGET_DIR = "./_testcases"


@dataclass
class ProblemRunType:
    name: str
    dir: str
    runner: str
    prefix: str
    prerunner: str = ""


class ProblemRunEnum:
    zeta_python: ProblemRunType = ProblemRunType(
        name="zeta_python",
        dir="./zeta_python",
        runner="python {source}",
        prefix="py",
    )
    zeta_C: ProblemRunType = ProblemRunType(
        name="zeta_C",
        dir="./zeta_C",
        runner="./a.out",
        prefix="c",
        prerunner="gcc -std=c11 {source}",
    )
    zeta_cpp: ProblemRunType = ProblemRunType(
        name="zeta_cpp",
        dir="./zeta_cpp",
        runner="./a.out",
        prefix="cpp",
        prerunner="g++ -std=c++17 {source}",
    )
    zeta_kotlin: ProblemRunType = ProblemRunType(
        name="zeta_kotlin",
        dir="./zeta_kotlin",
        runner="java -jar a.jar -Dfile.encoding=UTF-8 < {input}",
        prefix="kt",
        prerunner="kotlinc-jvm {source} -include-runtime -d a.jar",
    )
    # WIP
    zeta_lua: ProblemRunType = ProblemRunType(
        name="zeta_lua", dir="./zeta_lua", runner="", prefix="lua"
    )
    # ExceptionType
    err: ProblemRunType = ProblemRunType(name="err", dir="", runner="", prefix="")

    @staticmethod
    def quicklink(link: str) -> ProblemRunType:
        match link.lower().replace("-", "").replace("_", ""):
            case "zetapython" | "bojpython" | "bojpy" | "zetapy" | "zpy":
                return ProblemRunEnum.zeta_python
            case "zeta_c" | "zetac" | "boj_c" | "bojc" | "zc":
                return ProblemRunEnum.zeta_C
            case "zeta_cpp" | "zetacpp" | "boj_cpp" | "bojcpp" | "zcpp":
                return ProblemRunEnum.zeta_cpp
            case "zeta_lua" | "zetalua" | "boj_lua" | "bojlua" | "zlua":
                return ProblemRunEnum.zeta_lua
            case "zetakotlin" | "bojkotlin" | "zetakt" | "bojkt" | "zkt":
                return ProblemRunEnum.zeta_kotlin
            case _:
                return ProblemRunEnum.err


@click.group()
def cli():
    pass


@click.command()
@click.argument("runtype_string", type=str, nargs=1, required=True)
@click.argument("problem_code", type=str, nargs=1, required=True)
@click.argument("count", type=int, nargs=1, required=False, default=1)
@click.option("--verbose", "-v", is_flag=True)
@click.option("--profile", "-p", is_flag=True)
def run(
    runtype_string: str, problem_code: str, count: int, verbose: bool, profile: bool
):
    runtype: ProblemRunType = ProblemRunEnum.quicklink(runtype_string)
    if runtype is ProblemRunEnum.err:
        raise click.UsageError("Error RUNTYPE.")

    if count < 1:
        raise click.UsageError("COUNT must be greater than 1.")

    problem_code_path = f"{runtype.dir}/{problem_code}.{runtype.prefix}"

    if not os.path.exists(problem_code_path):
        raise click.UsageError("Problem does not exist.")

    tc_files: list[str] = [
        fpath
        for fpath in os.listdir(TC_TARGET_DIR)
        if os.path.isfile(os.path.join(TC_TARGET_DIR, fpath))
    ]

    for c in range(1, count + 1):
        inpf = None
        outputfs = []
        for name in tc_files:
            ix = re.match(f"{c}.in", name)
            ox = re.match(f"{c}.out[0-9]*", name)
            if ix:
                inpf = name
            elif ox:
                outputfs.append(name)

        if (not inpf) or (not outputfs):
            click.echo(f"{c:02d}: {None}")
            continue
        else:
            # check
            with open(os.path.join(TC_TARGET_DIR, inpf), "r", encoding="utf-8") as f:
                if runtype.prerunner:
                    try:
                        _ = subprocess.run(
                            runtype.prerunner.format(
                                source=problem_code_path,
                            ).split(),
                            check=True,
                            capture_output=True,
                            text=True,
                        )
                    except subprocess.CalledProcessError as e:
                        click.echo(">>>>>> [Error while PRERUNNER process] >>>>>>")
                        click.echo(f"{e.stderr}")
                        click.echo(">>>>>> [Error while PRERUNNER process] >>>>>>")
                        continue
                try:
                    stdout = subprocess.run(
                        runtype.runner.format(
                            source=problem_code_path,
                        ).split(),
                        check=True,
                        capture_output=True,
                        text=True,
                        stdin=f,
                    ).stdout
                except subprocess.CalledProcessError as e:
                    click.echo(">>>>>> [Error while RUNNER process] >>>>>>")
                    click.echo(f"{e.stderr}")
                    click.echo(f"returncode: {e.returncode}")
                    click.echo(">>>>>> [Error while RUNNER process] >>>>>>")
                    stdout = ""
                    continue

                flag = False
                for outputpath in outputfs:
                    with open(
                        os.path.join(TC_TARGET_DIR, outputpath), "r", encoding="utf-8"
                    ) as f:
                        if stdout.strip() == f.read().strip():
                            flag = True
                            break

                click.echo(f"{c:02d}: {flag}")
                if verbose:
                    click.echo("===" * 3)
                    click.echo(stdout)
                    click.echo("===" * 3)


@click.command(name="init")
@click.argument("count", type=int)
def init(count: int):
    if count < 1:
        raise click.UsageError("COUNT must be greater than 1.")
    else:
        if not os.path.exists("./" + TC_TARGET_DIR):
            os.mkdir(TC_TARGET_DIR)
        for i in range(1, count + 1):
            default_in_path = "./" + TC_TARGET_DIR + f"/{i}.in"
            default_out_path = "./" + TC_TARGET_DIR + f"/{i}.out"
            with open(default_in_path, "w", encoding="utf-8") as _:
                pass

            with open(default_out_path, "w", encoding="utf-8") as _:
                pass

        click.echo("Testcases was made successfully!")


cli.add_command(run)
cli.add_command(init)

if __name__ == "__main__":
    cli()
