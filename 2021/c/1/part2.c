#include <stdio.h>
#define WINDOW_SIZE 3

int main(int argc, char** argv)
{
        FILE *f;
        int i;
        int window[WINDOW_SIZE];
        int depth;
        int hits;

        f = fopen("input.txt", "r");
        if (!f) {
                return 1;
        }

        hits = 0;
        for(i = 0; (1 == fscanf(f, "%d", &depth)) ; i++) {
                if (i >= WINDOW_SIZE) {
                        if (depth > window[i % WINDOW_SIZE]) {
                                ++hits;
                        }
                }
                window[i % WINDOW_SIZE] = depth;
        }

        printf("%d\n", hits);
        fclose(f);
        return 0;
}
