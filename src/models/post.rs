use crate::database::last_insert_id;
use crate::schema::posts;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Insertable, AsChangeset, Queryable, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: bool,
}

impl Post {
    pub fn all(conn: &MysqlConnection) -> Vec<Post> {
        let results: Vec<Post> = posts::table
            .select(posts::all_columns)
            .filter(posts::published.eq(true))
            .load::<Post>(conn)
            .expect("Cannot load posts")
            .into_iter()
            .collect();
        results
    }

    pub fn find(_id: i64, conn: &MysqlConnection) -> Option<Post> {
        let result = posts::table.find(_id).first::<Post>(conn);
        match result {
            Ok(data) => Some(data),
            Err(_) => None,
        }
    }

    pub fn create(data: Post, conn: &MysqlConnection) -> Post {
        let res = diesel::insert_into(posts::table).values(data).execute(conn);
        println!("{:?}", res);

        let generated_id: i64 = diesel::select(last_insert_id).first(conn).unwrap();
        let result: Post = posts::table
            .find(generated_id)
            .first::<Post>(conn)
            .expect("Cannot load post");
        result
    }

    pub fn update(_id: i64, data: Post, conn: &MysqlConnection) -> Post {
        let target = posts::table.find(_id);
        let res = diesel::update(target).set(data).execute(conn);
        println!("{:?}", res);

        let result = target.first::<Post>(conn).expect("Cannot load post");
        result
    }

    pub fn destroy(_id: i64, conn: &MysqlConnection) -> bool {
        let result = diesel::delete(posts::table.find(_id)).execute(conn).is_ok();
        result
    }
}
