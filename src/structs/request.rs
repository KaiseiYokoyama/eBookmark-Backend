/// 本の登録
#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    /// NFCタグのID
    pub id: String,
    /// 本のタイトル
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Delete {
    /// 削除対象の本に対応したNFCのID
    pub id: String,
}