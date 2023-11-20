use diesel::prelude::*;
use crate::models::user::User;
use chrono::{ DateTime, Local };

#[derive(Queryable, Selectable, Associations)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(belongs_to(User, foreign_key = author))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
  pub id: i32,
  pub content: String,
  pub publish_date: DateTime::<Local>,
  pub author: Option<i32>
}

