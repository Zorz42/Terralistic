#include <stdexcept>
#include "glfwAbstraction.hpp"
#include "graphics-internal.hpp"

const char* vertex_shader_code =
"#version 330 core\n"
"layout(location = 0) in vec2 vertex_position;"
"layout(location = 1) in vec4 vertex_color;"
"layout(location = 2) in vec2 vertex_uv;"
"out vec4 fragment_color;"
"out vec2 uv;"
"out vec2 back_uv;"
"uniform int has_color_buffer;"
"uniform vec4 default_color;"
"uniform mat3 transform_matrix;"
"uniform mat3 texture_transform_matrix;"
"void main() {"
"   gl_Position = vec4(transform_matrix * vec3(vertex_position.xy, 1), 1);"
"   fragment_color = mix(default_color, vertex_color, has_color_buffer);"
"   uv = (texture_transform_matrix * vec3(vertex_uv, 1)).xy;"
"   back_uv = (texture_transform_matrix * vec3(vertex_position, 1)).xy;"
"}";

const char* fragment_shader_code =
"#version 330 core\n"
"in vec4 fragment_color;"
"in vec2 uv;"
"in vec2 back_uv;"
"layout(location = 0) out vec4 color;"
"uniform sampler2D texture_sampler;"
"uniform sampler2D back_texture_sampler;"
"uniform int has_texture;"
"uniform int blend_multiply;"
"void main() {"
"   color = mix(vec4(1.f, 1.f, 1.f, 1.f), texture(texture_sampler, uv).rgba, has_texture) * fragment_color;"
"   color = mix(color, vec4(texture(back_texture_sampler, back_uv).rgb * color.rgb, 1), blend_multiply);"
"}";

void framebufferSizeCallback(GLFWwindow* window, int width, int height) {
    gfx::window_width = width / gfx::global_scale_x;
    gfx::window_height = height / gfx::global_scale_y;
    gfx::window_width_reciprocal = 1.f / gfx::window_width;
    gfx::window_height_reciprocal = 1.f / gfx::window_height;
    gfx::window_resized_counter++;
    
    gfx::window_normalization_transform = gfx::_Transformation();
    gfx::window_normalization_transform.stretch(gfx::window_width_reciprocal * 2, -gfx::window_height_reciprocal * 2);
    gfx::window_normalization_transform.translate(-float(gfx::window_width) / 2, -float(gfx::window_height) / 2);
    
    glBindTexture(GL_TEXTURE_2D, gfx::window_texture);
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, gfx::window_width, gfx::window_height, 0, GL_BGRA, GL_UNSIGNED_BYTE, nullptr);
    
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
    
    glBindTexture(GL_TEXTURE_2D, gfx::window_texture_back);
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, gfx::window_width, gfx::window_height, 0, GL_BGRA, GL_UNSIGNED_BYTE, nullptr);
    
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
    
    //if(gfx::curr_scene != nullptr)
    //    gfx::curr_scene->renderAll(); // TODO: implement
}

void windowContentScaleCallback(GLFWwindow* window, float scale_x, float scale_y) {
#ifndef __APPLE__
    scale_x = 1;
    scale_y = 1;
#endif

    if(gfx::global_scale == 0) {
        gfx::global_scale_x = scale_x;
        gfx::global_scale_y = scale_y;
    } else {
        gfx::global_scale_x = gfx::global_scale;
        gfx::global_scale_y = gfx::global_scale;
    }
    
    gfx::system_scale_x = scale_x;
    gfx::system_scale_y = scale_y;
    
    int window_width, window_height;
    glfwGetWindowSize(gfx::glfw_window, &window_width, &window_height);
    
    gfx::Scene* temp_scene = gfx::curr_scene;
    gfx::curr_scene = nullptr;
    framebufferSizeCallback(gfx::glfw_window, window_width * gfx::system_scale_x, window_height * gfx::system_scale_y);
#ifndef __APPLE__
    gfx::setMinimumWindowSize(gfx::window_width_min, gfx::window_height_min);
#endif
    gfx::curr_scene = temp_scene;
}

void gfx::initGlfw(int window_width_, int window_height_) {
    window_width = window_width_;
    window_height = window_height_;

    if(!glfwInit())
        throw std::runtime_error("Failed to initialize GLFW");

    glfwWindowHint(GLFW_SAMPLES, 0);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    glfw_window = glfwCreateWindow(window_width, window_height, "Test Window", nullptr, nullptr);
    glfwSetFramebufferSizeCallback(glfw_window, framebufferSizeCallback);
    glfwSetWindowContentScaleCallback(glfw_window, windowContentScaleCallback);

    float scale_x, scale_y;
    glfwGetWindowContentScale(glfw_window, &scale_x, &scale_y);

    if(!glfw_window)
        throw std::runtime_error("Failed to open GLFW window.");

    glfwMakeContextCurrent(glfw_window);
    
    if(!gladLoadGLLoader((GLADloadproc) glfwGetProcAddress))
        throw std::runtime_error("Failed to initialize OpenGL context");

    glfwSetInputMode(glfw_window, GLFW_STICKY_KEYS, GL_TRUE);

    shader_program = CompileShaders(vertex_shader_code, fragment_shader_code);
    uniform_has_texture = glGetUniformLocation(shader_program, "has_texture");
    uniform_default_color = glGetUniformLocation(shader_program, "default_color");
    uniform_texture_sampler = glGetUniformLocation(shader_program, "texture_sampler");
    uniform_has_color_buffer = glGetUniformLocation(shader_program, "has_color_buffer");
    uniform_transform_matrix = glGetUniformLocation(shader_program, "transform_matrix");
    uniform_texture_transform_matrix = glGetUniformLocation(shader_program, "texture_transform_matrix");
    uniform_back_texture_sampler = glGetUniformLocation(shader_program, "back_texture_sampler");
    uniform_blend_multiply = glGetUniformLocation(shader_program, "blend_multiply");
    
    glEnable(GL_BLEND);
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    
    glUseProgram(shader_program);
    
    glGenTextures(1, &window_texture);
    glGenTextures(1, &window_texture_back);
    glGenFramebuffers(1, &default_framebuffer);
    glBindFramebuffer(GL_FRAMEBUFFER, default_framebuffer);

    windowContentScaleCallback(glfw_window, scale_x, scale_y);

    GLfloat rect_outline_vertex_array[] = {
        0.f, 0.f,
        1.f, 0.f,
        1.f, 1.f,
        0.f, 1.f,
    };
    
    glGenBuffers(1, &rect_outline_vertex_buffer);
    glBindBuffer(GL_ARRAY_BUFFER, rect_outline_vertex_buffer);
    glBufferData(GL_ARRAY_BUFFER, sizeof(rect_outline_vertex_array), rect_outline_vertex_array, GL_STATIC_DRAW);
    
    GLfloat rect_vertex_array[] = {
        0.f, 0.f,
        1.f, 0.f,
        0.f, 1.f,
        1.f, 0.f,
        0.f, 1.f,
        1.f, 1.f,
    };
    
    glGenBuffers(1, &rect_vertex_buffer);
    glBindBuffer(GL_ARRAY_BUFFER, rect_vertex_buffer);
    glBufferData(GL_ARRAY_BUFFER, sizeof(rect_vertex_array), rect_vertex_array, GL_STATIC_DRAW);
    
    GLenum DrawBuffers[1] = {GL_COLOR_ATTACHMENT0};
    glDrawBuffers(1, DrawBuffers);
}

void gfx::setGlobalScale(float scale) {
    global_scale = scale;
    windowContentScaleCallback(gfx::glfw_window, gfx::system_scale_x, gfx::system_scale_y);
}

unsigned int gfx::CompileShaders(const char* vertex_code, const char* fragment_code) {
    GLuint vertex_id = glCreateShader(GL_VERTEX_SHADER);
    GLuint fragment_id = glCreateShader(GL_FRAGMENT_SHADER);

    glShaderSource(vertex_id, 1, &vertex_code, nullptr);
    glCompileShader(vertex_id);

    glShaderSource(fragment_id, 1, &fragment_code, nullptr);
    glCompileShader(fragment_id);

    GLuint program_id = glCreateProgram();

    glAttachShader(program_id, vertex_id);
    glAttachShader(program_id, fragment_id);
    glLinkProgram(program_id);

    glDetachShader(program_id, vertex_id);
    glDetachShader(program_id, fragment_id);

    glDeleteShader(vertex_id);
    glDeleteShader(fragment_id);

    return program_id;
}
