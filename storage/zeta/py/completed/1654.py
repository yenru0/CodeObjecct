import sys

input = sys.stdin.readline


def solve(K, N, E):
    before = 1
    head = max(E)
    mid = (head + before) // 2
    # do
    S = sum(e // mid for e in E)
    while before <= head:
        if S >= N:
            before = mid + 1
        else:
            head = mid - 1
        mid = (before + head) // 2
        S = sum(e // mid for e in E)
    return mid


if __name__ == '__main__':
    K, N = map(int, input().split())
    E = [int(input()) for _ in range(K)]
    print(solve(K, N, E))
