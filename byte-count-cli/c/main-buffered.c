#include <stdio.h>
#include <stdlib.h>

int main(int argc, char * argv[]) {

    if (argc < 2) return -1;

    int counts[256] = {};
    
    {
        FILE * file = fopen(argv[1], "rb");

        if (file == NULL) return -1;

        const int buffer_size = 4096;
        unsigned char * buffer = (unsigned char *) malloc(buffer_size);
        
        int read_count;
        while ((read_count = fread(buffer, 1, buffer_size, file)) != 0)
        {
            for (int i = 0; i < read_count; i++) {
                unsigned char index = buffer[i];
                counts[index] += 1;
            }
        }

        fclose(file);
    }

    for (int i = 0; i < 256; i++) {
        printf("0x%02x: %d\n", i, counts[i]);
    }
}