#include<stdio.h>

int main() {
    int min=1e6; int max=-1e6;
    int N; int a;
    scanf("%d", &N);
    
    for (int i=0;i<N;i++) {
        scanf("%d", &a);
        if (a < min) {
            min = a;
        }
        if (a > max){
            max = a;
        }
    }
    printf("%d %d", min, max);
}