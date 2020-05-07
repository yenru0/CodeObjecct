c = 2; n =int(input()); P = []
if n == 1:
    print(2)
else:
    k = 3
    while c <= n:
        for p in P:
            if k%p == 0:
                break
            if p**2 > k:
                P.append(k)
                c+=1
                break
        else:
            P.append(k)
            c+=1
        k += 2
    print(P[-1])
