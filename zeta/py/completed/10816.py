from collections import Counter

if __name__ == "__main__":
    N = int(input())
    S = list(map(int, input().split()))
    counter = Counter(S)
    M = int(input())
    I = list(map(int, input().split()))
    print(" ".join(str(counter[i]) for i in I))
