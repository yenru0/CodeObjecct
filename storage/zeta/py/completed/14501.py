N = int(input())
D = [tuple(map(int, input().split())) for i in range(N)]

T = []


def progress(day, cost):
    if day == N:
        T.append(cost)
        return
    elif day > N:
        return
    if D[day][0] == 1:
        progress(day + 1, cost + D[day][1])
    else:
        progress(day + D[day][0], cost + D[day][1])
        progress(day + 1, cost)


progress(0, 0)
print(max(T))
