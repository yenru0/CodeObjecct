A =[list(map(int, input().split())) for i in range(3)]

if A[0][0] == A[1][0]:
    print(A[2][0], end=" ")
elif A[1][0] == A[2][0]:
    print(A[0][0], end=" ")
else:
    print(A[1][0], end=" ")

if A[0][1] == A[1][1]:
    print(A[2][1], end="")
elif A[1][1] == A[2][1]:
    print(A[0][1], end="")
else:
    print(A[1][1], end="")