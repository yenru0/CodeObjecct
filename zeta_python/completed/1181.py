N = int(input())
S = [input() for i in range(N)]
S = list(set(S))
S.sort(key=lambda v: (len(v), v))
for s in S:
    print(s)
