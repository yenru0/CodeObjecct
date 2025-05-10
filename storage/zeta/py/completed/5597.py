L = [i + 1 for i in range(30)]
for _ in range(28):
    i = int(input()) - 1
    L[i] = False

for i in L:
    print(i) if i else 0
