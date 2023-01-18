#![allow(unused)] 
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::io::Read;
use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use surrealdb::sql::{thing, Datetime, Object, Thing, Value};
use surrealdb::{Datastore, Response, Session};

mod flag;

const NEW_FILE :&str = "../data/new_file.txt";
const OLD_FILE :&str = "../data/file.txt";
const S_FLAG :&str = "../data/sflag.txt";
const R_FLAG :&str = "../data/rflag.txt";

type DB = (Datastore, Session);

#[tokio::main]
async fn main() -> Result<()> {
	let db: &DB = &(Datastore::new("memory").await?, Session::for_db("my_ns", "my_db"));
	let (ds, ses) = db;

	if flag::main(S_FLAG) == "START" {
		db_drop((ds, ses)).await?;
		let tasks = read_file(NEW_PATH);
		db_write((ds, ses), tasks).await?;
		let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(S_FLAG);
		file.write_all("STOP");
		file.write_all("\n");
	}

	if flag::main(R_FLAG) == "START" {
		db_read((ds, ses)).await?;
		let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(R_FLAG);
		file.write_all("STOP");
		file.write_all("\n");
	}	

	Ok(())
}

async fn db_write((ds, ses): &DB, tasks: Vec<&str>) -> Result<()> {
	let sql_cr = "CREATE tasklist";
	ds.execute(sql_cr, ses, None, false).await?;
	for task in tasks {
		let sql_in = "INSERT INTO tasklist VALUES $ts"
		let vars: BTreeMap<String, Value> = [("ts".into(), task.into())].into();
		ds.execute(sql_in, ses, Some(vars), true).await?;	
	}
	Ok(())	
}

async fn db_read((ds, ses): &DB) -> Result<()> {
	let sql = "SELECT * FROM tasklist";
	let ress = ds.execute(sql, ses, None, false).await?;
	let mut file = File::create(NEW_FILE)?;
	for object in into_iter_objects(ress)? {
		file.write_all(object.to_string());
		file.write_all("\n");
	}
	let path = Path::new(OLD_FILE)
	if path.exists() {
		fs::remove_file(OLD_FILE)?;
		fs::rename(NEW_FILE, OLD_FILE)?;
	}
	Ok(())
}

async fn db_drop((ds, ses): &DB) -> Result<()> {
	let sql = "DROP TABLE tasklist";
	ds.execute(sql, ses, None, false).await?;
	Ok(())	
} 

fn into_iter_objects(ress: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
	let res = ress.into_iter().next().map(|rp| rp.result).transpose()?;

	match res {
		Some(Value::Array(arr)) => {
			let it = arr.into_iter().map(|v| match v {
				Value::Object(object) => Ok(object),
				_ => Err(anyhow!("A record was not an Object")),
			});
			Ok(it)
		}
		_ => Err(anyhow!("No records found.")),
	}
}

fn read_file(filename: &str) -> Vec<&str> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read");
    let lines: Vec<String> = file_contents.split("\n")
        .map(|s: &str| s.to_string())
        .collect();
    lines
}
