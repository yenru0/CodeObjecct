N, M = map(int, input().split())
trace = [(sorted(list(map(int, input().split()))), M, [])]

while trace:
    n, m, d = trace.pop()
    for i in range(len(n) - 1, -1, -1):
        trace.append((n[:i] + n[i + 1:], m - 1, d + [n[i]]))
    if m == 0:
        print(" ".join(map(str, d)))
# 아마도 백트래킹일거임 아마도 그럼 ㅇㅇ
