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

#endif
#include<SDL_image.h>
#include<SDL_ttf.h>

//#include"app.hpp"
typedef void (*OnPaint)(void*);
typedef void (*OnQuit)(void*); 
typedef void (*OnInit)(void*);
#ifdef __cplusplus
extern"C"{
#endif
	void* app_init();
	void app_quit(void*);
	int run_step(void*,int,int);
	SDL_Renderer* get_sdl_renderer(void* p);
	void set_on_paint(void*,OnPaint);
	void set_on_quit(void*,OnQuit);
	void set_on_init(void*,OnInit);
	void set_ud(void*,void* p);
#ifdef __cplusplus
}
#endif
