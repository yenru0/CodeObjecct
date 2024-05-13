import os
import sys
import pathlib
from enum import Enum, unique, auto
from dataclasses import dataclass
import re
import subprocess

TC_TARGET_DIR = "./_testcases"


@dataclass
class ProblemRunType:
    name: str
    dir: str
    runner: str
    prefix: str


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
        runner="gcc -std=c11 {source} && ./a.out < {input}",
        prefix="c",
    )
    zeta_cpp: ProblemRunType = ProblemRunType(
        name="zeta_cpp",
        dir="./zeta_cpp",
        runner="g++ -std=c++17 {source} && ./a.out < {input}",
        prefix="cpp",
    )
    zeta_kotlin: ProblemRunType = ProblemRunType(
        name="zeta_kotlin",
        dir="./zeta_kotlin",
        runner="kotlinc-jvm {source} -include-runtime -d a.jar && java -jar a.jar -Dfile.encoding=UTF-8 < {input}",
        prefix="kt",
    )
    zeta_lua: ProblemRunType = ProblemRunType(
        name="zeta_lua", dir="./zeta_lua", runner="", prefix="lua"
    )
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


if __name__ == "__main__":
    rtype: ProblemRunType
    problem_code: int
    cnt: int
    verbose_flag: bool = False
    match len(sys.argv):
        case 3:
            rtype = ProblemRunEnum.quicklink(sys.argv[1])
            problem_code = sys.argv[2] if sys.argv[2].isdigit() else None
            cnt = 1

        case 4:
            rtype = ProblemRunEnum.quicklink(sys.argv[1])
            problem_code = int(sys.argv[2]) if sys.argv[2].isdigit() else None
            if sys.argv[3] == "--verbose":
                verbose_flag = True
                cnt = 1
            else:
                cnt = (
                    int(sys.argv[3])
                    if sys.argv[3].isdigit() and int(sys.argv[3]) > 0
                    else 1
                )
        case 5:
            rtype = ProblemRunEnum.quicklink(sys.argv[1])
            problem_code = int(sys.argv[2]) if sys.argv[2].isdigit() else None

            cnt = (
                int(sys.argv[3])
                if sys.argv[3].isdigit() and int(sys.argv[3]) > 0
                else 1
            )

            if sys.argv[4] == "--verbose":
                verbose_flag = True
            else:
                pass
        case _:
            print("Invalid argument count.")
            sys.exit()

    if rtype is ProblemRunEnum.err:
        print(f"Invalid run type: `{sys.argv[1]}`.")
        sys.exit()

    problem_code_path = f"{rtype.dir}/{problem_code}.{rtype.prefix}"
    if not os.path.exists(problem_code_path):
        print(f"Invalid problem code: `{problem_code}`")
        sys.exit()

    io_files: list[str] = [
        fpath
        for fpath in os.listdir(TC_TARGET_DIR)
        if os.path.isfile(os.path.join(TC_TARGET_DIR, fpath))
    ]

    for c in range(1, cnt + 1):
        inputs = None
        outputs = []
        for name in io_files:
            ix = re.match(f"{c}.in", name)
            ox = re.match(f"{c}.out[0-9]*", name)
            if ix:
                inputs = name
            elif ox:
                outputs.append(name)
        if (not inputs) or (not outputs):
            print(f"{c:02d}: Pass")
            continue
        else:
            with open(os.path.join(TC_TARGET_DIR, inputs), 'r', encoding='utf-8') as inputf:
                stdout = subprocess.run(
                    rtype.runner.format(
                        source=problem_code_path, input= os.path.join(TC_TARGET_DIR, inputs)
                    ).split(),
                    check=True,
                    capture_output=True,
                    text=True,
                    stdin = inputf
                ).stdout

            flag = False
            for outputpath in outputs:
                with open(
                    os.path.join(TC_TARGET_DIR, outputpath), "r", encoding="utf-8"
                ) as f:
                    if stdout.strip() == f.read().strip():
                        flag = True
                        break

            print(f"{c:02d}: {flag}")
            if verbose_flag:
                print("===")
                print(stdout)
                print("===")


### TODO: --profile argument 추가를 위한 command dispatcher 개선
### TODO: WIP
