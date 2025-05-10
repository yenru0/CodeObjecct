noHear = set()
noSeer = set()

N, M = map(int, input().split())

for i in range(N):
    noHear.add(input())
for i in range(M):
    noSeer.add(input())

new = noHear & noSeer
print(len(new))
for name in sorted(new):
    print(name)
