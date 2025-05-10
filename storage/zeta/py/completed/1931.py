N = int(input())
I = [tuple(map(int, input().split())) for _ in range(N)]

I.sort(key=lambda x: (x[1], x[0]))

last = 0
cnt = 0
for i in range(N):
    if last <= I[i][0]:
        last = I[i][1]
        cnt += 1

print(cnt)
