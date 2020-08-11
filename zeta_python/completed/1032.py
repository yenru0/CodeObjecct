N = int(input())
K = [input() for i in range(N)]
shared = K[0]

for k in K:
    for i, c in enumerate(k):
        if c == "?":
            pass
        elif shared[i] == c:
            pass
        else:
            shared = shared[:i] + "?" + shared[i + 1:]
print(shared)
