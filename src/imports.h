#pragma once
#ifdef USE_SDL2
	#include<SDL2/SDL.h>
	#include<SDL2_gfxPrimitives.h>
	#include<SDL2_framerate.h>
	#include<SDL2_imageFilter.h>
	#include<SDL2_rotozoom.h>
#elif defined(USE_SDL3)
	#include<SDL3/SDL.h>
	#include<SDL3_gfxPrimitives.h>
	#include<SDL3_framerate.h>
	#include<SDL3_imageFilter.h>
	#include<SDL3_rotozoom.h>
	#include<SDL3_Image/SDL_image.h>
	#include<SDL3_ttf/SDL_ttf.h>

#endif

//#include"app.hpp"

#ifdef __cplusplus
extern"C"{
#endif
	void ui_init(SDL_Window* pWindow,SDL_Renderer* pRenderer);
	void ui_poll_events(SDL_Event* pEvent);
	void ui_quit();
	void ui_begin(const char* lpszCaption);
	void ui_end();
	void ui_text(const char* lpszCaption);
	void ui_checkbox(const char* lpszCaption,bool* pFlag);
	bool ui_button(const char* lpszCaption);
	void ui_sameline();
	void ui_newline();
	void ui_space();
	void ui_begin_frame();
	void ui_end_frame();
	void ui_render(SDL_Renderer* pRenderer);
	bool ui_begin_main_menubar();
	void ui_end_main_menubar();
	bool ui_begin_menu(const char* lpszCaption);
	void ui_end_menu();
	bool ui_menu_item(const char* lpszCaption);
	bool ui_begin_listbox(const char* lpszID,int w,int h);
	void ui_end_listbox();
	bool ui_selectable(const char* lpszCaption,bool isSelected);
	void ui_image(SDL_Texture* lpTexture,int w,int h);
	void ui_subimage(SDL_Texture* lpTexture,int w,int h,float l,float t,float r,float b);

#ifdef __cplusplus
}
#endif
