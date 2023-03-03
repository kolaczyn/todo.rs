use crate::file_utils::read_todos_from_file;

pub fn read_command() -> () {
    let todo = read_todos_from_file();
    match todo {
        Ok(x) => print!("{:?}", x),
        Err(_) => (),
    }
}
