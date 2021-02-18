T = int(input())
M = [1, 1, 1] + [0 for i in range(100 - 3)]
I = [int(input()) for _ in range(T)]

for i in range(3, max(I)):
    M[i] = M[i - 2] + M[i - 3]

for i in I:
    print(M[i - 1])
