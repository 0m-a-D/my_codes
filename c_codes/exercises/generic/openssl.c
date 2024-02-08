#include <stdio.h>
#include <openssl/md5.h>
#include<string.h>

int main() {
    char input[] = "Hello, World!";
    unsigned char output[MD5_DIGEST_LENGTH];

    MD5((unsigned char*)input, strlen(input), output);

    printf("MD5 Hash: ");
    for (int i = 0; i < MD5_DIGEST_LENGTH; i++) {
        printf("%02x", output[i]);
    }
    printf("\n");

    return 0;
}