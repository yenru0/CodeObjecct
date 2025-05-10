import sys

input = sys.stdin.readline


def team(a: int):
    return 0 if a <= 4 else 1


if __name__ == "__main__":
    score = [0, 0]
    beforeheads = [-100] * 8
    N = int(input())
    history = [tuple(map(int, input().split())) for _ in range(N)]
    for t, a, b in history:
        tm, oppo = team(a), team(b)
        if tm == oppo:
            continue
        elif t - beforeheads[a - 1] <= 10:
            score[tm] += 50
        score[tm] += 100
        beforeheads[a - 1] = t
    print(" ".join(map(str, score)))
