from collections import deque


def solve(N, K):
    q = deque(range(1, N + 1))
    tq = deque()
    cnt = 0
    ret = []
    while q:
        while q:
            cnt += 1
            cnt %= K
            if cnt % K == 0:
                ret.append(q.popleft())
            else:
                tq.append(q.popleft())
        q = tq
        tq = deque()
    return "<" + ", ".join(map(str, ret)) + ">"


if __name__ == "__main__":
    print(solve(*map(int, input().split())))
