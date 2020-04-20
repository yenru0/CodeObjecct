A = [int(input()) for i in range(9)]
I = 0
M = 0
for i, v in enumerate(A):
    if v > M:
        I = i
        M = v
print(M)
print(I+1)