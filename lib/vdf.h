#ifndef VDF_H
#define VDF_H

#include <stddef.h>
#include <stdint.h>

typedef struct Buffer
{
    unsigned char *data;
    size_t len;
} Buffer;

typedef struct WesolowskiVDF
{
    uint16_t init_num_bits;
} WesolowskiVDF;

WesolowskiVDF *create_wesolowski_vdf(uint16_t num_bits);
void free_wesolowski_vdf(WesolowskiVDF *vdf);
void free_buffer(Buffer buf);

int vdf_compute(WesolowskiVDF *vdf, Buffer chellenge, uint64_t difficulty, Buffer *output_solution);
int vdf_verify(WesolowskiVDF *vdf, Buffer chellenge, uint64_t difficulty, Buffer solution);

#endif