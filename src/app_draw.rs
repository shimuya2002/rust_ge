#[macro_use]
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
 # [cfg(feature="non_bindings")]
            let renderer=self.sdl_renderer;
 # [cfg(not(feature="non_bindings"))]
            let renderer=get_sdl_renderer(self.p_app);

                if let Ok(t)=Texture::load(renderer,path){
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
 # [cfg(feature="use_sdl3")]
                let copy_rect=SDL_Rect{
                    x:x,
                    y:y,
                    w:if 0<dst_w{dst_w}else{0},
                    h:if 0<dst_h{dst_h}else{0}
                };
                let copy_rect=rect_type!(
                    x,
                    y,
                    if 0<dst_w{dst_w}else{0},
                    if 0<dst_h{dst_h}else{0}
                );
 # [cfg(feature="use_sdl3")]
                self.copy_tex_sdl3(im_ref,None,&copy_rect);
 # [cfg(feature="use_sdl2")]
                self.copy_tex_sdl2(im_ref,None,&copy_rect);
                let mut tmp=ZeroRect;

                rect_get_union(&self.dirty_rect_tbl[self.render_page],
                    &copy_rect,
                    &mut tmp);
                self.dirty_rect_tbl[self.render_page]=tmp;

            }
        }
        
    }
    pub fn update_screen(&mut self,w:i32,h:i32){
        unsafe{
 # [cfg(feature="non_bindings")]
            let renderer=self.sdl_renderer;
 # [cfg(not(feature="non_bindings"))]
            let renderer=get_sdl_renderer(self.p_app);

            SDL_SetRenderTarget(renderer,null_mut());
            SDL_SetRenderDrawColor(renderer,0,0,0,0xFF);
            SDL_RenderClear(renderer);
 # [cfg(feature="use_sdl3")]
            SDL_RenderTexture(renderer,
                self.g_pages[self.display_page],
                null_mut(),
                null_mut());
 # [cfg(feature="use_sdl2")]
            SDL_RenderCopy(renderer,
                self.g_pages[self.display_page],
                null_mut(),
                null_mut());

 # [cfg(feature="use_sdl3")]
            SDL_FlushRenderer(self.sdl_renderer);
 # [cfg(feature="use_sdl2")]
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
    pub fn copy(&mut self,idx:usize,src_rect:&RectType,dst_rect:&RectType){
        unsafe{

# [cfg(feature="use_sdl3")]
            self.copy_sdl3(idx,src_rect,dst_rect);
# [cfg(feature="use_sdl2")]
            self.copy_sdl2(idx,src_rect,dst_rect);



            let mut tmp=ZeroRect;

            rect_get_union(&self.dirty_rect_tbl[idx],
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
    pub fn draw_rect(&mut self,rect:&RectType){
        unsafe{
 # [cfg(feature="use_sdl3")]
            self.draw_rect_sdl3(rect);            
 # [cfg(feature="use_sdl2")]
            self.draw_rect_sdl2(rect);

            let mut tmp=gen_rect_i32(0,0,0,0);
            rect_get_union(&self.dirty_rect_tbl[self.render_page],
                rect,
                &mut tmp);
            self.dirty_rect_tbl[self.render_page]=tmp;

        }
    }
    pub fn fill_rect(&mut self,rect:&RectType){
        unsafe{
 # [cfg(feature="use_sdl3")]
            self.fill_rect_sdl3(rect);
 # [cfg(feature="use_sdl2")]
            self.fill_rect_sdl2(rect);
            let mut tmp=gen_rect_i32(0,0,0,0);

            rect_get_union(&self.dirty_rect_tbl[self.render_page],
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
            let renderer=get_sdl_renderer(self.p_app);
            SDL_GetRenderDrawColor(renderer,&mut r,&mut g,&mut b,&mut a);

 # [cfg(feature="use_sdl3")]
            circleRGBA(renderer,x as f32,y as f32,rad as f32,r,g,b,a);
 # [cfg(feature="use_sdl2")]
            circleRGBA(renderer,x as i16,y as i16,rad as i16,r,g,b,a);
            let mut tmp=rect_type!{0,0,0,0};
            rect_get_union(&self.dirty_rect_tbl[self.render_page],
                &rect_type!{x-rad,y-rad,rad*2,rad*2},
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
            let renderer=get_sdl_renderer(self.p_app);
            SDL_GetRenderDrawColor(renderer,&mut r,&mut g,&mut b,&mut a);

 # [cfg(feature="use_sdl3")]
            filledCircleRGBA(renderer,x as f32,y as f32,rad as f32,r,g,b,a);
 # [cfg(feature="use_sdl2")]
            filledCircleRGBA(renderer,x as i16,y as i16,rad as i16,r,g,b,a);

                let mut tmp=rect_type!{0,0,0,0};
            rect_get_union(&self.dirty_rect_tbl[self.render_page],
                &rect_type!{x-rad,y-rad,rad*2,rad*2},
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
                let src_rect=rect_type!{0,0,(*surf).w,(*surf).h};
                let dst_rect=rect_type!{x,y,(*surf).w,(*surf).h};
 # [cfg(feature="non_bindings")]
                let renderer=self.sdl_renderer;
 # [cfg(not(feature="non_bindings"))]
                let renderer=get_sdl_renderer(self.p_app);
                let t_r=Texture::from_surface(renderer,surf);
 # [cfg(feature="use_sdl3")]
                SDL_DestroySurface(surf);                
 # [cfg(feature="use_sdl2")]
                SDL_FreeSurface(surf);                

                if let Ok(tex)=t_r{
 # [cfg(feature="use_sdl3")]
                        self.copy_tex_sdl3(&tex,Some(&src_rect),&dst_rect);
 # [cfg(feature="use_sdl2")]
                        self.copy_tex_sdl2(&tex,Some(&src_rect),&dst_rect);
                        let mut tmp=rect_type!{0,0,0,0};
                        rect_get_union(&self.dirty_rect_tbl[self.render_page],
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
            *i=rect_type![0,0,0,0];
        }
    }    
}