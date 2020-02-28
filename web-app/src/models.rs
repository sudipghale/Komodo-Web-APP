use super::schema::account;
#[derive(Insertable)]
#[table_name = "account"]
pub struct NewAccount<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}
#[derive(Queryable)]
pub struct Account {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
}
