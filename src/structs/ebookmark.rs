use crate::prelude::*;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct EBookmarkData {
    books: Vec<Book>,
}

impl EBookmarkData {
    pub fn push(&mut self, book: Book) {
        self.books.push(book);
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Book {
    /// 本のタイトル
    title: String,
    /// 対応するタグのID
    id: String,
    /// タグの一覧
    tags: Vec<String>,
    /// ブックマークされているか否か
    is_bookmark: bool,
    /// メモ
    memo: String,
    // todo 読書記録に対応
}

impl From<Register> for Book {
    fn from(reg: Register) -> Self {
        let Register {
            title, id
        } = reg;

        Book {
            title,
            id,
            ..Book::default()
        }
    }
}