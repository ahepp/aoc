#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_LEN 255
#define MAX_LINES 1023
#define BITS 12

int main(int argc, char** argv)
{
        FILE *f;
        char oxy[MAX_LINES];
        char co2[MAX_LINES];
        char lines[MAX_LINES][MAX_LEN];
        int n;
        int i;
        int j;
        int ocidx;
        int ccidx;


        f = fopen("input.txt", "r");
        if (!f) {
                return 1;
        }

        for(n= 0; fgets(lines[n], MAX_LEN, f); n++);

        memset(oxy, 1, MAX_LINES * sizeof(char));
        memset(co2, 1, MAX_LINES * sizeof(char));
        for(i = 0; i < BITS; i++) {
                int ocan = 0;
                int osum = 0;
                int omed;
                int ccan = 0;
                int csum = 0;
                int cmed;

                printf("\nBIT %d\n", i);

                for(j = 0; j < n; j++) {
                        if (oxy[j]) {
                                osum += lines[j][i] - '0';
                                ocan++;
                        }
                        if (co2[j]) {
                                csum += lines[j][i] - '0';
                                ccan++;
                        }
                }

                if (osum * 2 + 1 > ocan) {
                        omed = '1';
                } else {
                        omed = '0';
                }
                if (csum * 2 + 1 > ccan) {
                        cmed = '1';
                } else {
                        cmed = '0';
                }

                printf("om: %c\tcm: %c\n", omed, cmed);

                ocan = 0;
                ccan = 0;
                for(j = 0; j < n; j++) {
                        oxy[j] &= (lines[j][i] == omed);
                        co2[j] &= (lines[j][i] != cmed);
                        if (oxy[j]) {
                                ocan++;
                                //printf("ocan: %s\n", lines[j]);
                                ocidx = j;
                        }
                        if (co2[j]) {
                                ccan++;
                                //printf("ccan: %s\n", lines[j]);
                                ccidx = j;
                        }
                }
                if(ocan == 1) {
                        printf("o: %s\n", lines[ocidx]);
                }
                if(ccan == 1) {
                        printf("c: %s\n", lines[ccidx]);
                }
        }

        printf("%d\n", (int) strtol(lines[ocidx], NULL, 2) * strtol(lines[ccidx], NULL, 2));

        fclose(f);
        return 0;
}
