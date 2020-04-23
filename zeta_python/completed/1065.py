a = int(input())
r = 0
k = []
for i in range(1, a+1):
    k = tuple(map(int, str(i)))
    if sum(k) == len(k)*(k[0] + k[-1]) / 2:
        r += 1
print(r)