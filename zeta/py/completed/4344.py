All = int(input())
case = []
v=0
for i in range(All):
    test = list(map(int,input().split()))
    case.append(test)
for i in range(All):
    test_all = sum(case[i])-case[i][0]
    Average = test_all/(len(case[i])-1)
    for j in range(len(case[i])-1):
        if Average < case[i][j+1]:
            v+=1
        else : pass
    print("%.3f%%"%(round(((100*v)/(len(case[i])-1)),3)))
    v=0