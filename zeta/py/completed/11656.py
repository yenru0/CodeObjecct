I = input()
T = set()
for i in range(len(I)):
    T.add(I[i:])
for t in sorted(T):
    print(t)
