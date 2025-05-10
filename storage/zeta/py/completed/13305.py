import sys


def solve(N, E, C):
    S = 0

    m = C[0]
    for i in range(0, len(E)):
        if C[i] <= m:
            m = C[i]
        S += m * E[i]

    return S


if __name__ == '__main__':
    print(solve(int(sys.stdin.readline()),
                list(map(int, sys.stdin.readline().split())),
                list(map(int, sys.stdin.readline().split()))))
