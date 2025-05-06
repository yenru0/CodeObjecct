N = int(input())
c = 0
for i in range(N):
    s = input()
    beforehead = s[0]
    
    befores = set()
    befores.add(beforehead)
    for t in s:
        if t == beforehead:
            continue
        elif t in befores:
            break
        else:
            beforehead = t
            befores.add(t)
    else:
        c+=1
print(c)