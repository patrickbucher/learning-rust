use akshually::io::prompt_line;
use redis;

const REDIS_URL: &str = "redis://127.0.0.1/1";

fn main() {
    let client = redis::Client::open(REDIS_URL).expect("creating Redis client failed");
    let mut con = client.get_connection().expect("connection to Redis failed");

    loop {
        println!("Todo List. Enter:");
        println!("[1] to add tasks.");
        println!("[2] to show all tasks.");
        println!("[3] to remove a task.");
        println!("[4] to quit.");
        let choice: usize = prompt_line::<String>("Your choice: ")
            .expect("no choice entered")
            .parse()
            .expect("input is not a number");

        match choice {
            1 => loop {
                let task: String =
                    prompt_line("Task [blank to finish]: ").expect("no task entered");
                if task == "" {
                    break;
                }
                if let Err(e) = redis::cmd("LPUSH")
                    .arg("tasks")
                    .arg(task)
                    .query::<usize>(&mut con)
                {
                    eprintln!("{e}");
                }
            },
            2 => match get_tasks(&mut con) {
                Ok(tasks) => {
                    for (i, task) in tasks {
                        println!("[{i}]: {task}");
                    }
                }
                Err(e) => eprintln!("fetching tasks: {e}"),
            },
            3 => match get_tasks(&mut con) {
                Ok(tasks) => {
                    let tasks: Vec<(usize, String)> = tasks.into_iter().collect();
                    for (i, task) in &tasks {
                        println!("[{i}]: {task}");
                    }
                    let i: usize =
                        prompt_line::<usize>("Which task to delete? ").expect("no number entered");
                    match tasks.iter().find(|(j, _)| *j == i) {
                        Some((_, task)) => {
                            match redis::cmd("LREM")
                                .arg("tasks")
                                .arg("1")
                                .arg(task)
                                .query::<usize>(&mut con)
                            {
                                Ok(_) => println!("deleted task"),
                                Err(e) => eprintln!("deleting task: {e}"),
                            }
                        }
                        None => eprintln!("no such task '{i}'"),
                    }
                }
                Err(e) => eprintln!("fetching tasks: {e}"),
            },
            4 => break,
            _ => {
                println!("No such choice '{choice}'");
                continue;
            }
        }
    }
}

fn get_tasks(con: &mut redis::Connection) -> Result<Vec<(usize, String)>, redis::RedisError> {
    let iter: redis::Iter<String> = redis::cmd("LRANGE")
        .arg("tasks")
        .arg("0")
        .arg("-1")
        .clone()
        .iter(con)?;
    let tasks: Vec<(usize, String)> = iter.enumerate().collect();
    Ok(tasks)
}
