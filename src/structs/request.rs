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

#[derive(Serialize, Deserialize, Debug)]
pub struct Search {
    /// 探したい本のタイトル
    pub title: Option<String>,
    /// 探したい本についているtag
    pub tags: Option<Vec<String>>,
    /// ブックマークか否か
    pub is_bookmark: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetOne {
    /// 検索対象の本に対応したNFCのID
    pub id: String,
}