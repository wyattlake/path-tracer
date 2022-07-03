#pragma once

#define OBJECT_ARGS_DEF                                                        \
    uint byte_len, uint float_len, __global const unsigned char *byte_buffer, \
        __global const float *float_buffer

#define OBJECT_ARGS byte_len, float_len, byte_buffer, float_buffer