import sys

input = sys.stdin.readline

if __name__ == "__main__":
    s = 0
    while (n := int(input())) != -1:
        s += n

    print(s)
