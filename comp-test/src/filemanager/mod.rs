use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

/* load_file
 *
 * input:   Path to read from
 * purpose: Read the contents of the file
 * output:  String of the file contents
 */
pub fn load_file(path : &String) -> Option<String> {

    let file_path = File::open(path);
    let mut file : File;
    let mut map_string = String::new();

    match file_path {
        Ok(res) => {file = res},
        Err(_) => return None
    }

    return match file.read_to_string(&mut map_string) {
        Ok(_) => {Some(map_string)},
        Err(_) => None
    }
}

 /* out_file
 *
 * input:   Path to write to, contents to write
 * purpose: Write the contents to the file
 * output:  None
 */
pub fn out_file(path : &String, contents : &String) {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    match file.write_all(contents.as_bytes()) {
        Err(_) => {
            panic!("couldn't write to file");
        },
        Ok(_) => println!("compilation successful"),
    }
}