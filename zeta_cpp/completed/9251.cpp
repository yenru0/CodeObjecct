#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

int lenLCSeq(string s1, string s2) {
    vector<vector<int>> DP(s1.length(), vector<int>(s2.length(), 0));

    for (int i = 0; i < s1.length(); i++) {
        for (int j = 0; j < s2.length(); j++) {
            if (s1[i] == s2[j]) {
                if (i == 0 && j == 0) {
                    DP[i][j] = 1;
                } else if (i == 0 && j != 0) {
                    DP[i][j] = 1;
                } else if (i != 0 && j == 0) {
                    DP[i][j] = 1;
                } else {
                    DP[i][j] = DP[i - 1][j - 1] + 1;
                }
            } else {
                if (i == 0 && j == 0) {

                } else if (i == 0 && j != 0) {
                    DP[i][j] = DP[i][j - 1];
                } else if (i != 0 && j == 0) {
                    DP[i][j] = DP[i - 1][j];
                } else {
                    DP[i][j] = max(DP[i - 1][j], DP[i][j - 1]);
                }
            }
        }
    }

    return DP[s1.length() - 1][s2.length() - 1];
}

int main(void) {
    string s1, s2;

    cin >> s1 >> s2;

    cout << lenLCSeq(s1, s2) << endl;

    return 0;
}