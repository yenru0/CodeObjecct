N, M = map(int, input().split())
trace = [([i], M, i) for i in range(N, 0, -1)]  # prev, depth, now

while trace:
    p, d, n = trace.pop()
    if d == 1:
        print(" ".join(map(str, p)))
        continue
    for i in range(N, n-1, -1):
        trace.append((p + [i], d-1, i))

# 코드 재사용 팝트래킹
