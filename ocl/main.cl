#include "camera.cl"
#include "object/object.cl"
#include "ray.cl"
#include "scene.cl"

__kernel void render_direct_lighting(int2 size, __global float *color_buffer,
                                     __global uint *random_buffer,
                                     SCENE_DATA_DEF, CAMERA_ARGS_DEF) {
    int2 pos = (int2)(get_global_id(0), get_global_id(1));
    int idx = pos.x + pos.y * size.x;
    uint seed = random_buffer[idx];

    Ray ray = camera_ray(pos, size, CAMERA_ARGS);
    float3 color = intersect_scene(ray, SCENE_DATA);

    vstore3(color, idx, color_buffer);
}

__kernel void render_indirect_lighting(int2 size, __global float *color_buffer,
                                       __global uint *random_buffer,
                                       SCENE_DATA_DEF, CAMERA_ARGS_DEF) {
    int2 pos = (int2)(get_global_id(0), get_global_id(1));
    int idx = pos.x + pos.y * size.x;
    uint seed = random_buffer[idx];
    const float mult = idx / (float)(size.x * size.y);
    float3 color = (float3)(1.0 - mult, 0.5 - mult, mult);
    vstore3(color, idx, color_buffer);
}