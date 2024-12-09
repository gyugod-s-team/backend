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

    pub fn cleanup(conn: &mut DbConn) -> anyhow::Result<usize> {
        use crate::schema::user;
        let result = diesel::delete(user::table)
            .execute(conn)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::get_connection;

    use super::*;

    #[test]
    fn test_create() {
        let mut conn = get_connection().unwrap();
        let data = UserCreate {
            user_id: "test".to_string(),
            nickname: "test".to_string(),
            email: "test".to_string(),
            phone_number: "test".to_string(),
            password_hash: "test".to_string(),
            last_login_at: None,
        };
        let result = User::create(&mut conn, &data).unwrap();
        assert_eq!(result, 1);

        let result = User::cleanup(&mut conn).unwrap();
        assert_eq!(result, 1);
    }
}