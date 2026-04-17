//SDL2版の描画処理

impl App{

    /// テクスチャ描画
    /// 'tex' 描画元のテクスチャ
    /// 'src_rect' 描画するtex内の座標
    ///            Noneの場合はテクスチャ全体を描画
# [cfg(feature="use_sdl2")]
    pub fn copy_tex_sdl2(&mut self,tex:&Texture,src_rect:Option<&RectType>,dst_rect:&RectType){
        unsafe{
 # [cfg(feature="non_bindings")]
            let renderer=self.sdl_renderer;
 # [cfg(not(feature="non_bindings"))]
            let renderer=get_sdl_renderer(self.p_app);

            SDL_RenderCopy(renderer,
                tex.tex,
                if let Some(src_rect_ref)=src_rect{src_rect_ref}else{null_mut()},
                dst_rect);
        }
    
    }
    ///グラフィックページの描画
    /// 'idx' 描画するグラフィックページ
    /// 'src_rect' 描画するグラフィックページ内の座標
    /// 'dst_rect' 描画先の座標
# [cfg(feature="use_sdl2")]
    pub fn copy_sdl2(&mut self,idx:usize,src_rect:&RectType,dst_rect:&RectType){
        unsafe{
 # [cfg(feature="non_bindings")]
            let renderer=self.sdl_renderer;
 # [cfg(not(feature="non_bindings"))]
            let renderer=get_sdl_renderer(self.p_app);
            SDL_RenderCopy(renderer,self.g_pages[idx],src_rect,dst_rect);
        }
    }
    ///矩型塗りつぶし
    ///'rect' 塗りつぶしを行う座標
 # [cfg(feature="use_sdl2")]
    pub fn fill_rect_sdl2(&mut self,rect:&SDL_Rect){
        unsafe{
 # [cfg(feature="non_bindings")]
            let renderer=self.sdl_renderer;
 # [cfg(not(feature="non_bindings"))]
            let renderer=get_sdl_renderer(self.p_app);

            SDL_RenderFillRect(renderer,rect);

        }
    }
    ///矩型描画
    ///'rect' 描画を行う座標
# [cfg(feature="use_sdl2")]
    pub fn draw_rect_sdl2(&mut self,rect:&SDL_Rect){
        unsafe{
 # [cfg(feature="non_bindings")]
            let renderer=self.sdl_renderer;
 # [cfg(not(feature="non_bindings"))]
            let renderer=get_sdl_renderer(self.p_app);

            SDL_RenderDrawRect(renderer,rect);

        }
    }

}