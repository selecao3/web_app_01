use multipart::server::Multipart;


use multipart::server::save::Entries;

use multipart::server::save::SaveResult::*;

use rocket::Data;
use rocket::http::{ContentType, Status};
use rocket::response::Stream;
use rocket::response::status::Custom;

use std::io::{self, Cursor, Write};
use rocket::response::Redirect;
use std::env;





#[post("/unko", data = "<data>")]
// signature requires the request to have a `Content-Type`
fn multipart_upload(cont_type: &ContentType, data: Data) -> Result<Stream<Cursor<Vec<u8>>>, Custom<String>> {
    // this and the next check can be implemented as a request guard but it seems like just
    // more boilerplate than necessary

    let (_, boundary) = cont_type.params().find(|&(k, _)| k == "boundary").ok_or_else(
            || Custom(
                Status::BadRequest,
                "`Content-Type: multipart/form-data` boundary param not provided".into()
            )
        )?;
    //boundaryの取得

    match process_upload(boundary, data) {
        Ok(resp) => Ok(Stream::from(Cursor::new(resp))),
        Err(err) => Err(Custom(Status::InternalServerError, err.to_string()))
    }
}

fn process_upload(boundary: &str, data: Data) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();
    println!("process_upload関数");

    // saves all fields, any field longer than 10kB goes to a temporary directory
    // Entries could implement FromData though that would give zero control over
    // how the files are saved; Multipart would be a good impl candidate though
    match Multipart::with_body(data.open(), boundary).save().with_dir("static/post_image") {
        //全てのフィールドを一旦保存する
        Full(entries) => process_entries(entries, &mut out)?,
        //成功,entriesにはフィールドが全て詰まっている
        Partial(partial, reason) => {
            //途中で失敗した。
            writeln!(out, "Request partially processed: {:?}", reason)?;
            if let Some(field) = partial.partial {
                writeln!(out, "Stopped on field: {:?}", field.source.headers)?;
            }

            process_entries(partial.entries, &mut out)?
        },
        Error(e) => return Err(e),
    }

    Ok(out)
}

// having a streaming output would be nice; there's one for returning a `Read` impl
// but not one that you can `write()` to

use multipart::server::FieldHeaders;
fn process_entries(entries: Entries, mut out: &mut Vec<u8>) -> io::Result<()> {
    {
        println!("======¥n{:?}¥n========",entries.fields.get(&"file".to_string()).unwrap());
        //let aaa = &entries.fields.get(&"file".to_string()).unwrap().get(0).unwrap().data;

    }

    writeln!(out, "Entries processed")
}

