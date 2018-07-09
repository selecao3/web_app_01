#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;


use rocket::http::RawStr;
use rocket::response::Redirect;
use rocket_contrib::Template;

#[derive(Serialize)]
struct TemplateRenderTest{
    name: String
}
//テンプレートファイルに渡すstruct

//getメソッド群
#[get("/")]
fn home() -> Template{
    let context = TemplateRenderTest{
        name: "name".to_string()
        //nameという文字列がHome.html.teraの{{name}}に渡される
    };
    Template::render("news", &context)
}

/*#[get("/")]
fn news() -> Template{
    let context = TemplateRenderTest{
        name: "name".to_string()
        //nameという文字列がHome.html.teraの{{name}}に渡される
    };
    Template::render("news", &context)
}*/

#[get("/creater", rank = 2)]              // <- route attribute
fn creater() -> Template{  // <- request handler
    let context = TemplateRenderTest{
        name: "name".to_string()
        //nameという文字列がHome.html.teraの{{name}}に渡される
    };
    Template::render("creaters", &context)
}

#[get("/images")]              // <- route attribute
fn images() -> Template {  // <- request handler
    let context = TemplateRenderTest{
        name: "name".to_string()
        //nameという文字列がHome.html.teraの{{name}}に渡される
    };
    Template::render("gallary", &context)
}
#[get("/about_me")]              // <- route attribute
fn about_me() -> Template {  // <- request handler
    let context = TemplateRenderTest{
        name: "name".to_string()
        //nameという文字列がHome.html.teraの{{name}}に渡される
    };
    Template::render("about_me", &context)
}

#[get("/signup")]              // <- route attribute
fn signup() -> Template {  // <- request handler
    let context = TemplateRenderTest{
        name: "name".to_string()
        //nameという文字列がHome.html.teraの{{name}}に渡される
    };
    Template::render("sign_up", &context)
}
#[get("/login")]              // <- route attribute
fn login() -> Template {  // <- request handler
    let context = TemplateRenderTest{
        name: "name".to_string()
        //nameという文字列がHome.html.teraの{{name}}に渡される
    };
    Template::render("hoge", &context)
}

//落書き
#[derive(FromForm)]
struct User{
    name: String,
    mail_address: String,
    account: String,
    password: String,
    register: bool,
}
//落書き


use rocket::request::FromForm;

#[get("/creater/<user>")]              // <- route attribute
fn admin(user: &RawStr) -> String {  // <- request handler
    format!("{}の個人ページ", user.as_str())
}
#[get("/creater/account", rank = 2)]              // <- route attribute
fn user() -> Template {  // <- request handler
   let context = TemplateRenderTest{
        name: "name".to_string()
        //nameという文字列がHome.html.teraの{{name}}に渡される
    };
    Template::render("creater_1", &context)
}
#[get("/creater/account", rank = 3)]
fn redirect_admin() -> Redirect {
    Redirect::to("/login")
}
//getメソッド群終わり


use rocket::http::{Cookie, Cookies};
use rocket::request::Form;

#[derive(FromForm)]
struct Profile{
    profile: String,
    //個人ページの自己紹介
}

//postメソッド群
//postアトリビュートのurlは、formのURIに対応している。
#[post("/post/contribute", data = "<profile_form>")]
fn contribute(profile_form: Form<Profile>) -> String{
    profile_form.into_inner().profile
    //テキストエリアでpostされた文章を返す関数
}
/*#[post("/post/sign_in", data = "<sign_in>")]
fn signin(sign_in: Form<Profile>) -> String{
}
#[post("/post/sign_up", data = "<sign_up>")]
fn regist(sign_up: Form<Profile>) -> String{
}
#[post("/post/logout", data = "<logout>")]
fn logout(logout: Form<Profile>) -> String{
}*/

use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/<path..>", rank = 5)]
fn all(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}


fn main() {
    rocket::ignite()
        .mount("/", routes![
    home,creater,images,about_me,signup,login,
    admin,user,redirect_admin,all
    ])
        .attach(Template::fairing())
        .launch();
}