N = int(input())
points = [list(map(int, input().split())) for i in range(N)]
points += [points[0]]
N +=1
print("%.1f" % (abs(sum(points[i][0] * points[j][1] for i, j in zip(range(N - 1), range(1, N))) - sum(points[j][0] * points[i][1] for i, j in zip(range(N - 1), range(1, N))))/2))
