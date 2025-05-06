if __name__ == "__main__":
    N = int(input())
    I = [input() for _ in range(N)]
    I_a = sorted(I)
    I_d = I_a.copy()[::-1]
    if I == I_a:
        print("INCREASING")
    elif I == I_d:
        print("DECREASING")
    else:
        print("NEITHER")
