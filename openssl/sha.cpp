#include <iostream>
#include <cstdio>
#include <string.h>
#include <openssl/sha.h>

int main(){
    unsigned char ibuf[] = "ya_mudda";
    unsigned char obuf[SHA256_DIGEST_LENGTH];

    char obuf_str[SHA256_DIGEST_LENGTH * 2];
    SHA256(ibuf, strlen((const char*)ibuf), obuf);       
    int i;
    for (i = 0; i < SHA256_DIGEST_LENGTH; i++) {
        std::sprintf(obuf_str + 2 * i, "%02x", obuf[i]);
    }
    std::cout << obuf_str << std::endl;

    return 0;
}
