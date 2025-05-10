N, M = map(int, input().split())
trace = [([i], M) for i in range(N, 0, -1)]  # prev, depth

while trace:
    p, d = trace.pop()
    if d == 1:
        print(" ".join(map(str, p)))
        continue
    for i in range(N, 0, -1):
        trace.append((p + [i], d-1))
# 킹트래킹
