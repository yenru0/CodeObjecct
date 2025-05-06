if __name__ == "__main__":
    S = input()
    cnt = 0
    now = 0
    find = S.find("DKSH", now)
    while find != -1:
        cnt += 1
        now = find + 4
        find = S.find("DKSH", now)
    print(cnt)
