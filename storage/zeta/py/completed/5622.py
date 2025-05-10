ss = 0
for n in input():
    for i, s in enumerate(["ABC", "DEF", "GHI", "JKL", "MNO", "PQRS", "TUV", "WXYZ"]):
        if n in s:
            ss += i+2+1


print(ss)