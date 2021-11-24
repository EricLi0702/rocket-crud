use crate::database::last_insert_id;
use crate::guards::auth::Claims;
use crate::schema::users;
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Queryable, Serialize, Deserialize, Insertable, AsChangeset, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserAuth {
    pub user: Option<User>,
    pub token: Option<String>,
}

impl User {
    pub fn all(conn: &MysqlConnection) -> Vec<User> {
        let results: Vec<User> = users::table
            .select(users::all_columns)
            .load::<User>(conn)
            .expect("Cannot load users")
            .into_iter()
            .collect();
        results
    }

    pub fn find(_id: i64, conn: &MysqlConnection) -> Option<User> {
        let result = users::table.find(_id).first::<User>(conn);
        match result {
            Ok(data) => Some(data),
            Err(_) => None,
        }
    }

    pub fn find_by_email_and_password(
        _email: &str,
        _password: &str,
        conn: &MysqlConnection,
    ) -> Option<User> {
        let user = users::table
            .filter(users::email.eq(_email))
            .order(users::id)
            .first::<User>(conn)
            .ok()?;
        if bcrypt::verify(_password, user.password.as_deref().unwrap()) {
            return Some(user);
        }
        None
    }

    pub fn create(data: User, conn: &MysqlConnection) -> User {
        let user = User {
            password: Some(bcrypt::hash(data.password.as_deref().unwrap()).expect("hash error")),
            ..data
        };

        let res = diesel::insert_into(users::table)
            .values(&user)
            .execute(conn);
        println!("{:?}", res);

        let generated_id: i64 = diesel::select(last_insert_id).first(conn).unwrap();
        let result: User = users::table
            .find(generated_id)
            .first::<User>(conn)
            .expect("Cannot load user");
        result
    }

    pub fn update(_id: i64, data: User, conn: &MysqlConnection) -> User {
        let target = users::table.find(_id);
        let res = diesel::update(target).set(data).execute(conn);
        println!("{:?}", res);
        let result = target.first::<User>(conn).expect("Cannot load user");
        result
    }

    pub fn destroy(_id: i64, conn: &MysqlConnection) -> bool {
        let result = diesel::delete(users::table.find(_id)).execute(conn).is_ok();
        result
    }

    pub fn to_user_auth(&self, secret: &str) -> UserAuth {
        let exp = Utc::now() + Duration::days(60); // TODO: move to config
        let token = Claims {
            sub: self.id.unwrap_or(0),
            exp: exp.timestamp(),
        }
        .token(secret);
        UserAuth {
            user: Some(self.clone()),
            token: Some(token),
        }
    }
}
