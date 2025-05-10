def fraction_ize(p, q):
    while True:
        # one process
        for i in range(2, int(max((p, q)) ** (1 / 2)) + 1):
            if q % i == 0:
                if p % i == 0:
                    q //= i
                    p //= i
                    break
        else:
            break
    return p, q


def solve(N, M):
    chains = M[0]
    T = []
    for i in range(1, N):
        T.append(fraction_ize(chains, M[i]))
    return "\n".join(f"{p}/{q}" for p, q in T)


if __name__ == '__main__':
    print(solve(int(input()), list(map(int, input().split()))))
