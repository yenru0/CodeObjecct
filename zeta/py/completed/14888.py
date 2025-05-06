N = int(input())
ns = list(map(int, input().split()))
ops = list(map(int, input().split()))

stack = []

traces = []

def calc(ind, a, b):
    if ind == 0:
        return a+b
    elif ind == 1:
        return a-b
    elif ind == 2:
        return a*b
    elif ind == 3:
        return int(a/b)
    

_ops = ops[:]
for i, v in enumerate(ops):
    if v == 0:
        pass
    else:
        _ops[i] = _ops[i] - 1
        stack.append((1, _ops, calc(i, ns[0], ns[1])))
        _ops = ops[:]

while stack:
    d, o, n = stack.pop()
    if not any(o):
        traces.append(n)
    else:
        _ops = o[:]
        for i, v in enumerate(o):
            if v == 0:
                pass
            else:
                _ops[i] = _ops[i] - 1
                stack.append((d+1, _ops, calc(i, n, ns[d+1])))
                _ops = o[:]

print(max(traces))
print(min(traces))