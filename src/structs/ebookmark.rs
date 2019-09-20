use crate::prelude::*;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct EBookmarkData {
    pub books: Vec<Book>,
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

    pub fn search<'a>(&'a self, search: &request::Search) -> Vec<&'a Book> {
        let iter = self.books.iter().clone();

        let mut vec = iter
            // 絞り込み
            .filter(|b| {
                // タグの条件
                (
                    if let Some(tags) = &search.tags {
                        // 条件にあるタグを全て含む
                        tags.iter().all(|t| b.tags.contains(t))
                    } else {
                        // 条件なし
                        true
                    }
                ) && (
                    if let Some(bookmark) = search.is_bookmark.clone() {
                        bookmark == b.is_bookmark
                    } else {
                        // 条件なし
                        true
                    }
                )
            }).collect::<Vec<&Book>>();

        // ソート
        if let Some(title) = &search.title {
            vec.sort_by(|a, b| {
                levenshtein::levenshtein(title, &a.title).cmp(&levenshtein::levenshtein(title, &b.title))
            });
            vec.reverse();
        }

        vec
    }

    pub fn get_by_id<'a>(&'a self, id: &str) -> Option<&'a Book> {
        let mut book_res = None;
        // 該当するidを持つbookを探す
        for book in self.books.iter() {
            if book.id == id {
                book_res = Some(book);
                break;
            }
        }

        book_res
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