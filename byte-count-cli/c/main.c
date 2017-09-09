#include <stdio.h>

int main(int argc, char * argv[]) {

    if (argc < 2) return -1;

    int counts[256] = {};
    
    {
        FILE * file = fopen(argv[1], "rb");

        if (file == NULL) return -1;

        int byte;

        while ((byte = fgetc(file)) != EOF) {
            unsigned char index = byte;
            counts[index] += 1;
        }

        fclose(file);
    }

    for (int i = 0; i < 256; i++) {
        printf("0x%02x: %d\n", i, counts[i]);
    }
}