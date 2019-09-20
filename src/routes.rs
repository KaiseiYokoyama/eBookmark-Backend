use crate::prelude::*;
use rocket::{
    http::Status,
    response::status::Custom,
};
use rocket_contrib::templates::Template;

fn communication<'de, T: Deserialize<'de>>(string: &'de String) -> Result<(EBookmarkData, T), Custom<Template>> {
    let data = read_data().map_err(|e| {
        unimplemented!()
        // todo 500 Internal Server Error
    })?;

    let strct: T = serde_json::from_str::<'de>(&string).map_err(|e| {
        unimplemented!()
        // todo 500 Internal Server Error
    })?;

    Ok((data, strct))
}

fn write_data_or_err_template(data: &EBookmarkData) -> Result<(), Custom<Template>> {
    write_data(&data).map_err(|e| {
        unimplemented!()
        // todo 500 Internal Server Error
    })
}

pub mod register {
    use super::*;

    /// 本を登録する
    #[post("/", data = "<string>")]
    pub fn register(string: String) -> Result<Status, Custom<Template>> {
        let (mut data, register): (EBookmarkData, request::Register) = communication(&string)?;

        let book = Book::from(register);

        data.push(book);

        write_data_or_err_template(&data)?;

        Ok(Status::Ok)
    }
}

pub mod delete {
    use super::*;

    /// 本を削除する
    #[post("/", data = "<string>")]
    pub fn delete(string: String) -> Result<Status, Custom<Template>> {
        let (mut data, register): (EBookmarkData, request::Delete) = communication(&string)?;

        let request::Delete { id, .. } = register;

        // 削除
        let _removed = data.remove(&id).ok_or({
            unimplemented!()
            // todo 500 Internal Server Error
        })?;

        write_data_or_err_template(&data)?;

        Ok(Status::Ok)
    }
}

pub mod search {
    use super::*;

    #[post("/", data = "<string>")]
    pub fn search(string: String) -> Result<String, Custom<Template>> {
        let (data, search): (EBookmarkData, request::Search) = communication(&string)?;

        // 検索
        let results = data.search(&search);

        let string = serde_json::to_string(&results).map_err(|e| {
            unimplemented!()
            // todo 500 Internal Server Error
        })?;

        Ok(string)
    }
}

pub mod get {
    use super::*;

    pub mod one {
        use super::*;

        /// idに対応する本を取得する
        #[post("/one", data = "<string>")]
        pub fn get_one(string: String) -> Result<Option<String>, Custom<Template>> {
            let (data, get_one): (EBookmarkData, request::GetOne) = communication(&string)?;

            let book = data.get_by_id(&get_one.id);

            if let Some(book) = book {
                match serde_json::to_string(book).map_err(|e| {
                    unimplemented!()
                    // todo 500 Internal Server Error
                }) {
                    Ok(s) => Ok(Some(s)),
                    Err(e) => Err(e),
                }
            } else {
                // 該当なし
                Ok(None)
            }
        }
    }

    pub mod all {
        use super::*;

        /// 登録されている本をすべて取得する
        #[get("/all")]
        pub fn get_all() -> Result<String, Custom<Template>> {
            let data = read_data().map_err(|e| {
                unimplemented!()
                // todo 500 Internal Server Error
            })?;

            serde_json::to_string(&data.books).map_err(|e| {
                unimplemented!()
                // todo 500 Internal Server Error
            })
        }
    }
}

pub mod books {
    use super::*;

    #[get("/<id>")]
    pub fn book(id: String) -> Option<Template> {
        let data = read_data().ok()?;

        let book = data.get_by_id(&id)?;

        // todo bookのデータとTemplateを用いたページの配信
        unimplemented!()
    }
}