A, B = map(int, input().split())
p = 1
for i in range(A, 1, -1):
    if A % i == 0 and B % i == 0:
        p = i
        break
print(p)
print(A*B//p)