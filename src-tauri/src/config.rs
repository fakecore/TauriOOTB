


//see https://github.com/mehcode/config-rs/issues/243

#[derive(serde::Deserialize,Debug)]
pub struct Config{
    demo: Demo,
    database: DataBase
}

#[derive(serde::Deserialize,Debug)]
struct Demo{
    name: String,
    dob: String

}

#[derive(serde::Deserialize,Debug)]
struct DataBase{
    enabled:bool,
    ports:Vec<i32>,
    data: Vec<Vec<String>>,
}