import sys
from collections import deque

input = sys.stdin.readline

round = lambda x: int(x) + 1 if x - int(x) >= 0.5 else int(x)

if __name__ == "__main__":
    N = int(input())
    A = [int(input()) for _ in range(N)] if N != 0 else [0]
    A.sort()
    A = deque(A)
    pcs = round(N * 0.15)
    for _ in range(pcs):
        A.popleft()
        A.pop()

    print(round(sum(A) / len(A)))
