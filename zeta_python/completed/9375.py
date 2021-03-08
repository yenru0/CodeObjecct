def case(N, cloth):
    cloth_dict = {}
    for i in range(N):
        if cloth[i][1] in cloth_dict:
            cloth_dict[cloth[i][1]] += 1
        else:
            cloth_dict[cloth[i][1]] = 1
    ret = 1
    for i in cloth_dict.values():
        ret *= i + 1
    return ret - 1


def solve(T, I):
    return "\n".join([str(case(I[i][0], I[i][1])) for i in range(T)])


if __name__ == '__main__':
    T = int(input())
    I = []
    for _ in range(T):
        N = int(input())
        temp = []
        for _ in range(N):
            gs = input().split()
            temp.append(gs)
        I.append((N, temp))

    print(solve(T, I))
