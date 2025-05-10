import sys
N = int(sys.stdin.readline())
C = [0] * 8001
I = []
for i in range(N):
    t = int(sys.stdin.readline())
    C[t + 4000] += 1
    I.append(t)
print(round(sum(I)/N))
I.sort()
print(I[N//2])
M, m = I[-1], I[0]

mor = 0
ner = 0
cnd = 0
for i, c in enumerate(C):
    if c > mor:
        mor = c
        cnd = 0
        ner = i
    elif c == mor:
        if cnd == 0:
            cnd = 1
            ner = i
        elif cnd == 1:
            pass
print(ner - 4000)

print(abs(M-m))
