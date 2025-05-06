import sys

input = sys.stdin.readline

"""
A1 + A2 = S1
A2 + A3 = S2
...
AN + A1 = SN

S1 - S2 = A1 - A3
S3 - S4 = A3 - A5
..
S(N-2) - S(N-1) = A(N-2) - AN

S1 - S(N-1) = A1 - AN;


S1 = A1 + A2
S2 = A2 + A3
S3 = A3 + A1

S1 - S(N=3 - 1) + S(N=3) = 2 * A1
S2 - S3 + S1 = 2 * A2
S3 - S1 + S2 = 2 * A3
S1 - S2 + S3
S2 - S3 + S1
S3 - S1

S1 - S2 + S3 - S4 ... + SN
S2 - S3 ...
S3 - S4 + 

구하고자 하는 수 r에 대해서

2Ar = Sr - S(r+1) ... (원환 순환) / + S(r-1)

"""

if __name__ == "__main__":
    N = int(input())
    S = list(int(input()) for _ in range(N))
    transformer = lambda x: x % N
    n = (N - 1) // 2  # N = 2n + 1
    for i in range(N):
        print(
            (
                sum(
                    S[transformer(i + 2 * j)] - S[transformer(i + 2 * j + 1)]
                    for j in range(n)
                )
                + S[transformer(i - 1)]
            )
            // 2
        )
