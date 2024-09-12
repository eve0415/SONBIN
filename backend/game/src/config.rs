use serde::{Deserialize, Serialize};

/// ゲームモード
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GameMode {
    NORMAL,
    /// パチスロ
    SLOT,
    /// サイコロ
    DICE,
    /// 麻雀
    MAHJONG,
    /// 2進数
    BINARY,
    /// 16進数
    HEXADECIMAL,
    /// 2進数と16進数
    MIX,
    /// 数式
    FORMULA,
    /// 絵文字
    EMOJI,
    /// カラーコード
    COLOR,
    /// フォント
    FONT,
    /// 著名人
    CELEBRITY,
    /// 国旗
    FLAG,
    /// 異国語数字
    FOREIGN,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct GameSettings {
    /// 何回でもビンゴできるかどうか
    pub multiple_bingo: bool,
    pub auto_open: bool,
    /// 最大プレイヤー数
    pub max_player: Option<usize>,
}
