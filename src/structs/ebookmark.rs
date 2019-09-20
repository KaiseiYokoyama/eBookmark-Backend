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

    pub fn remove(&mut self, id: &str) -> Option<Book> {
        let mut index = None;
        // 該当するidを持つbookを探す
        for (idx, book) in self.books.iter().enumerate() {
            if book.id == id {
                index = Some(idx);
                break;
            }
        }

        let index = index?;
        Some(self.books.remove(index))
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

impl From<request::Register> for Book {
    fn from(reg: request::Register) -> Self {
        let request::Register {
            title, id
        } = reg;

        Book {
            title,
            id,
            ..Book::default()
        }
    }
}