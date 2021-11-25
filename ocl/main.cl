__kernel void render(int2 size, __global float *color_buffer,
                     __global uint *random_buffer,
                     __global const int *object_buffer_int,
                     __global const float *object_buffer_float) {
  int2 pos = (int2)(get_global_id(0), get_global_id(1));
  int idx = pos.x + pos.y * size.x;
  uint seed = random_buffer[idx];
  const float mult = idx / 100.0;
  float3 color = (float3)(mult, 1.0 - mult, 1.0 - mult);
  vstore3(color, idx, color_buffer);
}