use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::DbConn;

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub user_id: String,
    pub nickname: String,
    pub email: String,
    pub phone_number: String,
    pub password_hash: String,
    pub last_login_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]

pub struct UserCreate {
    pub user_id: String,
    pub nickname: String,
    pub email: String,
    pub phone_number: String,
    pub password_hash: String,
    pub last_login_at: Option<NaiveDateTime>,
}

impl User {
    pub fn create(conn: &mut DbConn, data: &UserCreate) -> anyhow::Result<usize> {
        use crate::schema::user;
        let result = diesel::insert_into(user::table)
            .values(data)
            .execute(conn)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
}