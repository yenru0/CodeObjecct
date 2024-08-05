import sys

input = sys.stdin.readline


def solve(A: list[int]):
    A.reverse()
    B = []
    cnt = 0
    while A:
        a = A.pop()
        l = len(B)
        for i, b in enumerate(B):
            if b > a:
                B.insert(i, a)
                cnt += l - i
                break
        else:
            B.append(a)

    return cnt


if __name__ == "__main__":
    T = int(input())
    for _ in range(T):
        t, *A = map(int, input().split())
        print(t, solve(A))
