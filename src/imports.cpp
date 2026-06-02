#include"imports.h"
#include <imgui.h>
#ifdef USE_SDL2
#include <backends/imgui_impl_sdl2.h>
#include <backends/imgui_impl_sdlrenderer2.h>
#elif defined(USE_SDL3)
#include <backends/imgui_impl_sdl3.h>
#include <backends/imgui_impl_sdlrenderer3.h>
#endif
#include <misc/cpp/imgui_stdlib.h>

#include<format>
bool do_render_ui=false;
void ui_init(SDL_Window* pWindow,SDL_Renderer* pRenderer){
    do_render_ui=false;
    IMGUI_CHECKVERSION();
    ImGui::CreateContext();
    ImGuiIO& io = ImGui::GetIO();
    io.ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard;
#ifdef USE_SDL2
    ImGui_ImplSDL2_InitForSDLRenderer(pWindow, pRenderer);
    ImGui_ImplSDLRenderer2_Init(pRenderer);    
#elif defined(USE_SDL3)
    ImGui_ImplSDL3_InitForSDLRenderer(pWindow, pRenderer);
    ImGui_ImplSDLRenderer3_Init(pRenderer);    

#endif

}
void ui_poll_events(SDL_Event* pEvent){
#ifdef USE_SDL2    
    ImGui_ImplSDL2_ProcessEvent(pEvent);
#elif defined(USE_SDL3)
    ImGui_ImplSDL3_ProcessEvent(pEvent);
#endif

}
void ui_quit(){
#ifdef USE_SDL2    
    ImGui_ImplSDLRenderer2_Shutdown();
    ImGui_ImplSDL2_Shutdown();
#elif defined(USE_SDL3)
    ImGui_ImplSDLRenderer3_Shutdown();
    ImGui_ImplSDL3_Shutdown();

#endif
    ImGui::DestroyContext();
}
void ui_begin(const char* lpszCaption){
    
    ImGui::Begin(lpszCaption);//,NULL,ImGuiWindowFlags_Popup);
}
void ui_end(){
    ImGui::End();
}
void ui_space(){
    ImGui::Spacing();
}
void ui_text(const char* lpszCaption){
    ImGui::Text(lpszCaption);
}
void ui_checkbox(const char* lpszCaption,bool* pFlag){
    ImGui::Checkbox(lpszCaption, pFlag);
}
bool ui_button(const char* lpszCaption){
    return ImGui::Button(lpszCaption);

}
void ui_sameline(){
    ImGui::SameLine();
}
void ui_newline(){
    ImGui::NewLine();
}
void ui_begin_frame(){
#ifdef USE_SDL2
    ImGui_ImplSDLRenderer2_NewFrame();
    ImGui_ImplSDL2_NewFrame();
#elif defined(USE_SDL3)
    ImGui_ImplSDLRenderer3_NewFrame();
    ImGui_ImplSDL3_NewFrame();
#endif
    ImGui::NewFrame();
    do_render_ui=true;

}
void ui_end_frame(){
    ImGui::EndFrame();
}
void ui_render(SDL_Renderer* pRenderer){
    if(do_render_ui){
        ImGui::Render();
    #ifdef USE_SDL2
        ImGui_ImplSDLRenderer2_RenderDrawData(ImGui::GetDrawData(), pRenderer);
    #elif defined(USE_SDL3)
        ImGui_ImplSDLRenderer3_RenderDrawData(ImGui::GetDrawData(), pRenderer);
    #endif

    }

}
bool ui_begin_main_menubar(){
    return ImGui::BeginMainMenuBar();
}
void ui_end_main_menubar(){
    ImGui::EndMainMenuBar();
}
bool ui_begin_menu(const char* lpszCaption){
    if(ImGui::BeginMenu(lpszCaption)){
        ImGui::MenuItem(std::format("##Item_{}",rand()).c_str(),"",false,false);
        return true;
    }

    return false;


}
void ui_end_menu(){
    
    ImGui::EndMenu();
}
bool ui_menu_item(const char* lpszCaption){
    return ImGui::MenuItem(lpszCaption,"");
}
bool ui_begin_listbox(const char* lpszID,int w,int h){
    return ImGui::BeginListBox(lpszID,ImVec2(-1==w?0.0f:(float)w,-1==h?0.0f:(float)h));
}
void ui_end_listbox(){
    ImGui::EndListBox();
}
bool ui_selectable(const char* lpszCaption,bool isSelected){
    return ImGui::Selectable(lpszCaption,isSelected);
}
void ui_image(SDL_Texture* lpTexture,int w,int h){
    ImGui::Image(lpTexture,ImVec2((float)w,(float)h));
}
void ui_subimage(SDL_Texture* lpTexture,int w,int h,float l,float t,float r,float b){
    ImGui::Image(lpTexture,
        ImVec2((float)w,(float)h),
        ImVec2(l,t),
        ImVec2(r,b)
    );

}