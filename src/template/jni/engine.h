#ifndef ENGINE_H_
#define ENGINE_H_

#include <android_native_app_glue.h>
#include <EGL/egl.h>
#include <GLES2/gl2.h>

struct engine {
    struct android_app* app;
    EGLDisplay display;
    EGLSurface surface;
    EGLContext context;
    int32_t width;
    int32_t height;

    /* Shader program */
    GLuint program;
    GLuint aPosition;
    GLuint aTexCoord;
    GLuint uProjection;
    GLuint uTexture;
    GLuint uColor;

    /* Font texture */
    GLuint font_tex;

    /* Vertex buffer for text quads */
    GLuint vbo;
};

/* Lifecycle */
int  engine_init_egl(struct engine* eng);
void engine_draw(struct engine* eng);
void engine_term_display(struct engine* eng);

/* Text rendering */
void engine_draw_text(struct engine* eng, const char* text,
                      float x, float y, float r, float g, float b, float a,
                      float scale);

#endif /* ENGINE_H_ */
