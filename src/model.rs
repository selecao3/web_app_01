#[derive(Serialize, Queryable, Debug,Clone,Insertable)]
#[table_name = "posts"]
struct Post {
    id: Option<i32>,
    title: String,
    body: String,
    regulation: bool
}
#[derive(Serialize, Queryable, Debug,Clone,Insertable)]
#[table_name = "user"]
struct User{
    id: Option<i32>,
    name: String,
    mail_address: String,
    account: String,
    password : String
}

#[derive(Serialize, Queryable, Debug,Clone,Insertable)]
#[table_name = "profile"]
struct Profile{
    id: Option<i32>,
    img: Option<String>,
    body: String,
    regulation: bool
}
