import sys
import os
import io

input = io.BytesIO(os.read(0, os.fstat(0).st_size)).readline

sys.setrecursionlimit(1000000)

Mem: list
D: list
E: list


def get_time(now):
    if Mem[now] != -1:
        return Mem[now]
    elif not E[now]:
        Mem[now] = D[now]
        return D[now]
    else:
        s = max([get_time(target) for target in E[now]]) + D[now]
        Mem[now] = s
        return s


if __name__ == "__main__":
    T = int(input())
    for _ in range(T):
        N, K = map(int, input().split())
        D = [0] + list(map(int, input().split()))
        E = [[] for _ in range(N + 1)]
        for _ in range(K):
            u, v = map(int, input().split())
            E[v].append(u)
        W = int(input())
        Mem = [-1] * (N + 1)
        print(get_time(W))
