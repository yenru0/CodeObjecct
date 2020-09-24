N = int(input())
p = 2
while N % p == 0:
    N //= p
    print(p)
p += 1
while N != 1:
    if N % p == 0:
        N //= p
        print(p)
    else:
        p += 2
