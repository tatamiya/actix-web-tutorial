use std::ops::Deref;

use diesel::prelude::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

use crate::model::{NewTask, Task};

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;
type SqlitePooledConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn init_pool(database_url: &str) -> Result<SqlitePool, PoolError> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager)
}

fn get_conn(pool: &SqlitePool) -> Result<SqlitePooledConnection, &'static str> {
    pool.get().map_err(|_| "Can't get connection")
}

pub fn get_all_tasks(pool: &SqlitePool) -> Result<Vec<Task>, &'static str> {
    Task::all(get_conn(pool)?.deref()).map_err(|_| "Error retrieving tasks")
}

pub fn create_task(todo: String, pool: &SqlitePool) -> Result<(), &'static str> {
    let new_task = NewTask { description: todo };
    Task::insert(new_task, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error inserting task")
}

pub fn toggle_task(id: i32, pool: &SqlitePool) -> Result<(), &'static str> {
    Task::toggle_with_id(id, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error toggling task completion")
}

pub fn delete_task(id: i32, pool: &SqlitePool) -> Result<(), &'static str> {
    Task::delete_with_id(id, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error deletign task")
}
