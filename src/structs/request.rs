/// 本の登録
#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    /// NFCタグのID
    id: String,
    /// 本のタイトル
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Delete {
    /// 削除対象の本に対応したNFCのID
    id: String,
}