#include <stdio.h>
#include <string.h>

#define MAX_LEN 255
#define BITS 12

int main(int argc, char** argv)
{
        FILE *f;
        char line[MAX_LEN];
        unsigned int sum[BITS];
        int lines;
        int i;
        int gamma;
        int epsilon;


        f = fopen("input.txt", "r");
        if (!f) {
                return 1;
        }

        memset(sum, 0, BITS * sizeof(int));
        for(lines = 0; fgets(line, MAX_LEN, f); ++lines) {
                for(i = 0; i < BITS; i++) {
                        sum[i] += line[i] - '0';
                }
        }

        gamma = 0;
        epsilon = 0;
        for(i = 0; i < BITS; i++) {
                int med;
                if (sum[i] * 2 > lines) {
                        med = 1;
                } else {
                        med = 0;
                }
                printf("%d, ", med);
                gamma += med << (BITS - 1 - i);
                epsilon += (~med & 1) << (BITS - 1 - i);
        }
        printf("\ng: %d\ne: %d\np: %d\n", gamma, epsilon, gamma * epsilon);

        fclose(f);
        return 0;
}
