#pragma once

// Packed scene data represented by 3 buffers
#define SCENE_DATA_DEF                                                         \
    __global const uint32 *len_buffer,                                         \
        __global const uint8 *object_buffer_int,                               \
        __global const float *object_buffer_float

#define SCENE_DATA len_buffer, object_buffer_int, object_buffer_float