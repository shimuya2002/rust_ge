///グラフィックページの数
pub const G_PAGE_NUM:usize=8;
///描画対象のグラフィックページ(バックバッファ)
pub const RENDER_GPAGE:usize=0;
///背景用のグラフィックページ
pub const BG_GPAGE:usize=1;
///バストアップ用のグラフィックページ
pub const BUSTUP_GPAGE:usize=BG_GPAGE+1;
///テキスト用のグラフィックページ
pub const TEXT_GPAGE:usize=BUSTUP_GPAGE+1;
///ウィンドゥ用のグラフィックページ
pub const UI_GPAGE1:usize=TEXT_GPAGE+1;
///ポップアップリスト用のグラフィックページ
pub const UI_GPAGE2:usize=UI_GPAGE1+1;
///プレイヤーキャラクタ用のグラフィックページ
pub const PLAYER_CHARA_GPAGE:usize=UI_GPAGE2+1;

pub const ANIM_SET_NUM:usize=32;
///ウインドウの横幅
pub const WND_W:i32=640;
///ウインドウの縦幅
pub const WND_H:i32=480;
///キャッシュの最大保持数
pub const CACHE_MAX_NUM:usize=8;

///状態を管理するボタンの数
pub const BUTTON_NUM:usize=16;
///上ボタン、カーソルキー上
pub const BUTTON_UP:usize=0;
///下ボタン、カーソルキー下
pub const BUTTON_DOWN:usize=1;
///左ボタン、カーソルキー左
pub const BUTTON_LEFT:usize=2;
///右ボタン、カーソルキー右
pub const BUTTON_RIGHT:usize=3;

pub const BUTTON_A:usize=4;
pub const BUTTON_B:usize=5;
pub const BUTTON_X:usize=6;
pub const BUTTON_Y:usize=7;
pub const BUTTON_L:usize=8;
pub const BUTTON_R:usize=9;
pub const BUTTON_UNUSED:usize=10;
pub const BUTTON_ZR:usize=11;
pub const BUTTON_ZL:usize=12;

///左バストアップ
pub const LEFT_IMAGE:i32=0;
///右バストアップ
pub const RIGHT_IMAGE:i32=1;
///背景
pub const BG_IMAGE:i32=2;
pub const PLAYER_CHARA_IMAGE:i32=3;
pub const TEXT_LOG_NUM:usize=32;
pub const TEXT_LEN:usize=128;
///フォントファイルパス
pub const FONT_FILE_PATH:&str="./assets/NotoSansJP-Black.ttf";
///テキストメッセージのフォントサイズ
pub const MSG_TEXT_SIZE:i32=16;
///UIのフォントサイズ
pub const UI_TEXT_SIZE:i32=16;
pub const USE_MSG_FONT:i32=0;
pub const USE_UI_FONT:i32=USE_MSG_FONT+1;
pub const UI_BORDER_SIZE:i32=8;

///画面のフレームレート
pub const DEF_FRAME_RATE:u32=16;

///ローグ型ダンジョンのデフォルトの横幅
pub const ROUGE_DEF_W:i32=17;
///ローグ型ダンジョンのデフォルトの縦幅
pub const ROUGE_DEF_H:i32=13;
///ローグ型ダンジョンの1フロア当たりの最大部屋数
pub const ROUGE_ROOM_MAX:usize=5;
pub const ROUGE_ROOM_GEN_PARAM:i32=2;
pub const ROUGE_ROOM_MIN_W:i32=3;
pub const ROUGE_ROOM_MIN_H:i32=3;
pub const ROUGE_ROOM_MAX_W:i32=17;
pub const ROUGE_ROOM_MAX_H:i32=13;
