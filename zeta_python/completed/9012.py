Glevel = 0
Nlevel = 0

testCase = int(input())
isVPS = True
for i in range(testCase):
    inString = input()
    if len(inString) % 2:
        print("NO")
        continue
    for i in inString:
        if i == "(":
            Glevel += 1
        elif i == ")":
            Glevel -= 1
        if Glevel < 0:
            isVPS = False
            break

    if isVPS and Glevel == 0:
        print("YES")
    else :
        print("NO")
        isVPS = True
    Glevel = 0