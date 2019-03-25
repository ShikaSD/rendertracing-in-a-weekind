#pragma version(1)
#pragma rs java_package_name(com.shika.rayrender)

const float3 lower_left_corner = {-2, -1, -1};
const float3 horizontal = {4, 0, 0};
const float3 vertical = {0, 2, 0};
const float3 origin = {0, 0, 0};

typedef struct {
    float3 origin;
    float3 direction;
} ray;

static float3 ray_at(const ray *r, float t) {
    return r->origin + r->direction * t;
}

static float hit_sphere(float3 center, float radius, const ray *r) {
    float3 d_oc = r->origin - center;
    float a = dot(r->direction, r->direction);
    float b = 2 * dot(d_oc, r->direction);
    float c = dot(d_oc, d_oc) - radius * radius;
    float d = b * b - 4 * a * c;
    if (d < 0) {
        return -1;
    } else {
        return (-b - sqrt(d)) / (2 * a);
    }
}

static float3 color(const ray *r) {
    float3 center = { 0, 0, -1 };
    float dist = hit_sphere(center, 0.5, r);
    if (dist > 0) {
        float3 normal = normalize(ray_at(r, dist)) - center;
        return 0.5 * (float3) { normal.x + 1, normal.y + 1, normal.z + 1 };
    }

    float3 norm_direction = normalize(r->direction);
    float t = 0.5 * (norm_direction.y + 1);
    return (1 - t) * (float3) { 1, 1, 1 } + t * (float3) { 0.5, 0.7, 1.0 };
}

uchar4 RS_KERNEL raytrace(uchar4 in, rs_kernel_context context, int32_t x, int32_t y) {
    float4 input = rsUnpackColor8888(in);
    int32_t width = rsGetDimX(context);
    int32_t height = rsGetDimY(context);

    float u = ((float) x) / width;
    float v = 1 - ((float) y) / height;

    float3 direction = lower_left_corner + u * horizontal + v * vertical;
    ray r = { origin, direction };
    float3 col = color(&r);

    input.r = col.r;
    input.g = col.g;
    input.b = col.b;
    input.a = 1;

    return rsPackColorTo8888(input);
}
