#include <android_native_app_glue.h>
#include <string.h>
#include "engine.h"

static void handle_cmd(struct android_app* app, int32_t cmd) {
    struct engine* eng = (struct engine*)app->userData;
    switch (cmd) {
        case APP_CMD_INIT_WINDOW:
            if (app->window) {
                if (engine_init_egl(eng) == 0)
                    engine_draw(eng);
            }
            break;
        case APP_CMD_TERM_WINDOW:
            engine_term_display(eng);
            break;
    }
}

void android_main(struct android_app* app) {
    struct engine eng;
    memset(&eng, 0, sizeof(eng));
    eng.app = app;
    app->userData = &eng;
    app->onAppCmd = handle_cmd;

    while (1) {
        int events;
        struct android_poll_source* source;
        while (ALooper_pollAll(0, NULL, &events, (void**)&source) >= 0) {
            if (source) source->process(app, source);
        }
        if (app->destroyRequested) break;
    }
}


