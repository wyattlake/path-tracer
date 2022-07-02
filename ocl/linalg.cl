#pragma once

typedef struct {
    float4 x, y, z, w;
} matrix4;

inline matrix4 matrix4_load(__global const float **float_buffer) {
    matrix4 matrix = {
        .x = vload4(0, *float_buffer),
        .y = vload4(1, *float_buffer),
        .z = vload4(2, *float_buffer),
        .w = vload4(3, *float_buffer),
    };

    *float_buffer += 16;

    return matrix;
}

float4 matrix4_dot(matrix4 m, float4 v) {
    return (float4)(dot(m.x, v), dot(m.y, v), dot(m.z, v), dot(m.w, v));
}

void matrix4_print(matrix4 m) {
    printf("%f, %f, %f, %f\n%f, %f, %f, %f\n%f, %f, %f, %f\n%f, %f, %f, %f\n",
           m.x.x, m.x.y, m.x.z, m.x.w,
           m.y.x, m.y.y, m.y.z, m.y.w,
           m.z.x, m.z.y, m.z.z, m.z.w,
           m.w.x, m.w.y, m.w.z, m.w.w);
}

void vector4_print(float4 v) {
    printf("%f, %f, %f, %f\n", v.x, v.y, v.z, v.w);
}

float4 vector4_reflect(float4 v, float4 normal) {
    return v - 2.0f * dot(normal, v) * normal;
}