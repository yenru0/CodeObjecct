if __name__ == "__main__":
    T = int(input())
    for _ in range(T):
        N1 = int(input())
        I1 = set(map(int, input().split()))
        N2 = int(input())
        I2 = list(map(int, input().split()))
        for i in I2:
            if i in I1:
                print(1)
            else:
                print(0)
