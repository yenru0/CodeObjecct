if __name__ == "__main__":
    Mem = (1, 0)
    N = int(input())
    for i in range(1, N):
        Mem = ((Mem[1] + Mem[0]) % 10, Mem[0] % 10)
    print(sum(Mem) % 10)
