__kernel void render_direct_lighting(
    int2 size, __global float *color_buffer, __global uint *random_buffer,
    __global const uint32 *len_buffer, __global const uint8 *object_buffer_int,
    __global const float *object_buffer_float) {
    int2 pos = (int2)(get_global_id(0), get_global_id(1));
    int idx = pos.x + pos.y * size.x;
    uint seed = random_buffer[idx];
    const float mult = idx / (float)(size.x * size.y);
    float3 color = (float3)(1.0 - mult, 0.5 - mult, mult);
    vstore3(color, idx, color_buffer);
}

__kernel void render_indirect_lighting(
    int2 size, __global float *color_buffer, __global uint *random_buffer,
    __global const uint32 *len_buffer, __global const uint8 *object_buffer_int,
    __global const float *object_buffer_float) {
    int2 pos = (int2)(get_global_id(0), get_global_id(1));
    int idx = pos.x + pos.y * size.x;
    uint seed = random_buffer[idx];
    const float mult = idx / (float)(size.x * size.y);
    float3 color = (float3)(1.0 - mult, 0.5 - mult, mult);
    vstore3(color, idx, color_buffer);
}