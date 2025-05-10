a,b,c = map(int, input().split())
print(int(a/(c-b)+1)) if c-b>0 else print(-1)
