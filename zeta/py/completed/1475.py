T = [0 for i in range(9)]
for n in input():
    n = int(n)
    if n == 9:
        n = 6
    T[n] += 1
T[6] = (T[6] + 1) // 2
print(max(T))
