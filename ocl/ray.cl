#pragma once

#include "linalg.cl"

// Basic Ray struct, contains an origin and direction
typedef struct {
    float4 origin;
    float4 direction;
} Ray;

Ray ray_new(float4 origin, float4 direction) {
    Ray ray = {
        .origin = origin,
        .direction = direction,
    };
    return ray;
}

Ray ray_default() { return ray_new((float4)(0.0f), (float4)(0.0f)); }

Ray ray_transform(Ray ray, matrix4 matrix) {
    Ray transformed_ray;
    transformed_ray.origin = matrix4_dot(matrix, ray.origin);
    transformed_ray.direction = matrix4_dot(matrix, ray.direction);
    return transformed_ray;
}

float4 ray_pos(Ray ray, float t) {
    return ray.origin + ray.direction * t;
}