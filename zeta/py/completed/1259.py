t = input()
while t != "0":
    t_r = int(t[::-1])
    t_i = int(t)
    if t_r == t_i:
        print("yes")
    else:
        print("no")
    t = input()
