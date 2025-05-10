#include<iostream>
#include<map>
#include<cmath>
#include<vector>
#include<algorithm>

bool comp(std::pair<char, int> a, std::pair<char, int> b);

int solve(int N, std::string W[]) {

    std::map<char, int> V;

    for (int i = 0; i < N; ++i) {
        std::string w = W[i];
        for (int j = 0; j < w.length(); j++) {
            if (V.find(w[j]) != V.end()) {

                V[w[j]] += (int) pow(10, (double) (w.length() - j - 1));

            } else {
                V[w[j]] = (int) pow(10, (double) (w.length() - j - 1));
            }

        }
    }

    std::vector<std::pair<char, int>> v(V.begin(), V.end());

    sort(v.begin(), v.end(), comp);
    int s = 0;
    for (int i = 0; i < v.size(); i++) {
        s += (9 - i) * v[i].second;
    }


    return s;
}

bool comp(std::pair<char, int> a, std::pair<char, int> b) {
    return a.second > b.second;
}


int main() {
    int N;
    std::cin >> N;
    std::string W[N];
    for (int i = 0; i < N; i++) {
        std::cin >> W[i];
    }
    std::cout << solve(N, W);
    return 0;
}