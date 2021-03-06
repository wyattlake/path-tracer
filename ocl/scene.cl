#pragma once

#include "linalg.cl"
#include "object/object.cl"
#include "ray.cl"
#include "object/material.cl"

// Packed scene data represented by 3 buffers
#define SCENE_DATA_DEF                                                         \
    const uint object_count, __global const uint *len_buffer,                  \
        __global const unsigned char *object_buffer_byte,                      \
        __global const float *object_buffer_float

#define SCENE_DATA                                                             \
    object_count, len_buffer, object_buffer_byte, object_buffer_float

// Calculates lighting for a single ray intersecting the scene using the Phong model
float3 intersect_scene(Ray ray, SCENE_DATA_DEF) {
    // Indices to keep track of location in data buffers
    uint float_idx = 0;
    uint byte_idx = 0;

    for (uint i = 0; i < object_count; i++) {
        // Calculates current position in data buffers
        const uint byte_len = *(len_buffer + i * 2 + 1);
        const uint float_len = *(len_buffer + i * 2);

        // Pointers to data buffers
        __global const unsigned char *byte_buffer = object_buffer_byte + byte_idx;
        __global const float *float_buffer = object_buffer_float + float_idx;

        // Gets the object ID
        uchar object_id = *byte_buffer;
        byte_buffer += 1;

        // Formats the object's matrices from data buffers
        matrix4 inverse = matrix4_load(&float_buffer);
        matrix4 transposed_inverse = matrix4_load(&float_buffer);

        // Loads the object's material from data buffers
        Material material = material_load(&float_buffer, &byte_buffer);
        if (material.invisible) {
            continue;
        }

        // Declares a float array for the ray's intersections with the object
        float hit_times[2];
        
        // Continue if the ray successfully hits the object
        if (object_hit(object_id, hit_times, ray_transform(ray, inverse), OBJECT_ARGS) != 0L) {
            // Calculates the intersection point in world space and object space
            float4 world_position = ray_pos(ray, hit_times[0]);
            float4 object_position = matrix4_dot(inverse, world_position);

            // Sets up variables for light calculations
            float4 light_position = (float4)(0.0, 2.0, 5.0, 1.0);
            float3 light_color = (float3)(1.0, 1.0, 1.0);

            // Calculates effective color and ambient
            float3 effective_color = material.color * light_color;
            float3 ambient = effective_color * material.ambient;

            // Finds vector from light to the collision in world space
            float4 light_vector = normalize(light_position - world_position);

            // Calculates the normal in object space then converts to world space
            float4 normal = object_normal(object_id, object_position, OBJECT_ARGS);
            float4 world_normal = matrix4_dot(transposed_inverse, normal);
            world_normal.w = 0.0f;
            world_normal = normalize(world_normal);

            float light_normal_dot = dot(light_vector, world_normal);
            if (light_normal_dot < 0.0) {
                return ambient;
            }

            // Calculates diffuse and reflective components of the light
            float3 diffuse = material.color * light_normal_dot * material.diffuse;
            float4 reflected_light = normalize(vector4_reflect(light_vector, world_normal));
            float reflect_view_dot = dot(reflected_light, ray.direction);

            if (reflect_view_dot < 0.0f) {
                return ambient + diffuse;
            }

            // Calculates specular highlights
            float3 specular = light_color * pown(reflect_view_dot, 200) * material.specular;

            return ambient + diffuse + specular;
        }

        float_idx += float_len;
        byte_idx += byte_len;
    }

    return (float3)(0.0f, 0.0f, 0.0f);
}