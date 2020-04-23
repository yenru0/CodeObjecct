S = lambda n: n*(n+1)//2
N = int(input())
nor = round((2*N)**.5)
t = N - S(nor-1)
k = S(nor) - S(nor-1) 
if nor % 2:
    print("%d/%d"%(k-t+1, t))
else:
    print("%d/%d"%(t, k-t+1))
