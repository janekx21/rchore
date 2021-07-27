use crate::models::tasks::Tasks;
use anyhow::anyhow;

pub struct TasksDatabase {
    db: sled::Db,
}

#[derive(Default, serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct TasksDatabaseList(Vec<Tasks>);

#[derive(Default, serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct DefaultTaskListDatabase {
    title: String,
    id: String,
}

impl TasksDatabase {
    pub fn new() -> TasksDatabase {
        match home::home_dir() {
            Some(mut path) => {
                path.push(".r_chore");
                let db: sled::Db = sled::open(path).unwrap();
                return TasksDatabase { db: db };
            }
            None => {
                let db: sled::Db = sled::open("rchore_db").unwrap();
                return TasksDatabase { db: db };
            }
        }
    }

    pub fn insert_tasks(&self, taskslist: Vec<Tasks>) -> anyhow::Result<()> {
        let task_struct: TasksDatabaseList = TasksDatabaseList(taskslist);
        let bytes = bincode::serialize(&task_struct)?;
        self.db.insert("tasks_data", bytes)?;
        Ok(())
    }

    pub fn get_data(&self) -> anyhow::Result<Vec<Tasks>> {
        match self.db.get("tasks_data")? {
            Some(bytes) => {
                let tasks_list: TasksDatabaseList = bincode::deserialize(&bytes).unwrap();
                Ok(tasks_list.0)
            }
            None => Err(anyhow!("Error!")),
        }
    }

    pub fn insert_token(&self, token: String) -> anyhow::Result<()> {
        let bytes = bincode::serialize(&token)?;
        self.db.insert("token", bytes)?;
        Ok(())
    }

    pub fn get_token(&self) -> anyhow::Result<String> {
        match self.db.get("token")? {
            Some(bytes) => {
                let token: String = bincode::deserialize(&bytes)?;
                Ok(token)
            }
            None => Err(anyhow!("Error!")),
        }
    }

    pub fn insert_refresh_token(&self, token: String) -> anyhow::Result<()> {
        let bytes = bincode::serialize(&token)?;
        self.db.insert("r_token", bytes)?;
        Ok(())
    }

    pub fn get_refresh_token(&self) -> anyhow::Result<String> {
        match self.db.get("r_token")? {
            Some(bytes) => {
                let token: String = bincode::deserialize(&bytes)?;
                Ok(token)
            }
            None => Err(anyhow!("Error!")),
        }
    }

    pub fn insert_default_tasklist(&self, id: String, title: String) -> anyhow::Result<()> {
        let default_tasklist = DefaultTaskListDatabase {
            title: title,
            id: id,
        };
        let bytes = bincode::serialize(&default_tasklist)?;
        self.db.insert("tasklist", bytes)?;
        Ok(())
    }

    pub fn get_default_tasklist(&self) -> anyhow::Result<(String, String)> {
        match self.db.get("tasklist")? {
            Some(bytes) => {
                let default_tasklist: DefaultTaskListDatabase = bincode::deserialize(&bytes)?;
                Ok((default_tasklist.id, default_tasklist.title))
            }
            None => Err(anyhow!("Error!")),
        }
    }
}
