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
    use rocket_contrib::templates::Template;

    // todo mount them

    /// 本を登録する
    #[post("/", data = "<string>")]
    pub fn register(string: String) -> Result<Status, Custom<Template>> {
        let (mut data, register): (EBookmarkData, request::Register) = communication(&string)?;

        let book = Book::from(register);

        data.push(book);

        write_data_or_err_template(&data)?;

        Ok(Status::Ok)
    }

    /// 本を削除する
    #[post("/", data = "<string>")]
    pub fn delete(string: String) -> Result<Status, Custom<Template>> {
        let (mut data, register): (EBookmarkData, request::Delete) = communication(&string)?;

        let request::Delete { id, .. } = register;

        let removed = data.remove(&id).ok_or({
            unimplemented!()
            // todo 500 Internal Server Error
        })?;

        write_data_or_err_template(&data)?;

        Ok(Status::Ok)
    }
}