N = int(input())
for s in sorted([tuple(map(str, input().split())) for i in range(N)], key=lambda v: int(v[0])): print(s[0], s[1])
