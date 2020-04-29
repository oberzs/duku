#include "objects.glsl"

layout(set = 2, binding = 0) uniform texture2D TEXTURES[100];
layout(set = 2, binding = 1) uniform sampler SAMPLER;

layout(location = 0) in vec3 POSITION;
layout(location = 1) in vec3 NORMAL;
layout(location = 2) in vec2 UV;
layout(location = 3) in vec3 LIGHT_SPACE_POS;

#define ALBEDO_TINT M_UNIFORMS.albedo_tint
#define ALBEDO_INDEX CONSTANTS.albedo_index
#define ALBEDO sampler2D(TEXTURES[ALBEDO_INDEX], SAMPLER)
#define LIGHTS W_UNIFORMS.lights
#define VIEW_POS W_UNIFORMS.view_pos.xyz
#define TIME W_UNIFORMS.time