#include <iostream>

using namespace std;

int main() {
    int sides[3];

    for (int i = 0; i < 3; i++) {
        cin >> sides[i];
    }

    if (sides[0] + sides[1] + sides[2] != 180) {
        cout << "Error";
        return 0;
    }

    if (sides[0] == 60 and sides[1] == 60 and sides[2] == 60) {
        cout << "Equilateral";
        return 0;
    }
    if (sides[0] == sides[1] or sides[0] == sides[2] or sides[1] == sides[2]) {
        cout << "Isosceles";
        return 0;
    }

    cout << "Scalene";
    return 0;

}