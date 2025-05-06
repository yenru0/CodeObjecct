A = list(map(lambda x: x%42, (int(input()) for i in range(10))))
c = 0
for i in A:
    if i == -1:
        continue
    for k in range(10):
        if A[k] == i:
            A[k] = -1
    c += 1

print(c)
