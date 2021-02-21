N = int(input())

T = [0, ]

for i in range(1, N):
    new = i + 1
    t = []
    if new % 3 == 0:
        t.append(T[new // 3 - 1] + 1)
    if new % 2 == 0:
        t.append(T[new // 2 - 1] + 1)
    t.append(T[new - 1 - 1] + 1)
    T.append(min(t))

print(T[-1])
