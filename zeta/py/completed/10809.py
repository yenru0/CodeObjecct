from sys import *
c=0
D=[]
I=[ord(i)-97 for i in stdin.readline().rstrip()]
for i in range(26):
    try:D.append(I.index(i))
    except:D.append(-1)
D=list(map(str,D))
print(" ".join(D))
