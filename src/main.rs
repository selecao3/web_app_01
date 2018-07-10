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
    name: String,

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



use rocket::request::FromForm;

/*#[get("/creater/<user>", rank = 2)]              // <- route attribute
fn admin(user: &RawStr) -> String {  // <- request handler
    format!("{}の個人ページ", user.as_str())
}*/
#[get("/creater/account")]              // <- route attribute
fn user() -> Template {  // <- request handler
   let context = TemplateRenderTest02{
        text: "hogehoge".to_string()
        //nameという文字列がHome.html.teraの{{name}}に渡される
    };
    Template::render("creater_1", &context)
}
/*#[get("/creater/account", rank = 3)]
fn redirect_admin() -> Redirect {
    Redirect::to("/login")
}*/
//getメソッド群終わり


use rocket::http::{Cookie, Cookies};
use rocket::request::Form;

#[derive(FromForm)]
struct Profile{
    profile: String,
    //個人ページの自己紹介
}

#[derive(Serialize)]
struct TemplateRenderTest02{
    text: String,
    //writting text in user's textarea.
    //you can use in template.

}
//postメソッド群
//postアトリビュートのurlは、formのURIに対応している。
#[post("/post/contribute", data = "<profile_form>")]
fn contribute(profile_form: Form<Profile>) -> Template{
    let profile = TemplateRenderTest02{
        text: profile_form.into_inner().profile
    };
    Template::render("profile_text_template", profile)
    //テキストエリアでpostされた文章を返す関数
}


/*#[post("/post/sign_in", data = "<sign_in>")]
fn signin(sign_in: Form<Profile>) -> String{
}*/

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

/*#[post("/post/sign_up", data = "<sign_up>")]
fn regist(sign_up: Form<User>) -> String{

}*/
/*#[post("/post/logout", data = "<logout>")]
fn logout(logout: Form<Profile>) -> String{

}*/


//staticファイルを伝えるメソッド
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/<path..>", rank = 5)]
fn all(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}
#[get("/creater/<path..>", rank = 4)]
//creater/hogehogeにstaticディレクトリを適用する
fn creater_static(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}
//staticファイルを伝えるメソッド終わり

//databases
#[macro_use] extern crate diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

// An alias to the type for a pool of Diesel SQLite connections.
type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

// The URL to the database, set via the `DATABASE_URL` environment variable.
static DATABASE_URL: &'static str = env!("DATABASE_URL");

/// Initializes a database pool.
fn init_pool() -> MysqlPool{
    let manager = ConnectionManager::<MysqlConnection>::new(DATABASE_URL);
    Pool::new(manager).expect("db pool")
}
struct Connection(PooledConnection<ConnectionManager<MysqlConnection>>);


#[derive(Queryable)]
struct Post {
    id: i32,
    title: String,
    body: String,
    published: bool,
}
table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}
fn read(connection: &MysqlConnection) -> Vec<Post> {
    posts::table.order(posts::id).load::<Post>(connection).unwrap()
}
#[get("/hoge")]
fn hoge(connection: Connection) -> Template {
    Template::render("creater_1", read(&connection))
}
//databases


fn main() {
    rocket::ignite()
        .mount("/", routes![
    home,creater,images,about_me,signup,login,
    user,all,creater_static,
    hoge
    ])
        .manage(init_pool())
        .attach(Template::fairing())
        .launch();
}