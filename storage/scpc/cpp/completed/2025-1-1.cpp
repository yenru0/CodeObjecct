#include <iostream>
#include <vector>

using namespace std;

int max_customs_sold(const vector<int> &arr) {
    int num500 = 0;
    int num1000 = 0;
    int num5000 = 0;

    int cnt = 0;
    for (int e: arr) {
        if (e == 500) {
            num500++;
            cnt++;
        } else if (e == 1000) {
            if (num500 >= 1) {
                num1000++;
                num500--;
                cnt++;
            } else { break; }
        } else {
            if (num1000 >= 4 && num500 >= 1) {
                num5000++;
                num1000 -= 4;
                num500--;
                cnt++;
            } else if (num1000 >= 3 && num500 >= 3) {
                num5000++;
                num1000 -= 3;
                num500 -= 3;
                cnt++;
            } else if (num1000 >= 2 && num500 >= 5) {
                num5000++;
                num1000 -= 2;
                num500 -= 5;
                cnt++;
            } else if (num1000 >= 1 && num500 >= 7) {
                num5000++;
                num1000 -= 1;
                num500 -= 7;
                cnt++;
            } else if (num500 >= 9) {
                num5000++;
                num500 -= 9;
                cnt++;
            } else {
                break;
            }
        }
    }
    return cnt;
}


int main() {
    std::ios_base::sync_with_stdio(false);
    std::cin.tie(nullptr);

    int t;

    cin >> t;

    for (int test_case = 1; test_case <= t; test_case++) {
        int n;
        cin >> n;

        vector<int> v;

        for (int j = 0; j < n; j++) {
            int x;
            cin >> x;
            v.push_back(x);
        }

        cout << "Case #" << test_case << endl;
        cout << max_customs_sold(v) << endl;
    }
    return 0;
}