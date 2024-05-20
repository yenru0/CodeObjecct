import sys
import os
import io

sys.setrecursionlimit(10001)
input = io.BytesIO(os.read(0, os.fstat(0).st_size)).readline


P: list
N: int
K: int
Mem: list


def get_possible(C):
    if Mem[C] != -1:
        return Mem[C]
    m = float("inf")
    for p in P:
        if C - p > 0:
            m = min([get_possible(C - p) + 1, m])
        elif C == p:
            m = 1
        elif C - p < 0:
            continue
    Mem[C] = m
    return m


if __name__ == "__main__":
    N, K = map(int, input().split())
    P = [int(input()) for _ in range(N)]
    Mem = [-1] * (K + 1)
    ret = get_possible(K)
    if ret is float("inf"):
        print(-1)
    else:
        print(ret)
