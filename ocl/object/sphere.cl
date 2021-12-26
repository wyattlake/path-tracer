#pragma once

#include "object.cl"

inline bool sphere_hit(Ray ray, OBJECT_ARGS_DEF) {
    float a = dot(ray.direction, ray.direction);
    float b = 2 * dot(ray.direction, ray.origin);
    float c = dot(ray.origin, ray.origin) - 1.0f;

    float discriminant = b * b - 4 * a * c;
    if (discriminant < 0) {
        return false;
    }

    float t1 = ((-1 * b) - sqrt(discriminant)) / (2 * a);
    float t2 = ((-1 * b) + sqrt(discriminant)) / (2 * a);
    return (t1 > 0);
}