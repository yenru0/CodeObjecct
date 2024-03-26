#include <iostream>
#include <algorithm>

using namespace std;


int main() {
    int sides[3];
    while (true) {
        for (int i = 0; i < 3; i++) {
            cin >> sides[i];
        }

        if (sides[0] == 0) {
            break;
        }
        sort(sides, sides + 3);

        if (sides[0] + sides[1] <= sides[2]) {
            cout << "Invalid\n";
            continue;
        }

        if (sides[0] == sides[1] and sides[1] == sides[2] and sides[2] == sides[0]) {
            cout << "Equilateral\n";
            continue;
        }
        if (sides[0] == sides[1] or sides[0] == sides[2] or sides[1] == sides[2]) {
            cout << "Isosceles\n";
            continue;
        }

        cout << "Scalene\n";
    }
}
