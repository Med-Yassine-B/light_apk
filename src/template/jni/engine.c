#include "engine.h"
#include "font.h"
#include <android/log.h>
#include <string.h>
#include <stdlib.h>

#define LOG_TAG "LIGHT_APK"
#define LOGI(...) ((void)__android_log_print(ANDROID_LOG_INFO, LOG_TAG, __VA_ARGS__))
#define LOGE(...) ((void)__android_log_print(ANDROID_LOG_ERROR, LOG_TAG, __VA_ARGS__))

/* ─── Vertex shader ───────────────────────────────────────── */
static const char* VS_SRC =
    "uniform mat4 uProjection;\n"
    "attribute vec4 aPosition;\n"
    "attribute vec2 aTexCoord;\n"
    "varying vec2 vTexCoord;\n"
    "void main() {\n"
    "  gl_Position = uProjection * aPosition;\n"
    "  vTexCoord = aTexCoord;\n"
    "}";

/* ─── Fragment shader ─────────────────────────────────────── */
static const char* FS_SRC =
    "precision mediump float;\n"
    "varying vec2 vTexCoord;\n"
    "uniform sampler2D uTexture;\n"
    "uniform vec4 uColor;\n"
    "void main() {\n"
    "  float a = texture2D(uTexture, vTexCoord).a;\n"
    "  gl_FragColor = vec4(uColor.rgb, uColor.a * a);\n"
    "}";

/* ─── Compile shader ──────────────────────────────────────── */
static GLuint compile_shader(GLenum type, const char* src) {
    GLuint sh = glCreateShader(type);
    glShaderSource(sh, 1, &src, NULL);
    glCompileShader(sh);
    GLint ok;
    glGetShaderiv(sh, GL_COMPILE_STATUS, &ok);
    if (!ok) {
        char buf[256];
        glGetShaderInfoLog(sh, sizeof(buf), NULL, buf);
        LOGE("shader compile error: %s", buf);
        glDeleteShader(sh);
        return 0;
    }
    return sh;
}

/* ─── Link program ────────────────────────────────────────── */
static GLuint link_program(GLuint vs, GLuint fs) {
    GLuint prog = glCreateProgram();
    glAttachShader(prog, vs);
    glAttachShader(prog, fs);
    glLinkProgram(prog);
    GLint ok;
    glGetProgramiv(prog, GL_LINK_STATUS, &ok);
    if (!ok) {
        char buf[256];
        glGetProgramInfoLog(prog, sizeof(buf), NULL, buf);
        LOGE("program link error: %s", buf);
        glDeleteProgram(prog);
        return 0;
    }
    return prog;
}

/* ─── Build orthographic projection matrix ────────────────── */
static void ortho(float* m, float l, float r, float b, float t) {
    memset(m, 0, 16 * sizeof(float));
    m[0]  = 2.0f / (r - l);
    m[5]  = 2.0f / (t - b);
    m[10] = -1.0f;
    m[12] = -(r + l) / (r - l);
    m[13] = -(t + b) / (t - b);
    m[15] = 1.0f;
}

/* ─── Create font texture from bitmap data ────────────────── */
static GLuint create_font_texture() {
    int fw = 128, fh = 64;
    unsigned char* tex = calloc(fw * fh, 1);
    if (!tex) return 0;
    for (int c = 0; c < 95; c++) {
        int row = c / 16;
        int col = c % 16;
        for (int y = 0; y < 8; y++) {
            unsigned char bits = FONT[c][y];
            for (int x = 0; x < 8; x++) {
                int px = col * 8 + x;
                int py = row * 8 + y;
                tex[py * fw + px] = (bits & (0x80 >> x)) ? 255 : 0;
            }
        }
    }
    GLuint id;
    glGenTextures(1, &id);
    glBindTexture(GL_TEXTURE_2D, id);
    glTexImage2D(GL_TEXTURE_2D, 0, GL_ALPHA, fw, fh, 0,
                 GL_ALPHA, GL_UNSIGNED_BYTE, tex);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
    free(tex);
    return id;
}

/* ═══════════════════════════════════════════════════════════ */
/*  Public API                                                 */
/* ═══════════════════════════════════════════════════════════ */

int engine_init_egl(struct engine* eng) {
    const EGLint attribs[] = {
        EGL_SURFACE_TYPE, EGL_WINDOW_BIT,
        EGL_RENDERABLE_TYPE, EGL_OPENGL_ES2_BIT,
        EGL_BLUE_SIZE, 8,
        EGL_GREEN_SIZE, 8,
        EGL_RED_SIZE, 8,
        EGL_NONE
    };
    const EGLint ctx_attribs[] = {
        EGL_CONTEXT_CLIENT_VERSION, 2,
        EGL_NONE
    };
    EGLint w, h, format, numConfigs;
    EGLConfig config;
    EGLBoolean ret;

    EGLDisplay display = eglGetDisplay(EGL_DEFAULT_DISPLAY);
    if (display == EGL_NO_DISPLAY) { LOGE("eglGetDisplay failed"); return -1; }
    ret = eglInitialize(display, 0, 0);
    if (ret != EGL_TRUE) { LOGE("eglInitialize failed"); return -1; }
    ret = eglChooseConfig(display, attribs, &config, 1, &numConfigs);
    if (ret != EGL_TRUE || numConfigs == 0) { LOGE("eglChooseConfig failed"); return -1; }
    ret = eglGetConfigAttrib(display, config, EGL_NATIVE_VISUAL_ID, &format);
    if (ret != EGL_TRUE) { LOGE("eglGetConfigAttrib failed"); return -1; }
    ANativeWindow_setBuffersGeometry(eng->app->window, 0, 0, format);

    EGLSurface surface = eglCreateWindowSurface(display, config, eng->app->window, NULL);
    if (surface == EGL_NO_SURFACE) { LOGE("eglCreateWindowSurface failed"); return -1; }
    eglBindAPI(EGL_OPENGL_ES_API);
    EGLContext context = eglCreateContext(display, config, NULL, ctx_attribs);
    if (context == EGL_NO_CONTEXT) {
        LOGE("eglCreateContext failed");
        eglDestroySurface(display, surface);
        return -1;
    }
    ret = eglMakeCurrent(display, surface, surface, context);
    if (ret != EGL_TRUE) {
        LOGE("eglMakeCurrent failed");
        eglDestroyContext(display, context);
        eglDestroySurface(display, surface);
        return -1;
    }
    eglQuerySurface(display, surface, EGL_WIDTH, &w);
    eglQuerySurface(display, surface, EGL_HEIGHT, &h);

    eng->display = display;
    eng->surface = surface;
    eng->context = context;
    eng->width   = w;
    eng->height  = h;

    GLuint vs = compile_shader(GL_VERTEX_SHADER, VS_SRC);
    GLuint fs = compile_shader(GL_FRAGMENT_SHADER, FS_SRC);
    if (!vs || !fs) return -1;
    eng->program = link_program(vs, fs);
    glDeleteShader(vs);
    glDeleteShader(fs);
    if (!eng->program) return -1;

    eng->aPosition   = glGetAttribLocation(eng->program, "aPosition");
    eng->aTexCoord   = glGetAttribLocation(eng->program, "aTexCoord");
    eng->uProjection = glGetUniformLocation(eng->program, "uProjection");
    eng->uTexture    = glGetUniformLocation(eng->program, "uTexture");
    eng->uColor      = glGetUniformLocation(eng->program, "uColor");

    eng->font_tex = create_font_texture();
    glGenBuffers(1, &eng->vbo);

    glEnable(GL_BLEND);
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glViewport(0, 0, w, h);

    LOGI("EGL init OK - %d x %d", w, h);
    return 0;
}

void engine_draw(struct engine* eng) {
    if (!eng->display) return;
    glClearColor(0.05f, 0.05f, 0.2f, 1.0f);
    glClear(GL_COLOR_BUFFER_BIT);

    /* "Hello Light Apk!" centered on screen */
    char line1[] = "Hello Light Apk!";
    int   len1   = strlen(line1);
    float scale1 = 8.0f;                      /* large enough to be visible */
    float tw1    = len1 * 8.0f * scale1;
    float th1    = 8.0f * scale1;
    float cx1    = (eng->width  - tw1) / 2.0f;
    float cy1    = (eng->height - th1) / 2.0f;   /* exact vertical center */
    engine_draw_text(eng, line1, cx1, cy1, 0.0f, 0.9f, 1.0f, 1.0f, scale1);

    /* "Built with APK Studio" below it */
    char line2[] = "Built with Light APK";
    int   len2   = strlen(line2);
    float scale2 = 6.0f;
    float tw2    = len2 * 8.0f * scale2;
    float th2    = 8.0f * scale2;
    float cx2    = (eng->width  - tw2) / 2.0f;
    float cy2    = cy1 + th1 + 20.0f;        /* 20px below first line */
    engine_draw_text(eng, line2, cx2, cy2, 0.8f, 0.8f, 0.8f, 1.0f, scale2);

    eglSwapBuffers(eng->display, eng->surface);
}

void engine_draw_text(struct engine* eng, const char* text,
                      float x, float y, float r, float g, float b, float a, float scale) {
    if (!text || !*text || !eng->program) return;

    int len = strlen(text);
    float quad_w = 8.0f * scale;
    float quad_h = 8.0f * scale;
    float atlas_w = 128.0f;
    float atlas_h = 64.0f;

    glUseProgram(eng->program);

    float proj[16];
    ortho(proj, 0, (float)eng->width, (float)eng->height, 0);
    glUniformMatrix4fv(eng->uProjection, 1, GL_FALSE, proj);
    glUniform1i(eng->uTexture, 0);
    glUniform4f(eng->uColor, r, g, b, a);

    glActiveTexture(GL_TEXTURE0);
    glBindTexture(GL_TEXTURE_2D, eng->font_tex);
    glBindBuffer(GL_ARRAY_BUFFER, eng->vbo);
    glEnableVertexAttribArray(eng->aPosition);
    glEnableVertexAttribArray(eng->aTexCoord);
    glVertexAttribPointer(eng->aPosition, 2, GL_FLOAT, GL_FALSE,
                          4 * sizeof(float), (void*)0);
    glVertexAttribPointer(eng->aTexCoord, 2, GL_FLOAT, GL_FALSE,
                          4 * sizeof(float), (void*)(2 * sizeof(float)));

    for (int i = 0; i < len; i++) {
        unsigned char c = (unsigned char)text[i];
        if (c < 32 || c > 126) c = 0;
        int idx = c - 32;
        if (idx < 0) idx = 0;

        float tx0 = (float)((idx % 16) * 8) / atlas_w;
        float ty0 = (float)((idx / 16) * 8) / atlas_h;
        float tx1 = tx0 + 8.0f / atlas_w;
        float ty1 = ty0 + 8.0f / atlas_h;

        float x0 = x + i * quad_w;
        float x1 = x0 + quad_w;
        float y0 = y;
        float y1 = y + quad_h;

        float verts[] = {
            x0, y0,  tx0, ty0,
            x1, y0,  tx1, ty0,
            x0, y1,  tx0, ty1,
            x0, y1,  tx0, ty1,
            x1, y0,  tx1, ty0,
            x1, y1,  tx1, ty1,
        };
        glBufferData(GL_ARRAY_BUFFER, sizeof(verts), verts, GL_DYNAMIC_DRAW);
        glDrawArrays(GL_TRIANGLES, 0, 6);
    }

    glDisableVertexAttribArray(eng->aPosition);
    glDisableVertexAttribArray(eng->aTexCoord);
    glBindBuffer(GL_ARRAY_BUFFER, 0);
    glUseProgram(0);
}

void engine_term_display(struct engine* eng) {
    if (!eng->display) return;
    eglMakeCurrent(eng->display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
    if (eng->vbo)      glDeleteBuffers(1, &eng->vbo);
    if (eng->font_tex) glDeleteTextures(1, &eng->font_tex);
    if (eng->program)  glDeleteProgram(eng->program);
    eglDestroyContext(eng->display, eng->context);
    eglDestroySurface(eng->display, eng->surface);
    eglTerminate(eng->display);
    eng->display = EGL_NO_DISPLAY;
    eng->surface = EGL_NO_SURFACE;
    eng->context = EGL_NO_CONTEXT;
}
