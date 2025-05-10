N = int(input())

def seps(n):
    return n + sum(map(int, str(n)))

for i in range(1 if N < 101 else N-100, N):
    if seps(i) == N:
        print(i)
        break
else:
    print(0)