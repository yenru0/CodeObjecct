if __name__ == "__main__":
    T = int(input())
    for i in range(T):
        N = int(input())
        d, r = divmod(N, 5)
        print(" ".join(["++++"] * d) + ((" " if d else "") + "|" * r if r else ""))
