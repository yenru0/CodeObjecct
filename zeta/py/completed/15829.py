if __name__ == "__main__":
    N = int(input())
    M = 1234567891
    r = 31
    print(
        sum(
            [
                (a * r**i) % M
                for i, a in enumerate(map(lambda x: ord(x) - ord("a") + 1, input()))
            ]
        )
        % M
    )
