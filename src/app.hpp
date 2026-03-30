#pragma once
#include"imports.h"

class CApp{

    SDL_Window* m_p_window;
    SDL_Renderer* m_p_renderer;
    SDL_Event m_event;
    FPSmanager m_fpsManager;
    OnPaint m_on_paint;
    OnQuit m_on_quit;
    OnInit m_on_init;
    void* m_p_ud;
public:
    CApp();
    ~CApp();

    bool run_step(int w,int h);
    void begin_gui_frame();
    void end_gui_frame();

    SDL_Renderer* get_sdl_renderer(){
        return m_p_renderer;
    }
	void set_on_paint(OnPaint f){
        m_on_paint=f;
    }
	void set_on_quit(OnQuit f){
        m_on_quit=f;
    }
    void set_ud(void* p){
        m_p_ud=p;
    }
	void set_on_init(OnInit f){
        m_on_init=f;
    }
};