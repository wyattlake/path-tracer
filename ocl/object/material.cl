#pragma once

typedef struct {
    float3 color;
    float3 emission_color;
    float ambient;
    float diffuse;
    float specular;
    bool invisible;
} Material;

inline Material material_load(__global const float **float_buffer, __global const uchar **byte_buffer) {
    Material material = {
        .color = vload3(0, *float_buffer),
        .emission_color = vload3(1, *float_buffer),
        .ambient = *(*float_buffer + 6),
        .diffuse = *(*float_buffer + 7),
        .specular = *(*float_buffer + 8),
        .invisible = **byte_buffer,
    };

    *float_buffer += 9;
    *byte_buffer += 1;

    return material;
}