"""
2447: 별 찍기 - 10
문제:
    예제를 보고 규칙을 유추한 뒤에 별을 찍어 보세요.
입력:
    첫째 줄에 N이 주어진다. N은 항상 3의 제곱꼴인 수이다. (3, 9, 27, ...) (N=3^k, 1 ≤ k < 8)
출력:
    첫째 줄부터 N번째 줄까지 별을 출력한다.
"""
"""
TC1:
Input:
27
Output:
***************************
* ** ** ** ** ** ** ** ** *
***************************
***   ******   ******   ***
* *   * ** *   * ** *   * *
***   ******   ******   ***
***************************
* ** ** ** ** ** ** ** ** *
***************************
*********         *********
* ** ** *         * ** ** *
*********         *********
***   ***         ***   ***
* *   * *         * *   * *
***   ***         ***   ***
*********         *********
* ** ** *         * ** ** *
*********         *********
***************************
* ** ** ** ** ** ** ** ** *
***************************
***   ******   ******   ***
* *   * ** *   * ** *   * *
***   ******   ******   ***
***************************
* ** ** ** ** ** ** ** ** *
***************************
"""
s = []
def B(n):
    if n == 1:
        return "*"
    else :
        s.append([B(int(n/3))+B(int(n/3))+B(int(n/3))
        B(int(n/3))+" "*int(n/3)+B(int(n/3))
        B(int(n/3))+B(int(n/3)),B(int(n/3))
B(3)