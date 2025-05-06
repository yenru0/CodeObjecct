T = int(input())
S = [tuple(map(int, input().split()))for _ in range(T)]; S.sort(key=lambda v: (v[1], v[0]))
[print(i, j)for i, j in S]
