use redis;
use std::collections::HashMap;
use std::env;

const REDIS_URL: &str = "redis://127.0.0.1/3";

fn main() {
    let mut args = env::args();
    let bin = args.next().unwrap();
    let usage: &str = &format!("usage: {bin} [add name serial value | html | csv]");
    let client = redis::Client::open(REDIS_URL).expect("redis client");
    let mut con = client.get_connection().expect("connection to redis");
    let cmd: &str = &args.next().expect(usage);
    match cmd {
        "add" => {
            let name = args.next().expect(usage);
            let serial = args.next().expect(usage);
            let value: f32 = args.next().expect(usage).parse().expect("value as float");
            match redis::cmd("hset")
                .arg(&serial)
                .arg("name")
                .arg(&name)
                .arg("serial")
                .arg(&serial)
                .arg("value")
                .arg(&value)
                .query::<usize>(&mut con)
            {
                Ok(i) => eprintln!("created entry {serial} with {i} fields"),
                Err(err) => eprintln!("{err}"),
            }
        }
        "csv" => {
            let entries = load_data(&mut con);
            println!("name,serial,value");
            for entry in entries {
                println!("{},{},{}", entry.0, entry.1, entry.2);
            }
        }
        "html" => {
            let entries = load_data(&mut con);
            println!("<table>");
            println!("\t<tr><th>Name</th><th>Serial Number</th><th>Value</th></tr>");
            for entry in entries {
                println!(
                    "\t<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
                    entry.0, entry.1, entry.2
                );
            }
            println!("</table>");
        }
        _ => panic!("{}", usage),
    }
}

fn load_data(con: &mut redis::Connection) -> Vec<(String, String, f32)> {
    let keys: Vec<String> = match redis::cmd("keys").arg("*").query(con) {
        Ok(keys) => keys,
        Err(err) => {
            eprintln!("keys *: {err}");
            Vec::new()
        }
    };
    let mut entries: Vec<(String, String, f32)> = Vec::new();
    for key in keys {
        let entry = redis::cmd("hgetall")
            .arg(&key)
            .query::<Vec<String>>(con)
            .expect("hash entry");
        let (keys, vals): (Vec<(usize, &String)>, Vec<(usize, &String)>) =
            entry.iter().enumerate().partition(|(i, _)| i % 2 == 0);
        let keys_vals: HashMap<String, String> = keys
            .iter()
            .zip(vals.iter())
            .map(|((_, k), (_, v))| (String::from(*k), String::from(*v)))
            .collect();
        let name = String::from(keys_vals.get("name").expect("name"));
        let serial = String::from(keys_vals.get("serial").expect("serial"));
        let value = keys_vals.get("value").expect("value");
        let value = value.parse::<f32>().expect("float");
        entries.push((name, serial, value));
    }
    entries
}
