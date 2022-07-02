#pragma once

#include "sphere.cl"
#include "args.cl"

float* object_hit(float *hit_times, Ray ray, OBJECT_ARGS_DEF) {
    const unsigned short object_id = *int_buffer;
    switch (object_id) {
    case 0:
        return sphere_hit(hit_times, ray, OBJECT_ARGS);
    default:
        printf("INVALID OBJECT ID %d\n", object_id);
        return 0L;
    }
}

float4 object_normal(float4 point, OBJECT_ARGS_DEF) {
    const unsigned short object_id = *int_buffer;
    switch (object_id) {
    case 0:
        point.w = 0.0;
        return point;
    default:
        printf("INVALID OBJECT ID %d\n", object_id);
        return 0L;
    }
}