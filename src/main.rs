#![allow(deprecated)]
#![allow(unused_must_use)]

use std::fs;
use std::process;
use std::io::{stdout, Write};

use id3::{Tag, Version};
use rustop::opts;

use crossterm::{
    execute,
    cursor,
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand, Result,
    event::{self, read, Event, KeyCode, KeyEvent},
    terminal::{enable_raw_mode},
};

#[allow(dead_code)]
fn todo() {
    println!("Not implemented yet!");
    process::exit(75);
}

fn main() -> Result<()> {

    let (args, _rest) = opts! {
        opt file:Option<String>, desc:"Input file";
    }.parse_or_exit();

    if let Some(ref _file) = args.file { /* * Nothing here lmao * */ } 
    else {
        stdout()
            .execute(SetForegroundColor(Color::Red))?
            .execute(Print("The File Isn't Supplied Use -f {File Name}"))?
            .execute(ResetColor)?; 

        println!("");
        process::exit(1);
    }

    if !exists(&args.file.as_ref().unwrap()) {
        stdout()
            .execute(SetForegroundColor(Color::Red))?
            .execute(Print("The Inputed File Doesn't Exist."))?
            .execute(ResetColor)?; 

        println!("");
        process::exit(1);
    }

    _main(&args.file.unwrap());
    Ok(())
}

fn _main(file: &str) -> Result<()> {

    enable_raw_mode().unwrap();
    clear();

    stdout().execute(cursor::MoveTo(0, 0));

    stdout()
        .execute(SetForegroundColor(Color::Green))?
        .execute(Print("What do you want to do with the file?\n"))?
        .execute(Print("Click the key on your keyboard\n"))?
        .execute(ResetColor)?;
        print!("");

    stdout()
        .execute(SetForegroundColor(Color::DarkYellow))?
        .execute(Print("[1] "))?
        .execute(ResetColor)?;
    
    print!("");

    stdout()
        .execute(SetForegroundColor(Color::Cyan))?
        .execute(Print("Read/Change the Title      "))?
        .execute(ResetColor)?;

    print!("");

    stdout()
        .execute(SetForegroundColor(Color::DarkYellow))?
        .execute(Print("[2] "))?
        .execute(ResetColor)?;
    
    print!("");

    stdout()
        .execute(SetForegroundColor(Color::Cyan))?
        .execute(Print("Read/Change the Artist\n"))?
        .execute(ResetColor)?;

    print!("");

    stdout()
        .execute(SetForegroundColor(Color::DarkYellow))?
        .execute(Print("[3] "))?
        .execute(ResetColor)?;
    
    print!("");

    stdout()
        .execute(SetForegroundColor(Color::Cyan))?
        .execute(Print("Read/Change the Album      "))?
        .execute(ResetColor)?;

    print!("");

    stdout()
        .execute(SetForegroundColor(Color::DarkYellow))?
        .execute(Print("[4] "))?
        .execute(ResetColor)?;
    
    print!("");

    stdout()
        .execute(SetForegroundColor(Color::Cyan))?
        .execute(Print("Read/Change the Genre\n"))?
        .execute(ResetColor)?;

        
    stdout()
        .execute(SetForegroundColor(Color::DarkYellow))?
        .execute(Print("[5] "))?
        .execute(ResetColor)?;
    
    print!("");

    stdout()
        .execute(SetForegroundColor(Color::Cyan))?
        .execute(Print("Read/Change the Year\n"))?
        .execute(ResetColor)?;

    stdout()
        .execute(SetForegroundColor(Color::DarkYellow))?
        .execute(Print("[q] "))?
        .execute(ResetColor)?;
    
    print!("");

    stdout()
        .execute(SetForegroundColor(Color::Cyan))?
        .execute(Print("Exit\n"))?
        .execute(ResetColor)?;

    print!("");

    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => exit(),

            Event::Key(KeyEvent {
                code: KeyCode::Char('1'),
                ..
            }) => {title(file); break Ok(())},

            Event::Key(KeyEvent {
                code: KeyCode::Char('2'),
                ..
            }) => {artist(file); break Ok(())},

            Event::Key(KeyEvent {
                code: KeyCode::Char('3'),
                ..
            }) => {album(file); break Ok(())},

            Event::Key(KeyEvent {
                code: KeyCode::Char('4'),
                ..
            }) => {genre(&file); break Ok(())},
            Event::Key(KeyEvent {
                code: KeyCode::Char('5'),
                ..
            }) => {year(&file); break Ok(())},
            _ => (),
        }
    }
}

fn title(_file: &str){

    clear();
    std::thread::sleep_ms(30);

    execute!(
        stdout(),
        SetForegroundColor(Color::Green),
        Print("Do you want to Read or Change The Title\n"),
        Print("Click the key on your keyboard\n"),
        ResetColor,
    );
    
    print!("");

    execute!(
        stdout(),
        SetForegroundColor(Color::DarkYellow),
        Print("[1] "),
        ResetColor,
    );

    print!("");
    
    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print("Read     "),
        ResetColor,
    );

    print!("");

    execute!(
        stdout(),
        SetForegroundColor(Color::DarkYellow),
        Print("[2] "),
        ResetColor,
    );
    
    print!("");

    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print("Change    "),
        ResetColor,
    );

    print!("");

    execute!(
        stdout(),
        SetForegroundColor(Color::DarkYellow),
        Print("[q] "),
        ResetColor,
    );

    print!("");


    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print("quit\n"),
        ResetColor,
    );

    print!("");


    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => exit(),

            Event::Key(KeyEvent {
                code: KeyCode::Char('1'),
                ..
            }) => {_read(&_file); break},

            Event::Key(KeyEvent {
                code: KeyCode::Char('2'),
                ..
            }) => {_change(&_file); break},

            _ => ()
        }
    }

    fn _read(file: &str) -> Result<()> {
        let tag = Tag::read_from_path(file).unwrap();
        if let Some(title) = tag.title() {
            execute!(
                stdout(),
                SetForegroundColor(Color::DarkYellow),
                Print("The Title Is: "),
                ResetColor,
            );
            execute!(
                stdout(),
                SetForegroundColor(Color::Cyan),
                Print(title),
                Print("\n"),
                ResetColor,
            );
        } else {
            execute!(
                stdout(),
                SetForegroundColor(Color::DarkYellow),
                Print("This MP3 File Doesn't Have A Title"),
                ResetColor,
            );
        }
        Ok(())
    }

    fn _change(file: &str) -> Result<()> {
        let mut tag = Tag::read_from_path(file).unwrap();
        execute!(
            stdout(),
            SetForegroundColor(Color::DarkYellow),
            Print("What do you want the title to be > "),
            ResetColor,
        );

        print!("");
        std::io::stdout().flush().expect("Error, Couldnt flush io::stdout");

        let input_title = user_input().unwrap();

        if input_title.chars().count() > 60 {
            clear();
            execute!(
                stdout(),
                SetForegroundColor(Color::DarkRed),
                Print("The title Can't be logner than 60 chars"),
                ResetColor,
            );
            print!("");
            _change(&file);
        }
        tag.set_title(input_title);

        tag.write_to_path(file, Version::Id3v24);

        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            Print("Success\n"),
            ResetColor,
        );
        Ok(())
    }
}

fn artist(_file: &str) {
    clear();
    std::thread::sleep_ms(30);
    execute!(
        stdout(),
        SetForegroundColor(Color::Green),
        Print("Do you want to Read or Change the Artist\n"),
        Print("Click the key on your keyboard\n"),
        ResetColor,
    );
    print!("");
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkYellow),
        Print("[1] "),
        ResetColor,
    );
    print!("");
    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print("Read     "),
        ResetColor,
    );
    print!("");
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkYellow),
        Print("[2] "),
        ResetColor,
    );
    print!("");
    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print("Change    "),
        ResetColor,
    );
    print!("");
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkYellow),
        Print("[q] "),
        ResetColor,
    );
    print!("");
    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print("quit\n"),
        ResetColor,
    );
    print!("");
    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => exit(),
            Event::Key(KeyEvent {
                code: KeyCode::Char('1'),
                ..
            }) => {_read(&_file); break},
            Event::Key(KeyEvent {
                code: KeyCode::Char('2'),
                ..
            }) => {_change(&_file); break},
            _ => ()
        }
    }
    fn _read(file: &str) -> Result<()> {
        let tag = Tag::read_from_path(file).unwrap();
        if let Some(artist) = tag.artist() {
            execute!(
                stdout(),
                SetForegroundColor(Color::DarkYellow),
                Print("The Artist Is: "),
                ResetColor,
            );
            execute!(
                stdout(),
                SetForegroundColor(Color::Cyan),
                Print(artist),
                Print("\n"),
                ResetColor,
            );
        } else {
            execute!(
                stdout(),
                SetForegroundColor(Color::DarkYellow),
                Print("This MP3 File Doesn't Have An Artist"),
                ResetColor,
            );
        }
        Ok(())
    }
    fn _change(file: &str) -> Result<()> {
        let mut tag = Tag::read_from_path(file).unwrap();
        execute!(
            stdout(),
            SetForegroundColor(Color::DarkYellow),
            Print("What do you want the artist to be > "),
            ResetColor,
        );
        print!("");
        std::io::stdout().flush().expect("Error, Couldnt flush io::stdout");
        let input_artist = user_input().unwrap();
        if input_artist.chars().count() > 60 {
            clear();
            execute!(
                stdout(),
                SetForegroundColor(Color::DarkRed),
                Print("The Artist name Can't be logner than 60 chars"),
                ResetColor,
            );
            print!("");
            _change(&file);
        }
        tag.set_artist(input_artist);
        tag.write_to_path(file, Version::Id3v24);
        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            Print("Success\n"),
            ResetColor,
        );
        Ok(())
    }
}

fn album(_file: &str) {
    clear();
    std::thread::sleep_ms(30);
    execute!(
        stdout(),
        SetForegroundColor(Color::Green),
        Print("Do you want to Read or Change the Artist\n"),
        Print("Click the key on your keyboard\n"),
        ResetColor,
    );
    print!("");
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkYellow),
        Print("[1] "),
        ResetColor,
    );
    print!("");
    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print("Read     "),
        ResetColor,
    );
    print!("");
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkYellow),
        Print("[2] "),
        ResetColor,
    );
    print!("");
    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print("Change    "),
        ResetColor,
    );
    print!("");
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkYellow),
        Print("[q] "),
        ResetColor,
    );
    print!("");
    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print("quit\n"),
        ResetColor,
    );
    print!("");
    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => exit(),
            Event::Key(KeyEvent {
                code: KeyCode::Char('1'),
                ..
            }) => {_read(&_file); break},
            Event::Key(KeyEvent {
                code: KeyCode::Char('2'),
                ..
            }) => {_change(&_file); break},
            _ => ()
        }
    }
    fn _read(file: &str) -> Result<()> {
        let tag = Tag::read_from_path(file).unwrap();
        if let Some(album) = tag.album() {
            execute!(
                stdout(),
                SetForegroundColor(Color::DarkYellow),
                Print("The Album Is: "),
                ResetColor,
            );
            execute!(
                stdout(),
                SetForegroundColor(Color::Cyan),
                Print(album),
                Print("\n"),
                ResetColor,
            );
        } else {
            execute!(
                stdout(),
                SetForegroundColor(Color::DarkYellow),
                Print("This MP3 File Doesn't Have An Album"),
                ResetColor,
            );
        }
        Ok(())
    }
    fn _change(file: &str) -> Result<()> {
        let mut tag = Tag::read_from_path(file).unwrap();
        execute!(
            stdout(),
            SetForegroundColor(Color::DarkYellow),
            Print("What do you want the Album to be > "),
            ResetColor,
        );
        print!("");
        std::io::stdout().flush().expect("Error, Couldnt flush io::stdout");
        let input_album = user_input().unwrap();
        if input_album.chars().count() > 60 {
            clear();
            execute!(
                stdout(),
                SetForegroundColor(Color::DarkRed),
                Print("The Album name Can't be logner than 60 chars"),
                ResetColor,
            );
            print!("");
            _change(&file);
        }
        tag.set_album(input_album);
        tag.write_to_path(file, Version::Id3v24);
        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            Print("Success\n"),
            ResetColor,
        );
        Ok(())
    }
}

fn genre(_file: &str) {
    clear();
    std::thread::sleep_ms(30);
    execute!(
        stdout(),
        SetForegroundColor(Color::Green),
        Print("Do you want to Read\n"),
        Print("Click the key on your keyboard\n"),
        Print("Unfortunately You Can't Change The Genre\n"),
        ResetColor,
    );
    print!("");
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkYellow),
        Print("[1] "),
        ResetColor,
    );
    print!("");
    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print("Read     "),
        ResetColor,
    );
    print!("");
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkYellow),
        Print("[q] "),
        ResetColor,
    );
    print!("");
    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print("quit\n"),
        ResetColor,
    );
    print!("");
    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => exit(),
            Event::Key(KeyEvent {
                code: KeyCode::Char('1'),
                ..
            }) => {_read(&_file); break},
            Event::Key(KeyEvent {
                code: KeyCode::Char('2'),
                ..
            }) => {_change(&_file); break},
            _ => ()
        }
    }
    fn _read(file: &str) -> Result<()> {
        let tag = Tag::read_from_path(file).unwrap();
        if let Some(genre) = tag.genre() {
            execute!(
                stdout(),
                SetForegroundColor(Color::DarkYellow),
                Print("The Genre Is: "),
                ResetColor,
            );
            execute!(
                stdout(),
                SetForegroundColor(Color::Cyan),
                Print(genre),
                Print("\n"),
                ResetColor,
            );
        } else {
            execute!(
                stdout(),
                SetForegroundColor(Color::DarkYellow),
                Print("This MP3 File Doesn't Have A Genre\n"),
                ResetColor,
            );
        }
        Ok(())
    }
    fn _change(file: &str) -> Result<()> {
        let mut tag = Tag::read_from_path(file).unwrap();
        execute!(
            stdout(),
            SetForegroundColor(Color::DarkYellow),
            Print("What do you want the genre to be > "),
            ResetColor,
        );
        print!("");
        std::io::stdout().flush().expect("Error, Couldnt flush io::stdout");
        let input_genre = user_input().unwrap();
        if input_genre.chars().count() > 60 {
            clear();
            execute!(
                stdout(),
                SetForegroundColor(Color::DarkRed),
                Print("The Genre name Can't be logner than 60 chars"),
                ResetColor,
            );
            print!("");
            _change(&file);
        }
        tag.set_genre(input_genre);
        tag.write_to_path(file, Version::Id3v24);
        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            Print("Success\n"),
            ResetColor,
        );
        Ok(())
    }
}

fn year(_file: &str) {
    
    clear();
    std::thread::sleep_ms(30);

    execute!(
        stdout(),
        SetForegroundColor(Color::Green),
        Print("Do you want to Read or Change The Year\n"),
        Print("Click the key on your keyboard\n"),
        ResetColor,
    );
    
    print!("");

    execute!(
        stdout(),
        SetForegroundColor(Color::DarkYellow),
        Print("[1] "),
        ResetColor,
    );

    print!("");
    
    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print("Read     "),
        ResetColor,
    );

    print!("");

    execute!(
        stdout(),
        SetForegroundColor(Color::DarkYellow),
        Print("[2] "),
        ResetColor,
    );
    
    print!("");

    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print("Change    "),
        ResetColor,
    );

    print!("");

    execute!(
        stdout(),
        SetForegroundColor(Color::DarkYellow),
        Print("[q] "),
        ResetColor,
    );

    print!("");


    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print("quit\n"),
        ResetColor,
    );

    print!("");


    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => exit(),

            Event::Key(KeyEvent {
                code: KeyCode::Char('1'),
                ..
            }) => {_read(&_file); break},

            Event::Key(KeyEvent {
                code: KeyCode::Char('2'),
                ..
            }) => {_change(&_file); break},

            _ => ()
        }
    }

    fn _read(file: &str) -> Result<()> {
        let tag = Tag::read_from_path(file).unwrap();
        if let Some(year) = tag.year() {
            execute!(
                stdout(),
                SetForegroundColor(Color::DarkYellow),
                Print("The Year Is: "),
                ResetColor,
            );
            execute!(
                stdout(),
                SetForegroundColor(Color::Cyan),
                Print(year),
                Print("\n"),
                ResetColor,
            );
        } else {
            execute!(
                stdout(),
                SetForegroundColor(Color::DarkYellow),
                Print("This MP3 File Doesn't Have A Year\n"),
                ResetColor,
            );
        }
        Ok(())
    }

    fn _change(file: &str) -> Result<()> {
        let mut tag = Tag::read_from_path(file).unwrap();
        execute!(
            stdout(),
            SetForegroundColor(Color::DarkYellow),
            Print("What do you want the year to be > "),
            ResetColor,
        );

        print!("");
        std::io::stdout().flush().expect("Error, Couldnt flush io::stdout");

        let input_year = user_input().unwrap();

        if input_year.chars().count() > 60 {
            clear();
            execute!(
                stdout(),
                SetForegroundColor(Color::DarkRed),
                Print("The year Can't be logner than 60 chars"),
                ResetColor,
            );
            print!("");
            _change(&file);
        }

        ////! try to convert the input_year to i32 and if it fails print an error////
        if let Ok(year) = input_year.parse::<i32>() {
            tag.set_year(year);
        } else {
            clear();
            execute!(
                stdout(),
                SetForegroundColor(Color::DarkRed),
                Print("The year must be a number\n"),
                ResetColor,
            );
            print!("");
            _change(&file);
        }

        tag.write_to_path(file, Version::Id3v24);

        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            Print("Success\n"),
            ResetColor,
        );
        Ok(())
    }
}

fn exit() {
    clear();
    process::exit(0);
}

fn exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn clear() {print!("{}[2J", 27 as char);}

fn user_input() -> Result<String> {
    let mut line = String::new();
    while let Event::Key(KeyEvent { code, .. }) = event::read()? {
        match code {
            KeyCode::Enter => {
                break;
            }
            KeyCode::Char(c) => {
                stdout().execute(Print(c));
                line.push(c);
            }
            _ => {}
        }
    }

    Ok(line)
}