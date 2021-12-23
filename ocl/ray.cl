#pragma once

// Basic Ray struct, contains an origin and direction
typedef struct {
    float3 origin;
    float3 direction;
} Ray;

Ray ray_new(float3 origin, float3 direction) {
    Ray ray = {
        .origin = origin,
        .direction = direction,
    };
    return ray;
}

Ray ray_default() { return ray_new((float3)(0.0f), (float3)(0.0f)); }
