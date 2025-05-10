# 선영이가 이길 수 있는 솔루션은 하나 이상 Because 선영이가 지는 팀을 이길 수 있는 팀이 하나 이상 주어짐
# 아마도 'Top-Down' 으로 구현하는게 좋을듯 어짜피 마지막에선 선형이가 이길 수 있는 대상이 나와야 하니까

while True:
    T = []
    try:
        N = int(input())
        Seed = [list(map(int, input())) for _ in range(N)]
    except EOFError:
        break
    for i in range(N):
        if Seed[0][i]:
            T.append([[(0, i)], [0, i], [0, i]]) # share-plate, check-plate, used-plate
    while T:
        share, check, used = T.pop()
        if len(share) == N-1:
            print(share)
            break
        Temp = [[0]*N for _ in range(len(check))]
        for idx, chck in enumerate(check):
            for i in range(N):
                if chck == i:
                    continue
                if not Seed[chck][i] and i not in used:
                    Temp[idx][i] = 1
        # 난 못하겠소 다시는 못하겠소





    print(T)
    print('g')
