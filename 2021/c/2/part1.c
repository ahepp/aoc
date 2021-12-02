#include <stdio.h>
#define MAX_CMD_SIZE 255

int main(int argc, char** argv)
{
        FILE *f;
        char cmd[MAX_CMD_SIZE];
        int err;
        int mag;
        int x;
        int y;

        f = fopen("input.txt", "r");
        if (!f) {
                return 1;
        }

        err = 0;
        x = 0;
        y = 0;
        while (2 == fscanf(f, "%s%d", cmd, &mag)) {
                printf("%s %d\n", cmd, mag);
                switch(cmd[0]) {
                        case 'f':
                                x += mag;
                                break;
                        case 'u':
                                y -= mag;
                                break;
                        case 'd':
                                y += mag;
                                break;
                        default:
                                printf("unknown command\n");
                                err = 1;
                                goto cleanup;

                }
                printf("x: %d\ty: %d\n", x, y);
        }
        printf("%d\n", x * y);

cleanup:
        fclose(f);
        return err;
}
