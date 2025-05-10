T = list(map(int, input().split()))
for i in range(2):
    for j in range(2 - i):
        if T[j] > T[j + 1]:
            T[j], T[j + 1] = T[j + 1], T[j]
for i in T:
    print(i, end=' ')  # 버블
