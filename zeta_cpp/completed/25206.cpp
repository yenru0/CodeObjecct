#include<iostream>

double convert_grade(const std::string &g);

int main() {
    double sum_of_score = 0;
    double sum_of_prod_score_grade = 0;
    for (int i = 0; i < 20; i++) {
        std::string category;
        double score;
        std::string grade;
        std::cin >> category >> score >> grade;
        double s_grade = convert_grade(grade);
        if (s_grade < 0) {
            continue;
        } else {
            sum_of_score += score;
            sum_of_prod_score_grade += score * s_grade;
        }


    }

    std::cout << sum_of_prod_score_grade / sum_of_score;
    return 0;
}

double convert_grade(const std::string &g) {
    if (g == "A+") {
        return 4.5;
    } else if (g == "A0") {
        return 4.0;
    } else if (g == "B+") {
        return 3.5;
    } else if (g == "B0") {
        return 3.0;
    } else if (g == "C+") {
        return 2.5;
    } else if (g == "C0") {
        return 2.0;
    } else if (g == "D+") {
        return 1.5;
    } else if (g == "D0") {
        return 1.0;
    } else if (g == "F") {
        return 0.0;
    } else {
        return -1.0;
    }


}