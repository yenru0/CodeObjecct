N, M = map(int, input().split())
K = list(map(int, input().split()));K.sort()
trace = [([v], M, i) for i, v in reversed(list(enumerate(K)))]  # prev, depth, now

while trace:
    p, d, n = trace.pop()
    if d == 1:
        print(" ".join(map(str, p)))
        continue
    for i in range(N-1, n-1, -1):
        trace.append((p + [K[i]], d-1, i))

# 아마도 백트래킹일거임 아마도 그럼 ㅇㅇ
