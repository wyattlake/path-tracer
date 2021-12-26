#pragma once

#include "object.cl"
#include "sphere.cl"

bool object_hit(Ray ray, OBJECT_ARGS_DEF) {
    const unsigned short object_id = *int_buffer;
    switch (object_id) {
    case 0:
        return sphere_hit(ray, OBJECT_ARGS);
    default:
        printf("INVALID OBJECT ID %d\n", object_id);
        return false;
    }
}