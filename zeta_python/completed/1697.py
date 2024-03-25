from queue import Queue


def solve(N, K):
    D = Queue()
    M = [0] * 100001
    D.put((N, 0))
    while D:
        now, t = D.get()
        if now > 100000 or now < 0:
            continue
        if M[now] == 0:
            M[now] = t
        elif t > M[now]:
            continue

        target = []

        if now > K:
            target.append((now - 1, t + 1))
        elif now == K:
            return t
        elif now < K:
            target.append((2 * now, t + 1))
            target.append((now + 1, t + 1))
            target.append((now - 1, t + 1))
        for temp in target:
            D.put(temp)


if __name__ == '__main__':
    N, K = map(int, input().split())
    print(solve(N, K))
