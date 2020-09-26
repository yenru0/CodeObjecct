N = int(input())
I = sorted([int(input())for i in range(N)])
p = min([(j-i) for i, j in zip(I, I[1:])])
print(p)
T = [t for t in range(1, int(p**(1/2))+1) if p % t == 0]
if T[-1] == p // T[-1]:
    print(" ".join([str(t) for t in T[1:-1]] + [str(p // t) for t in T[::-1]]))
else:
    print(" ".join([str(t) for t in T[1:]] + [str(p // t) for t in T[::-1]]))
