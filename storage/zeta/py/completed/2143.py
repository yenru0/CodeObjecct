import sys

input = sys.stdin.readline

import bisect


def cum_sum(S):
    s = [0]

    for i in range(len(S)):
        s.append(S[i] + s[-1])

    return s


def interval_sum(cum, a, b):  # b > a
    return cum[b] - cum[a - 1]


def all_interval_sum(cum) -> list[int]:
    ret = []  # none
    start = 1
    while start < len(cum):
        end = start
        while end < len(cum):
            ret.append(interval_sum(cum, start, end))
            end += 1
        start += 1
    return ret


def solve(T, A, B):
    cum_A = cum_sum(A)
    cum_B = cum_sum(B)
    I_A = all_interval_sum(cum_A)
    I_B = all_interval_sum(cum_B)
    I_A.sort()
    I_B.sort()
    return sum(
        [
            bisect.bisect_right(I_B, T - I_A[i]) - bisect.bisect_left(I_B, T - I_A[i])
            for i in range(len(I_A))
        ]
    )


if __name__ == "__main__":
    T = int(input())
    N = int(input())
    A = list(map(int, input().split()))
    M = int(input())
    B = list(map(int, input().split()))
    print(solve(T, A, B))
