use std::fs;
use std::fs::File;
use tar::Builder;

fn delete_folder() -> std::io::Result<()> {
    fs::remove_dir_all("folder")?;
    Ok(())
}

fn main() {
    //when I create .tar and the name
    let file = File::create("data/foo.tar").unwrap();
    let mut a = Builder::new(file);

    //folder where to keep files
    a.append_path("data").unwrap();
    a.append_file("data/file2.txt", &mut File::open("data/file3.txt").unwrap())
        .unwrap();

    //delete a folder and its contents
    if let Err(err) = delete_folder() {
        println!("Error: {:?}", err);
    }
}
