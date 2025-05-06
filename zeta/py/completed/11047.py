N, K = map(int, input().split())
A = [int(input())for i in range(N)]

r = 0
back = 1

while K != 0:
    for i in A:
        if i > K:
            r += K // back
            K %= back
            break
        back = i
    else:
        r += K // back
        K %= back
print(r)