def solve(exp):
    t = exp.split("-")
    S = 0
    S += sum(map(int, t[0].split("+")))
    for i in range(1, len(t)):
        S -= sum(map(int, t[i].split("+")))
    return S


if __name__ == '__main__':
    print(solve(input()))
