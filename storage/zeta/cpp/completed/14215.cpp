#include <iostream>
#include <algorithm>
#include <numeric>

using namespace std;

int main() {
    int sides[3];

    for (int i = 0; i < 3; i++) {
        cin >> sides[i];
    }

    sort(sides, sides + 3);

    if (sides[2] >= sides[1] + sides[0]) {
        sides[2] = sides[1] + sides[0] - 1;
    }

    cout << accumulate(sides, sides + 3, 0) << endl;
    return 0;
}