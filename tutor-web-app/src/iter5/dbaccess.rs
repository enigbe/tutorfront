use crate::errors::TutorError;
use crate::model::*;
use sqlx::postgres::PgPool;

pub async fn get_user_record(pool: &PgPool, username: String) -> Result<User, TutorError> {
    let user_row = sqlx::query_as!(
        User,
        "select * from user_table where username = $1",
        username
    )
    .fetch_optional(pool)
    .await?;

    if let Some(user) = user_row {
        Ok(user)
    } else {
        Err(TutorError::NotFound("User name not found".into()))
    }
}

pub async fn post_new_user(pool: &PgPool, new_user: User) -> Result<User, TutorError> {
    let user_row= sqlx::query_as!(User,"insert into user_table (username, tutor_id, user_password) values ($1,$2,$3) returning username, tutor_id, user_password",
    new_user.username, new_user.tutor_id, new_user.user_password)
    .fetch_one(pool)
    .await?;

    Ok(user_row)
}
