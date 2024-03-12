#include <iostream>
#include <vector>

using namespace std;

int main() {
    vector<string> S;
    string temp;
    for (int i = 0; i < 5; i++) {
        cin >> temp;
        S.push_back(temp);
    }


    bool is_fin = false;
    bool flag = false;
    int j = 0;
    while (!is_fin) {
        flag = false;
        for (int i = 0; i < 5; i++) {
            if (S[i].length() <= j) {
                is_fin = is_fin;
                continue;
            }
            flag = true;
            cout << S[i][j];

        }

        if (!flag) is_fin = true;
        j++;
    }

    return 0;
}