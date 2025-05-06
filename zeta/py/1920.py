N = int(input())
A = list(map(int, input().split()))
M = int(input())
K = list(map(int, input().split()))

for k in K:
    if k in A:
        print(1)
    else:
        print(0)