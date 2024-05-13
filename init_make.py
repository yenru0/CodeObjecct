import sys
import os
import glob

TC_TARGET_DIR = "_testcases"

DEFAULT_COUNT = 1

if __name__ == "__main__":
    count = DEFAULT_COUNT
    match len(sys.argv):
        case 1:
            pass
        case 2 if sys.argv[1].isdigit() and int(sys.argv[1]) > 0:
            count = int(sys.argv[1])
        case _:
            print("Invalid arguments.")
            sys.exit()

    # default make
    if not os.path.exists("./" + TC_TARGET_DIR):
        os.mkdir(TC_TARGET_DIR)

    for i in range(1, count + 1):
        default_in_path = "./" + TC_TARGET_DIR + f"/{i}.in"
        default_out_path = "./" + TC_TARGET_DIR + f"/{i}.out"
        with open(default_in_path, "w", encoding="utf-8") as _:
            pass

        with open(default_out_path, "w", encoding="utf-8") as _:
            pass
