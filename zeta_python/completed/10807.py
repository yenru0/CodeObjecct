N = int(input())
L = list(map(int, input().split()))
v = int(input())
c = 0
for x in L:
    if x == v:
        c += 1
print(c)