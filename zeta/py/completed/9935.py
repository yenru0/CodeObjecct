I = input()
R = input()
r = len(R)
before = ""
T = []
for s in I:
    if R[-1] == s:
        if len(T) < r - 1:
            pass
        else:
            for i in range(r - 1):
                if T[1 - r + i] != R[i]:
                    break
            else:
                for i in range(r - 1):
                    T.pop()
                continue
    T.append(s)

if T:
    print("".join(T))
else:
    print("FRULA")
