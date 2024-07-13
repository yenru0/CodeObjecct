#include <stdio.h>

typedef struct Date {
    int year, month, day;
} Date;

Date *newDate(int year, int month, int day) {
    if (0 <= year && year <= 99) {
        if (1 <= month && month <= 12) {
            if (month == 1 || month == 3 || month == 5 || month == 7 || month == 8 || month == 10 || month == 12) {
                if (1 <= day && day <= 31) {
                    Date *date = (Date *) malloc(sizeof(Date));
                    date->year = year;
                    date->month = month;
                    date->day = day;
                    return date;
                }
            } else if (month == 2 && year % 4 == 0) {
                if (1 <= day && day <= 29) {
                    Date *date = (Date *) malloc(sizeof(Date));
                    date->year = year;
                    date->month = month;
                    date->day = day;
                    return date;
                }
            } else if (month == 2 && year % 4 != 0) {
                if (1 <= day && day <= 28) {
                    Date *date = (Date *) malloc(sizeof(Date));
                    date->year = year;
                    date->month = month;
                    date->day = day;
                    return date;
                }
            } else {
                if (1 <= day && day <= 30) {
                    Date *date = (Date *) malloc(sizeof(Date));
                    date->year = year;
                    date->month = month;
                    date->day = day;
                    return date;
                }
            }
        }
    }
    return NULL;
}

void delDate(Date *date) {
    free(date);
    date = NULL;
}

int valid(Date *x, Date now) {
    if (x == NULL) {
        return 1;
    }
    if (x->year < now.year) {
        return 0;
    } else if(x->year > now.year) {
        return 1;
    }

    if (x->month < now.month) {
        return 0;
    } else if (x->month > now.month) {
        return 1;
    } 

    if(x->day < now.day) {
        return 0;
    } else if(x->day >= now.day) {
        return 1;
    }

    return 0;
}

int main() {
    int N;
    Date NOW;
    int first, second, third;
    scanf("%d %d %d", &NOW.year, &NOW.month, &NOW.day);
    scanf("%d", &N);
    for (int i = 0; i < N; i++) {
        scanf("%d %d %d", &first, &second, &third);
        Date *d1 = newDate(first, second, third);
        Date *d2 = newDate(third, second, first);
        Date *d3 = newDate(third, first, second);
        if (d1 == NULL && d2 == NULL && d3 == NULL) {
            printf("invalid\n");
        } else if (valid(d1, NOW) && valid(d2, NOW) && valid(d3, NOW)) {
            printf("safe\n");
        } else {
            printf("unsafe\n");
        }
        delDate(d1);
        delDate(d2);
        delDate(d3);
    }
}