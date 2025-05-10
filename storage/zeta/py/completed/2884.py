h, m = map(int, input().split())

if m < 45:
    print((h-1)%24, 15+m)
else:
    print(h, m-45)