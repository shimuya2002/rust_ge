///アプリケーションへの入力関連を集めたモジュール

impl App{

    /// ボタンの押下状態を返す
    /// 'idx' 確認を行うボタン
    pub fn button(&self,idx:usize)->bool{
        return self.button_state_buf[(self.button_buf_idx+1) & 1][idx];
    }
    /// マウスのクリック状態を返す
    pub fn click(&self)->bool{
        return self.mouse_button_state_buf[(self.button_buf_idx+1) & 1];
    }
    /// マウスクリックをした座標を返す
    pub fn click_pos(&self)->&SDL_Point{
        return &self.click_pos;
    }
    /// 指定された時間待機する
    /// 'ms'　待機するミリ秒
    pub fn wait(&self,ms:u32){
        unsafe{
            SDL_Delay(ms);
        }
    }

}