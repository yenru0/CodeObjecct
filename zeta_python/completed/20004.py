if __name__ == "__main__":
    A = int(input())
    for i in range(1, A + 1):
        print(i) if 30 % (i + 1) == 0 else 0
