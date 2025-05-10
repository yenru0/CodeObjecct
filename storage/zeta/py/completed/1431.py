N = int(input())


def c(v):
    return sum([int(i) for i in v if i.isdigit()])


for c in sorted([input() for i in range(N)], key=lambda v: (len(v), c(v), v)):print(c)
