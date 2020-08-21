I = input()
stack = []
priority = {"*": 3, "/": 3, "+": 2, "-": 2, "(": 1}
for i in I:
    if i in "*/+-()":
        if len(stack) == 0:
            stack.append(i)
            continue
        if i == ")":
            while stack[-1] != "(":
                print(stack.pop(), end='')
            stack.pop()
        elif i == "(":
            stack.append(i)
        else:
            while priority[stack[-1]] >= priority[i]:
                print(stack.pop(), end='')
                if len(stack) == 0:
                    break
            stack.append(i)

    else:
        print(i, end='')
else:
    for s in stack[::-1]:
        print(s, end='')
