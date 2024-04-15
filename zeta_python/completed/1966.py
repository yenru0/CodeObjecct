import sys

input = sys.stdin.readline


# 너무 개같이 짬 PriorityQueue로 구현할 수 있지 않을까?
def case(N, M, I):
    D = [(v, i) for i, v in enumerate(I)]
    cnt = 0
    while D:
        most = max(D)[0]
        w, ind = D.pop(0)
        if w == most:
            cnt += 1
            if ind == M:  # if target
                return cnt
        else:
            D.append((w, ind))


if __name__ == "__main__":
    T = int(input())
    for _ in range(T):
        print(case(*map(int, input().split()), list(map(int, input().split()))))
