use sqlx::{ Error, PgPool };
use sqlx::postgres::PgPoolOptions;

use crate::model::{ Users, UsersInfo };

#[derive(Clone)]
pub struct UserService {
    pool: PgPool
}

impl UserService {
    pub async fn new() -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres@localhost/rust")
            .await?;
        Ok(Self {pool})
    }
    pub async fn list_users(&self, offset:i32, limit:i32) -> Result<Vec<Users>, Error> {
        let users = sqlx::query_as::<_, Users>("SELECT id, name, occupation, email, phone FROM users ORDER BY 
         id ASC OFFSET $1 LIMIT $2")
            .bind(offset)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }
    pub async fn get_users_by_id(&self, id: i32) -> Result<Users, Error> {
        let user = sqlx::query_as::<_,Users>("SELECT id, name, occupation, email, phone FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }
    pub async fn create_user(&self, user: UsersInfo) -> Result<(), Error> {
        sqlx::query("INSERT INTO users (name, occupation, email, phone) VALUES ($1, $2, $3, $4)")
            .bind(user.name)
            .bind(user.occupation)
            .bind(user.email)
            .bind(user.phone)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    pub async fn update_user(&self, user: Users) -> Result<(), Error> {
        sqlx::query("UPDATE users SET name = $1, 
        occupation = $2, email = $3, phone = $4 WHERE id = $5")
            .bind(user.name)
            .bind(user.occupation)
            .bind(user.email)
            .bind(user.phone)
            .bind(user.id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    pub async fn delete_user(&self, id: i32) -> Result<(), Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
