#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate multipart;



use rocket::http::RawStr;
use rocket::response::Redirect;
use rocket_contrib::Template;

//mod image;


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
    Template::render("login", &context)
}



use rocket::request::FromForm;

/*#[get("/creater/<user>", rank = 2)]              // <- route attribute
fn admin(user: &RawStr) -> String {  // <- request handler
    format!("{}の個人ページ", user.as_str())
}*/
/*#[get("/creater/account")]              // <- route attribute
fn user() -> Template {  // <- request handler
   let context = TemplateRenderTest02{
        text: "hogehoge".to_string()
        //nameという文字列がHome.html.teraの{{name}}に渡される
    };
    Template::render("creater_1", &context)
}*/
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
use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use diesel::QueryDsl;


// An alias to the type for a pool of Diesel Mysql Connection
type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

// The URL to the database, set via the `DATABASE_URL` environment variable.
static DATABASE_URL: &str = env!("DATABASE_URL");

/// Initialize the database pool.
fn connect() -> MysqlPool {
    let manager = ConnectionManager::<MysqlConnection>::new(DATABASE_URL);
    Pool::new(manager).expect("Failed to create pool")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
struct Connection(pub PooledConnection<ConnectionManager<MysqlConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<MysqlPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &Connection as an &MysqlConnection.
impl Deref for Connection {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


use diesel::prelude::*;


mod schema{
    table! {
    posts (id) {
        id -> Nullable<Integer>,
        title -> VarChar,
        //published -> Datetime,
        body -> Text,
        regulation -> Bool,
    }
}
}


use self::schema::posts;
use self::schema::posts::dsl::{posts as all_posts, regulation as post_regulation};

#[derive(Serialize, Queryable, Debug,Clone,Insertable)]
#[table_name = "posts"]
struct Post {
    id: Option<i32>,
    title: String,
    body: String,
    regulation: bool
}

#[derive(FromForm)]
struct PostForm{
    title: String,
    body: String,
/*    regulation: bool,*/
}


#[derive(Serialize, Queryable, Debug,Clone)]
#[table_name = "user"]
struct User{
    id: Option<i32>,
    name: String,
    mail_address: String,
    account: String,
    password: String,
}

#[derive(Debug,Serialize)]
struct Context{
    post: Vec<Post>
}

fn read(connection: &MysqlConnection) -> Vec<Post> {
    all_posts.order(posts::id.desc()).load::<Post>(connection).unwrap()
}
fn insert(postform:PostForm, conn: &MysqlConnection) -> bool{
    let t = Post{
        id: None,
        title: postform.title,
        body: postform.body,
        regulation: false
    };
/*    println!("insert method");
    println!("{}&{}",t.title,t.body);

    let a = diesel::insert_into(posts::table).values(&t).execute(conn).unwrap();

    //上の一行をコメントアウトすると一度のPOSTで二つ同じものをinsertすることになる（バグ）

    println!("{:?}",a);*/
    diesel::insert_into(posts::table).values(&t).execute(conn).is_ok()
}
use rocket::response::Flash;

#[post("/text", data = "<toukou_form>")]
fn new(toukou_form: Form<PostForm>, connection: Connection) -> Flash<Redirect>{
    let t = toukou_form.into_inner();
    println!("{}",t.body);
    println!("{}",t.title);

    println!("postを通ってます。");
    if insert(t,&connection) {
        println!("成功してる");

        Flash::success(Redirect::to("/creater/account"), "成功してる")
    } else {
        println!("失敗");
        Flash::error(Redirect::to("/creater/account"), "失敗した。")
    }
}


#[post("/form", data = "<toukou>")]
fn article(toukou: Form<PostForm>, connection: Connection) -> Flash<Redirect>{
    let t = toukou.into_inner();
    println!("{}",t.body);
    println!("{}",t.title);

    println!("post");
    if insert(t,&connection) {
        println!("成功");
        Flash::success(Redirect::to("/creater/account"), "成功してる")
    } else {
        println!("失敗");
        Flash::error(Redirect::to("/creater/account"), "失敗した。")
    }
}


use std::env;
use std::io;
use rocket::Data;


use std::io::Read;
use std::fs;
use std::fs::File;

use std::io::Write;

extern crate rocket_static_fs;

#[post("/upload", data = "<data>")]
fn upload(data: Data) -> io::Result<Redirect> {
    println!("upload function");
    let mut body:Vec<u8> = vec![];
    //let path = env::temp_dir().join("upload");

    let aaa = data.stream_to(&mut body).unwrap() as usize;

    let mut f = File::create("static/post_image/hoge.jpg").unwrap();
    f.write_all(&body);
    Ok(Redirect::to("/"))
}

#[get("/<path..>", rank = 6)]
//creater/hogehogeにstaticディレクトリを適用する
fn files(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/post_image").join(path)).ok()
}


impl Context{
    fn row(connection: &MysqlConnection) -> Context{
        Context{post: read(connection)}
    }
}


//fn raw(conn: &Connection) -> Vec<Post>{
//    post: Context::read()
//}


#[get("/creater/account")]
fn hoge(connection: Connection) -> Template {
    println!("get中");
    Template::render("creater_1", Context::row(&connection))
}
//databases



fn main() {
    rocket::ignite()
        .mount("/", routes![
home,creater,images,about_me,signup,login,
all,creater_static,hoge,files
])
        .mount("/creater/account/post/", routes![new,article,upload])
        .manage(connect())
        .attach(Template::fairing())

        .launch();
}