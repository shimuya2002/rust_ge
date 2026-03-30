#include"app.hpp"
#include<cassert>
#include <imgui.h>
#include <backends/imgui_impl_sdl2.h>
#include <backends/imgui_impl_sdlrenderer2.h>
#include <misc/cpp/imgui_stdlib.h>
CApp::CApp()
:m_p_window(nullptr),
 m_p_renderer(nullptr),
 m_on_paint(nullptr),
 m_on_quit(nullptr),
 m_p_ud(nullptr),
 m_on_init(nullptr)
{
    SDL_Init(SDL_INIT_EVERYTHING);
    IMG_Init(IMG_INIT_PNG | IMG_INIT_JPG /*| IMG_INIT_WEBP*/);
    TTF_Init();
}
CApp::~CApp(){
    TTF_Quit();
    IMG_Quit();
    SDL_Quit();
}
bool CApp::run_step(int w,int h){
    if(nullptr==m_p_window){
        m_p_window=SDL_CreateWindow(
            "",
            SDL_WINDOWPOS_CENTERED,
            SDL_WINDOWPOS_CENTERED,
            w,
            h,
            SDL_WINDOW_OPENGL);
        m_p_renderer=SDL_CreateRenderer(m_p_window,
                        -1,
                        SDL_RENDERER_ACCELERATED);
        IMGUI_CHECKVERSION();
        ImGui::CreateContext();
        ImGuiIO& io = ImGui::GetIO();
        io.ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard;
        ImGui_ImplSDL2_InitForSDLRenderer(m_p_window, 
            m_p_renderer);
        ImGui_ImplSDLRenderer2_Init(m_p_renderer);
        SDL_initFramerate (&m_fpsManager);
        if(nullptr!=m_on_init){
            m_on_init(m_p_ud);
        }
    }else{
        SDL_framerateDelay(&m_fpsManager);
        SDL_SetRenderDrawColor(this->m_p_renderer,0,0,0,0xFF);
        SDL_RenderClear(this->m_p_renderer);

        if(nullptr!=m_on_paint){
            m_on_paint(m_p_ud);
        }
        SDL_RenderFlush(this->m_p_renderer);
        SDL_RenderPresent(this->m_p_renderer);

        SDL_PollEvent(&m_event);
        ImGui_ImplSDL2_ProcessEvent(&m_event);
        switch(m_event.type){
            case SDL_QUIT:{
                if(nullptr!=m_on_quit){
                    m_on_quit(m_p_ud);
                }
                ImGui_ImplSDLRenderer2_Shutdown();
                ImGui_ImplSDL2_Shutdown();
                ImGui::DestroyContext();
                SDL_DestroyRenderer(m_p_renderer);
                SDL_DestroyWindow(m_p_window);
                m_p_renderer=nullptr;
                m_p_window=nullptr;
                return false;
            }break;
        }

    }
    return true;

}
void CApp::begin_gui_frame(){
    ImGui_ImplSDLRenderer2_NewFrame();
    ImGui_ImplSDL2_NewFrame();
    ImGui::NewFrame();
}
void CApp::end_gui_frame(){
    ImGui::EndFrame();
    ImGui::Render();
    ImGui_ImplSDLRenderer2_RenderDrawData(ImGui::GetDrawData(),
        m_p_renderer);

}
void* app_init(){
    return new CApp();
}
void app_quit(void* p){
    assert(nullptr!=p);
    CApp* pApp=(CApp*)p;
    delete pApp;
}
int run_step(void* p,int w,int h){
    assert(nullptr!=p);
    CApp* pApp=(CApp*)p;
    return pApp->run_step(w,h);
}
SDL_Renderer* get_sdl_renderer(void* p){
    assert(nullptr!=p);
    CApp* pApp=(CApp*)p;
    return pApp->get_sdl_renderer();    
}
void set_on_paint(void* p,OnPaint f){
    assert(nullptr!=p);
    CApp* pApp=(CApp*)p;
    pApp->set_on_paint(f);
}
void set_on_quit(void* p,OnQuit f){
    assert(nullptr!=p);
    CApp* pApp=(CApp*)p;
    pApp->set_on_quit(f);

}

void set_ud(void* p,void* pud){
    assert(nullptr!=p);
    CApp* pApp=(CApp*)p;
    pApp->set_ud(pud);

}
void set_on_init(void* p,OnInit f){
    assert(nullptr!=p);
    CApp* pApp=(CApp*)p;
    pApp->set_on_init(f);
}
