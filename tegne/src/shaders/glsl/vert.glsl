#include "objects.glsl"

layout(location = 0) in vec3 IN_POSITION;
layout(location = 1) in vec3 IN_NORMAL;
layout(location = 2) in vec2 IN_UV;

layout(location = 0) out vec3 OUT_POSITION;
layout(location = 1) out vec3 OUT_NORMAL;
layout(location = 2) out vec2 OUT_UV;
layout(location = 3) out vec3 OUT_LIGHT_SPACE_POS;

#define POSITION gl_Position
#define MODEL CONSTANTS.model
#define ALBEDO_INDEX CONSTANTS.albedo_index
#define VIEW W_UNIFORMS.view
#define PROJ W_UNIFORMS.proj
#define LIGHT_MATRIX W_UNIFORMS.light_matrix
#define TIME W_UNIFORMS.time