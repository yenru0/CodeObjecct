#include <iostream>
#include <vector>
#include <numeric>
#include <algorithm>

using namespace std;

vector<int> isPerfect(int N) {
    vector<int> v;
    for (int i = 1; i < N; i++) {
        if (N % i == 0) {
            v.push_back(i);
        }
    }

    int result = accumulate(v.begin(), v.end(), 0);
    if (result == N) {
        return v;
    } else {
        return vector<int>();
    }

}

int main() {
    int input;
    vector<int> out;
    cin >> input;
    while (input != -1) {

        out = isPerfect(input);
        if (out.size() > 0) {
            cout << input << " = " << out[0];
            for (int i = 1; i < out.size(); i++) {
                cout << " + " << out[i];
            }
            cout << endl;
        } else {
            cout << input << " is NOT perfect." << endl;
        }
        cin >> input;
    }

    return 0;
}