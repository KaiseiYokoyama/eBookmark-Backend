use crate::prelude::*;

pub mod register {
    use super::*;
    use rocket::{
        http::Status,
        response::status::Custom,
    };
    use rocket_contrib::templates::Template;

    #[post("/", data = "<string>")]
    pub fn register(string: String) -> Result<Status, Custom<Template>> {
        let mut data = read_data().map_err(|e| {
            unimplemented!()
            // todo 500 Internal Server Error
        })?;

        let register: request::Register = serde_json::from_str(&string).map_err(|e| {
            unimplemented!()
            // todo 500 Internal Server Error
        })?;

        let book = Book::from(register);

        data.push(book);

        write_data(&data)?;

        Ok(Status::Ok)
    }
}