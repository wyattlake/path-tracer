#pragma once

#define OBJECT_ARGS_DEF                                                        \
    uint int_len, uint float_len, __global const unsigned short *int_buffer,   \
        __global const float *float_buffer

#define OBJECT_ARGS int_len, float_len, int_buffer, float_buffer