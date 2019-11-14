while True:
    inp = tuple(map(int, input().split()))
    if inp[0] == inp[1] == 0:
        break
    else:
        if inp[0] >= inp[1]:
            s = "multiple" if inp[0] % inp[1] == 0 else "neither"
        else :
            s = "factor" if inp[1] % inp[0] == 0 else "neither"

    print(s)
