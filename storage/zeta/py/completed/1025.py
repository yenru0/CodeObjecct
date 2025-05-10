import sys

input = sys.stdin.readline


# 제곱수
def check_square(x):
    return (x ** (1 / 2)).is_integer()


def solve(N: int, M: int, I: list[list[str]]) -> int:  # length
    maxed = -1
    # 전탐색
    for n in range(N):
        for m in range(M):
            for dn in range(-N, N):
                for dm in range(-M, M):
                    if dn == 0 and dm == 0:
                        continue
                    s = ''
                    start = [n, m]
                    while 0 <= start[0] < N and 0 <= start[1] < M:
                        s += I[start[0]][start[1]]
                        if check_square(int(s)):
                            maxed = max(maxed, int(s))
                        start[0] += dn
                        start[1] += dm

    return maxed


if __name__ == '__main__':
    N, M = map(int, input().split())
    I = [list(map(str, input().rstrip())) for i in range(N)]
    print(solve(N, M, I))
