use structopt::StructOpt;
use std::io;
use chrono::{DateTime, Utc};

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

struct Note {
    title: String,
    filename: String,
    date: String,
    tags: String,
    time: DateTime<Utc>,
}

impl Note {
    fn time_to_date(now: DateTime<Utc>) -> String {
        let date = now.format("%Y-%m-%d").to_string();
        date
    }

    fn time_to_filename(now: DateTime<Utc>, title: String) -> String {
        let date = now.format("%Y%m%d%H%M").to_string();
        let filename = date + "-" + &title.replace(" ", "-") + ".md";

        filename
    }
    fn new(t: String) -> Note {
        let basic_time = Utc::now();
        Note {
            time: basic_time,
            title: t.clone(),
            filename: Note::time_to_filename(basic_time, t.clone()),
            date: Note::time_to_date(basic_time),
            tags: "temp".to_string()
        }
    }

    fn print (&self) {
        println!{"NOTE DATA \n\tTitle: {} \n\tFilename: {} \n\tDate: {} ", self.title, self.filename, self.date};
    }
}

fn read_in_window () {
    let mut uinput = String::new();
    /* while uinput !=  {
        io::stdin()
            .read_line(&mut uinput)
            .expect("Failed to read line");
        println!("You typed {}", uinput);
}*/
    let now: DateTime<Utc> = Utc::now();
    let temp: Note = Note::new("hello world".to_string());

    temp.print();
}

fn writeFile() {

}

fn backLink() {

}

fn main() {
    read_in_window();
}
