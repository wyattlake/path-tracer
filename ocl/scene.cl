#pragma once

#include "object/hit.cl"

// Packed scene data represented by 3 buffers
#define SCENE_DATA_DEF                                                         \
    const uint object_count, __global const uint *len_buffer,                  \
        __global const unsigned short *object_buffer_int,                      \
        __global const float *object_buffer_float

#define SCENE_DATA                                                             \
    object_count, len_buffer, object_buffer_int, object_buffer_float

float3 intersect_scene(Ray ray, SCENE_DATA_DEF) {
    uint float_idx = 0;
    uint int_idx = 0;

    for (uint i = 0; i < object_count; i++) {
        const uint int_len = *(len_buffer + i * 2 + 1);
        const uint float_len = *(len_buffer + i * 2);

        __global const unsigned short *int_buffer = object_buffer_int + int_idx;
        __global const float *float_buffer = object_buffer_float + float_idx;

        if (object_hit(ray, OBJECT_ARGS)) {
            return (float3)(1.0, 0.0, 0.0);
        }

        float_idx += float_len;
        int_idx += int_len;
    }

    return (float3)(0.0f, 0.0f, 0.0f);
}