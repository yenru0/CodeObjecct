import math

if __name__ == "__main__":
    N = int(input())
    S = list(map(int, input().split()))
    T, P = map(int, input().split())

    print(sum(math.ceil(s / T) for s in S))
    print(N // P, N % P)
