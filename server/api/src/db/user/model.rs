use chrono::NaiveDateTime;
use super::super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub rating: i32,
    pub joined: NaiveDateTime,
    pub banned: bool,
    pub avatar: String
}

impl Clone for User {
    fn clone(&self) -> User {
        User {
            id: self.id.clone(),
            firstname: self.firstname.clone(),
            lastname: self.lastname.clone(),
            username: self.username.clone(),
            password: self.password.clone(),
            email: self.email.clone(),
            rating: self.rating.clone(),
            joined: self.joined.clone(),
            banned: self.banned.clone(),
            avatar: self.avatar.clone()
        }
    }
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub firstname: &'a String,
    pub lastname: &'a String,
    pub username: &'a String,
    pub password: &'a String,
    pub email: &'a String
}