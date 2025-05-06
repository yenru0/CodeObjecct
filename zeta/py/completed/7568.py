N = int(input())
I = [list(map(int, input().split())) for i in range(N)]
K = []
for i, v1 in enumerate(I):
    k = 1
    for j, v2 in enumerate(I):
        if i == j:
            continue
        if v1[0] < v2[0] and v1[1] < v2[1]:
            k += 1

    K.append(str(k))
print(" ".join(K))
