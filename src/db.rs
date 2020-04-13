use crate::models::{TodoList};
use  deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error>{
    let statement=client.prepare("SELECT * FROM todo_list").await.unwrap();
    let todos = client.query(&statement, &[])
        .await
        .expect("Error: Could not retrieve lists")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();
    
        Ok(todos)
}