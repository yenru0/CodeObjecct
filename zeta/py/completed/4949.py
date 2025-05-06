I = input()
stack = []
while I != ".":
    stack = []
    for i in I:
        if i in "()[]":
            if i == ")" and stack:
                if not stack:
                    stack.append(0)
                    break
                elif stack[-1] == "(":
                    stack.pop()
                else:
                    break
            elif i == "]":
                if not stack:
                    stack.append(0)
                    break
                elif stack[-1] == "[":
                    stack.pop()
                else:
                    break
            else:
                stack.append(i)
    print("no" if stack else "yes")

    I = input()
