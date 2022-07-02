#pragma once

#include "object.cl"
#include "args.cl"

inline float* sphere_hit(float *hit_times, Ray ray, OBJECT_ARGS_DEF) {
    float4 sphere_to_ray =
        (float4)(ray.origin.x, ray.origin.y, ray.origin.z, 0.0);
    float a = dot(ray.direction, ray.direction);
    float b = 2 * dot(ray.direction, sphere_to_ray);
    float c = dot(sphere_to_ray, sphere_to_ray) - 1.0f;

    float discriminant = b * b - 4 * a * c;
    if (discriminant < 0) {
        return 0L;
    }

    hit_times[0] = (-b - sqrt(discriminant)) / (2 * a);
    hit_times[1] = (-b + sqrt(discriminant)) / (2 * a);

    if (hit_times[0] > 0) {
        return hit_times;
    } else {
        return 0L;
    }
}