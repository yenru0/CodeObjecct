import re
OUT = []
def turn_exp(start):
    t = start
    first = None
    second = None
    op = None
    if tokens[t].isalpha():
        first = tokens[t]
        t += 1
    elif tokens[t] == "(":
        rr = turn_exp(t+1)
        first = rr[1]
        t = rr[0]
        if tokens[t] == ")":
            t += 1
    if isinstance(first, str):
        OUT.append(first)
    if tokens[t] in "+-*/":
        op = tokens[t]
        t += 1

    if tokens[t].isalpha():
        second = tokens[t]
        t += 1
    elif tokens[t] == "(":
        rr = turn_exp(t+1)
        second = rr[1]
        t = rr[0]
        if tokens[t] == ")":
            t += 1
    if isinstance(second, str):
        OUT.append(second)
    OUT.append(op)
    return t, [first, second, op]

tokens = input()



#_, parsed = turn_exp(0)
#for o in OUT:
    #print(o, end = '')
# TODO: 중위 표기식의 연산순서 A+B+C 같은거