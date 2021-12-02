#include <stdio.h>

int main(int argc, char** argv)
{
        FILE *f;
        int depth;
        int ldepth = -1;
        int n = -1;

        f = fopen("input.txt", "r");
        if (!f) {
                return 1;
        }

        while (1 == fscanf(f, "%d", &depth)) {
                if (depth > ldepth) {
                        ++n;
                }
                ldepth = depth;
        }

        printf("%d\n", n);
        fclose(f);
        return 0;
}
