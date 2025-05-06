N, M = map(int, input().split())
T = [input() for i in range(N)]
Min = 10000000000000

for i in range(N-7):
    for j in range(M-7):
        temp = [t[j:j+8] for t in T[i:i+8]]
        start = temp[0][0]
        
        C = 0
        ifC = 0
        for ki, k in enumerate(range(8)):
            for l in range(k+1):
                if ki%2 == 0:
                    C += temp[k-l][l].count("B")
                    ifC += temp[k-l][l].count("W")
                else:
                    C += temp[k-l][l].count("W")
                    ifC += temp[k-l][l].count("B")
        for ki, k in enumerate(range(7)):
            for l in range(k+1):
                if ki%2 == 0:
                    C += temp[l-k+7][7-l] .count("B")
                    ifC += temp[l-k+7][7-l] .count("W")
                else:
                    C += temp[l-k+7][7-l] .count("W")
                    ifC += temp[l-k+7][7-l] .count("B")
                
        mine = min(C, ifC)
        if mine < Min:
            Min = mine
print(Min)