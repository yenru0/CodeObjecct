N = int(input())
a = 0
while (N%5+5*a)%3 != 0 :
    a+=1

if N//5-a < 0:
    print(-1)
else:
    print((N%5+5*a)//3 + (N//5-a))