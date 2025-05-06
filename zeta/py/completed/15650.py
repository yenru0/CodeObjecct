N, M = map(int, input().split())
trace = [([i], M, i) for i in range(N, 0, -1)]  # prev, depth, now

while trace:
    p, d, n = trace.pop()
    for i in range(N, n, -1):
        trace.append((p + [i], d-1, i))

    if d == 1:
        print(" ".join(map(str, p)))
# 아마도 백트래킹일거임 아마도 그럼 ㅇㅇ
