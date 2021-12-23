#pragma once

#include "ray.cl"

#define CAMERA_ARGS_DEF float3 camera_pos, float16 camera_map, float fov
#define CAMERA_ARGS camera_pos, camera_map, fov

// Shifts and converts worker position to floats
float2 pos_to_screen(int2 pos, int2 size) {
    float2 shifted_pos = convert_float2(pos) - 0.5f * convert_float2(size);
    shifted_pos.y = -shifted_pos.y;
    return shifted_pos;
}

// Builds a ray from the camera and worker position
Ray camera_ray(int2 pos, int2 size, CAMERA_ARGS_DEF) {
    float2 screen_point = pos_to_screen(pos, size);
    float3 direction = normalize(screen_point.x * camera_map.s012 +
                                 screen_point.y * camera_map.s456 -
                                 1.0f / fov * camera_map.s89a);
    Ray ray = ray_new(camera_pos, direction);
    return ray;
}