#include <algorithm>
#include <cctype>
#include <iostream>
#include <string>
#include <vector>

#define let auto

typedef int i32;
typedef long long i64;
typedef size_t usize;

using namespace std;

let fastio() {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
}

let is_vowel(char c) -> bool {
    return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
}

let get_rhyme_part(const string &line) -> string {
    if (line.empty()) return "";

    let end = (i32) line.length() - 1;
    while (end >= 0 && isspace(line[end])) {
        end--;
    }
    if (end < 0) return "";

    let start = end;
    while (start >= 0 && !isspace(line[start])) {
        start--;
    }

    let word = line.substr(start + 1, end - start);

    for (let &c: word) {
        c = tolower(c);
    }

    let vowel_pos = -1;
    for (let i = (i32) word.length() - 1; i >= 0; --i) {
        if (is_vowel(word[i])) {
            vowel_pos = i;
            break;
        }
    }

    if (vowel_pos != -1) {
        return word.substr(vowel_pos);
    } else {
        return word;
    }
}

let main() -> i32 {
    fastio();

    let line = string();
    getline(cin, line);
    if (line.empty()) return 0;

    let tc = (usize) stoi(line);
    for (let i = (usize) 0; i < tc; i++) {
        let rhymes = vector<string>(4);
        for (let j = 0; j < 4; j++) {
            getline(cin, line);
            rhymes[j] = get_rhyme_part(line);
        }

        let p12 = (rhymes[0] == rhymes[1]);
        let p23 = (rhymes[1] == rhymes[2]);
        let p34 = (rhymes[2] == rhymes[3]);
        let p13 = (rhymes[0] == rhymes[2]);
        let p24 = (rhymes[1] == rhymes[3]);
        let p14 = (rhymes[0] == rhymes[3]);

        if (p12 && p23 && p34) {
            cout << "perfect" << endl;
        } else if (p12 && p34) {
            cout << "even" << endl;
        } else if (p13 && p24) {
            cout << "cross" << endl;
        } else if (p14 && p23) {
            cout << "shell" << endl;
        } else {
            cout << "free" << endl;
        }
    }
    return 0;
}