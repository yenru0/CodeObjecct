A, B = map(int, input().split())
P = [2]
cnt = 0
t = 4
while B >= t:
    if A <= t <= B:
        cnt += 1
    t *= 2
for i in range(3, int(B ** (1 / 2)) + 1, 2):
    for p in P:
        if i % p == 0:
            break
        elif i < p ** 2:
            P.append(i)
            t = i * i
            while B >= t:
                if A <= t <= B:
                    cnt += 1
                t *= i
            break
    else:
        P.append(i)
        t = i * i
        while B >= t:
            if A <= t <= B:
                cnt += 1
            t *= i
print(cnt)  # 매우 빨라지고 싶은 에라토스테네스 내장형++
