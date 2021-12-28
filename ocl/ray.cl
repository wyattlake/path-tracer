#pragma once

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
