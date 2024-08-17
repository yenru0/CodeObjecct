# * *    * * *
# * * or * * *
# * *    * * *
if __name__ == "__main__":
    N = int(input())

    if N != 1:
        sqn = 0
        while sqn**2 < N:
            sqn += 1

        if (sqn - 1) * sqn > N:
            a = (sqn - 2 + sqn - 1) * 2
        else:
            a = (sqn - 1) * 4
    else:
        a = 4

    print(a)
