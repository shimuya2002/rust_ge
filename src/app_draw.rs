impl App{
    pub fn set_gpage(&mut self,r_page:usize,s_page:usize){
        self.render_page=r_page;
        self.display_page=s_page;
        unsafe{
            SDL_SetRenderTarget(
 # [cfg(feature="non_bindings")]
                self.sdl_renderer,
 # [cfg(not(feature="non_bindings"))]
                get_sdl_renderer(self.p_app),
                self.g_pages[self.render_page]);
        }

    }
    pub fn set_anim_set(&mut self,idx:usize,anim_set:AnimSet){
        self.anim_sets[idx]=anim_set;
    }
    pub fn load_image(&mut self,x:i32,y:i32,path:&str){
        let file_path=path.to_string();
        let mut image=self.image_cache.get(&file_path);
        if let None=image{

            unsafe{
                if let Ok(t)=Texture::load(
 # [cfg(feature="non_bindings")]
                                self.sdl_renderer,
 # [cfg(not(feature="non_bindings"))]
                                get_sdl_renderer(self.p_app),
                                path){
                    self.image_cache.insert(&file_path,Rc::new(t));
                    image=self.image_cache.get(&file_path);
                }

            }
        }
        if let Some(mut t)=image{
            let im_ref:&Texture=Rc::borrow(&t);
            let dst_w=min(WND_W-x,im_ref.w);
            let dst_h=min(WND_H-y,im_ref.h);
            unsafe{
                let copy_rect=SDL_Rect{
                    x:x,
                    y:y,
                    w:if 0<dst_w{dst_w}else{0},
                    h:if 0<dst_h{dst_h}else{0}
                };
                SDL_RenderCopy(
 # [cfg(feature="non_bindings")]
                    self.sdl_renderer,
 # [cfg(not(feature="non_bindings"))]
                    get_sdl_renderer(self.p_app),
                    im_ref.tex,
                    null_mut(),
                    &copy_rect
                );
                let mut tmp=SDL_Rect{x:0,y:0,w:0,h:0};

                SDL_UnionRect(&self.dirty_rect_tbl[self.render_page],
                    &copy_rect,
                    &mut tmp);
                self.dirty_rect_tbl[self.render_page]=tmp;

            }
        }
        
    }
    pub fn update_screen(&mut self,w:i32,h:i32){
        unsafe{
            SDL_SetRenderTarget(
 # [cfg(feature="non_bindings")]
                self.sdl_renderer,
 # [cfg(not(feature="non_bindings"))]
                get_sdl_renderer(self.p_app),

                null_mut());
            SDL_SetRenderDrawColor(
 # [cfg(feature="non_bindings")]
                self.sdl_renderer,
 # [cfg(not(feature="non_bindings"))]
                get_sdl_renderer(self.p_app),
                0,0,0,0xFF);
            SDL_RenderClear(self.sdl_renderer);
            SDL_RenderCopy(
 # [cfg(feature="non_bindings")]
                self.sdl_renderer,
 # [cfg(not(feature="non_bindings"))]
                get_sdl_renderer(self.p_app),
                        
                self.g_pages[self.display_page],
                null_mut(),
                null_mut());
            SDL_RenderFlush(self.sdl_renderer);
            SDL_RenderPresent(self.sdl_renderer);

        }

    }
    pub fn button(&self,idx:usize)->bool{
        return self.button_state_buf[(self.button_buf_idx+1) & 1][idx];
    }
    pub fn click(&self)->bool{
        return self.mouse_button_state_buf[(self.button_buf_idx+1) & 1];
    }
    pub fn click_pos(&self)->&SDL_Point{
        return &self.click_pos;
    }
    pub fn wait(&self,ms:u32){
        unsafe{
            SDL_Delay(ms);
        }
    }
    pub fn copy(&mut self,idx:usize,src_rect:&SDL_Rect,dst_rect:&SDL_Rect){
        unsafe{
            SDL_RenderCopy(
 # [cfg(feature="non_bindings")]
                self.sdl_renderer,
 # [cfg(not(feature="non_bindings"))]
                get_sdl_renderer(self.p_app),
                
                self.g_pages[idx],src_rect,dst_rect);
            let mut tmp=SDL_Rect{x:0,y:0,w:0,h:0};

            SDL_UnionRect(&self.dirty_rect_tbl[idx],
                dst_rect,
                &mut tmp);
            self.dirty_rect_tbl[idx]=tmp;

        }
    }
    pub fn measure_msg_utf8(&self,txt:&str)->Size{
        if let Some(font)=&self.msg_font{
            return font.measure_utf8_size(txt);

        }
        return Size{w:0,h:0};
    }
    pub fn measure_ui_utf8(&self,txt:&str)->Size{
        if let Some(font)=&self.ui_font{
            return font.measure_utf8_size(txt);

        }
        return Size{w:0,h:0};

    }
    pub fn set_mod_color(&self,page:usize,r:u8,g:u8,b:u8){
        unsafe{
            SDL_SetTextureColorMod(self.g_pages[page],r,g,b);

        }
    }
    pub fn set_mod_alpha(&self,page:usize,alpha:u8){
        unsafe{
            SDL_SetTextureAlphaMod(self.g_pages[page],alpha);
        }
    }
    pub fn set_draw_color(&self,r:u8,g:u8,b:u8,a:u8){
        unsafe{
            SDL_SetRenderDrawColor(self.sdl_renderer,r,g,b,a);

        }
    }
    pub fn draw_rect(&mut self,rect:&SDL_Rect){
        unsafe{
            SDL_RenderDrawRect(
 # [cfg(feature="non_bindings")]
                self.sdl_renderer,
 # [cfg(not(feature="non_bindings"))]
                get_sdl_renderer(self.p_app),
                
                rect);
            let mut tmp=SDL_Rect{x:0,y:0,w:0,h:0};
            SDL_UnionRect(&self.dirty_rect_tbl[self.render_page],
                rect,
                &mut tmp);
            self.dirty_rect_tbl[self.render_page]=tmp;

        }
    }
    pub fn fill_rect(&mut self,rect:&SDL_Rect){
        unsafe{
 # [cfg(feature="non_bindings")]
            let renderer=self.sdl_renderer;
 # [cfg(not(feature="non_bindings"))]
            let renderer=get_sdl_renderer(self.p_app),

 # [cfg(feature="use_sdl3")]
            let dst_rect=&SDL_FRect{x:rect.x as f32,
                y:rect.y as f32,w:rect.w as f32,h:rect.h as f32};
 # [cfg(feature="use_sdl2")]
            let dst_rect=&rect;
            SDL_RenderFillRect(renderer,dst_rect);
            let mut tmp=SDL_Rect{x:0,y:0,w:0,h:0};

            SDL_UnionRect(&self.dirty_rect_tbl[self.render_page],
                rect,
                &mut tmp);
            self.dirty_rect_tbl[self.render_page]=tmp;

        }
    }
    pub fn draw_circle(&mut self,x:i32,y:i32,rad:i32){
        unsafe{
            let mut r:u8=0;
            let mut g:u8=0;
            let mut b:u8=0;
            let mut a:u8=0;
 # [cfg(feature="non_bindings")]
            let renderer=self.sdl_renderer;
 # [cfg(not(feature="non_bindings"))]
            let renderer=get_sdl_renderer(self.p_app),
            SDL_GetRenderDrawColor(renderer,&mut r,&mut g,&mut b,&mut a);

 # [cfg(feature="use_sdl3")]
            circleRGBA(renderer,x as f32,y as f32,rad as f32,r,g,b,a);
 # [cfg(feature="use_sdl2")]
            circleRGBA(renderer,x as i16,y as i16,rad as i16,r,g,b,a);
            let mut tmp=SDL_Rect{x:0,y:0,w:0,h:0};
            SDL_UnionRect(&self.dirty_rect_tbl[self.render_page],
                &SDL_Rect{x:x-rad,y:y-rad,w:rad*2,h:rad*2},
                &mut tmp);
            self.dirty_rect_tbl[self.render_page]=tmp;

        }
    }
    pub fn fill_circle(&mut self,x:i32,y:i32,rad:i32){
        unsafe{
            let mut r:u8=0;
            let mut g:u8=0;
            let mut b:u8=0;
            let mut a:u8=0;
 # [cfg(feature="non_bindings")]
            let renderer=self.sdl_renderer;
 # [cfg(not(feature="non_bindings"))]
            let renderer=get_sdl_renderer(self.p_app),
            SDL_GetRenderDrawColor(renderer,&mut r,&mut g,&mut b,&mut a);

 # [cfg(feature="use_sdl3")]
            filledCircleRGBA(renderer,x as f32,y as f32,rad as f32,r,g,b,a);
 # [cfg(feature="use_sdl2")]
            filledCircleRGBA(renderer,x as i16,y as i16,rad as i16,r,g,b,a);

                let mut tmp=SDL_Rect{x:0,y:0,w:0,h:0};
            SDL_UnionRect(&self.dirty_rect_tbl[self.render_page],
                &SDL_Rect{x:x-rad,y:y-rad,w:rad*2,h:rad*2},
                &mut tmp);
            self.dirty_rect_tbl[self.render_page]=tmp;

        }
    }
    pub fn draw_msg(&mut self,x:i32,y:i32,txt:&str){
        self.draw_text(USE_MSG_FONT,x,y,txt);
    }
    pub fn draw_ui_text(&mut self,x:i32,y:i32,txt:&str){
        self.draw_text(USE_UI_FONT,x,y,txt);
        
    }
    fn draw_text(&mut self,font_type:i32,x:i32,y:i32,txt:&str){
        if 0 ==txt.len(){
            return;
        }
        unsafe{
            let use_font=if USE_MSG_FONT==font_type{
                    &self.msg_font
                }else{
                    &self.ui_font
                };
            if let Some(font)=use_font{
                let surf=font.render_utf8(txt,(WND_W-x) as u32);
                let src_rect=SDL_Rect{x:0,y:0,w:(*surf).w,h:(*surf).h};
                let dst_rect=SDL_Rect{x:x,y:y,w:(*surf).w,h:(*surf).h};
 # [cfg(feature="non_bindings")]
            let renderer=self.sdl_renderer;
 # [cfg(not(feature="non_bindings"))]
            let renderer=get_sdl_renderer(self.p_app),
                let t_r=Texture::from_surface(renderer,surf);
                SDL_FreeSurface(surf);                

                if let Ok(tex)=t_r{
                        SDL_RenderCopy(renderer,tex.tex,&src_rect,&dst_rect);
                        let mut tmp=SDL_Rect{x:0,y:0,w:0,h:0};
                        SDL_UnionRect(&self.dirty_rect_tbl[self.render_page],
                            &dst_rect,
                            &mut tmp);
                        self.dirty_rect_tbl[self.render_page]=tmp;

                }
            }
        }
    }
    ///* 各GPAGEの更新領域の情報をクリアする
    pub fn clear_dirty_rects(&mut self){
        for i in &mut self.dirty_rect_tbl{
            *i=SDL_Rect{x:0,y:0,w:0,h:0};
        }
    }    
}