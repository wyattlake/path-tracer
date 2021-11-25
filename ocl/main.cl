__kernel void render(
    int2 size,
    __global float *color_buffer,
    __global uint *random_buffer,
    __global const int *object_buffer_int,
    __global const float *object_buffer_float
) {
    int2 pos = (int2) (get_global_id(0), get_global_id(1));
    printf("%d, %d", pos.x, pos.y);
}