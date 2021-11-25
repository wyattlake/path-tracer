__kernel void mean(int2 size, uint dst_passes, uint src_passes,
                   __global float *dst_buffer,
                   __global const float *src_buffer) {
  int2 pos = (int2)(get_global_id(0), get_global_id(1));
  int idx = pos.x + pos.y * size.x;

  float3 dst_color = vload3(idx, dst_buffer);
  float3 src_color = vload3(idx, src_buffer);
  dst_color = (dst_color * dst_passes + src_color * src_passes) /
              (dst_passes + src_passes);
  vstore3(dst_color, idx, dst_buffer);
}

__kernel void rgb(int2 size, __global float *mean_buffer,
                  __global uchar *rgb_buffer) {
  int2 pos = (int2)(get_global_id(0), get_global_id(1));
  int idx = pos.x + pos.y * size.x;

  float3 color = vload3(idx, mean_buffer);
  uchar3 pixel = convert_uchar3(255.0f * clamp(color, 0.0f, 1.0f));
  vstore3(pixel, idx, rgb_buffer);
}