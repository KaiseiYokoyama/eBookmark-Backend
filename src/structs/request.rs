/// 本の登録
#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    /// NFCタグのID
    id: String,
    /// 本のタイトル
    title: String,
}