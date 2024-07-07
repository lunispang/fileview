use crossterm::{
    style::{Print}, 
    terminal::{Clear, ClearType},
    cursor::{MoveTo},
    style::Stylize,
    {ExecutableCommand}
};

use console::Term;
use arboard::Clipboard;

use std::{
    io,
    path::{
        Path,
        PathBuf
    },
    fs::read_dir,
};

fn main() -> io::Result<()> {
    let mut command_char: char; 
    let term = Term::stdout();
    let mut selected: usize = 0;
    let mut dir = std::env::current_dir()?;
    let mut clipboard = Clipboard::new().unwrap();
    loop {
        let title: &str = &dir.to_string_lossy().clone();
        clear_screen(Some(&title))?;
        let length = show_list(&dir, selected)?;
        command_char = term.read_char()?;
        match command_char {
            'q' => { break; },
            'w' | 'k' => { selected = (selected + length - 1) % length},
            'd' | 'j' => { selected = (selected + 1) % length},
            'e' | 'l' => { dir = enter_dir(dir, &mut selected)},
            'a' | 'h' => { dir = parent_dir(dir); selected = 0; }, 
            's' | '/' => { dir = search_dir(dir); selected = 0; },
            'c' | 'y' => { clipboard.set_text(dir.to_string_lossy().clone()).unwrap(); },
            _ => { continue; }
        }    
    }
    Ok(())
}

fn clear_screen(title: Option<&str>) -> io::Result<()> {
    io::stdout()
       .execute(Clear(ClearType::All))?
       .execute(MoveTo(0, 0))?;
    if let Some(title) = title {
        io::stdout()
            .execute(Print(title))?
            .execute(Print("\n\n"))?;
    }
    Ok(())
}

fn enter_dir(current_dir: PathBuf, selected: &mut usize) -> PathBuf {
    let mut dir = read_dir(current_dir.clone()).unwrap();
    let new_dir = dir.nth(*selected);
    if let Some(ok_dir) = new_dir {
        let ok_dir = ok_dir.unwrap();
        if ok_dir.path().is_dir() {
            *selected = 0;
            return ok_dir.path();
        }
    }
    return current_dir;
}

fn parent_dir(current_dir: PathBuf) -> PathBuf {
    if let Some(new_dir) = current_dir.parent() {
        return new_dir.to_path_buf();
    }
    return current_dir;
}

fn search_dir(current_dir: PathBuf) -> PathBuf {
    let dir = read_dir(current_dir.clone()).unwrap();
    let mut search_term = String::new();
    io::stdout()
        .execute(Print("enter search term:")).unwrap();
    io::stdin().read_line(&mut search_term).unwrap();
    search_term.pop();
    for obj in dir {
        let obj = obj.unwrap();
        if obj.path().is_dir() {
            let filename = obj.file_name().into_string().unwrap();
            let filename = filename.as_str();
            if filename.starts_with(&search_term.as_str()) {
                return obj.path();
            }
        }
    }
                
    return current_dir;
}

fn show_list(current_dir: &Path, selected: usize) -> std::io::Result<usize> {
    let list = read_dir(current_dir)?;
    let mut len = 0;
    let mut stdout = io::stdout();
    for (index, item) in list.enumerate().skip(match selected {10.. => selected - 10, _ => 0}) {
        let item = item?;
        let mut name: String = item.file_name().into_string().unwrap();
        if item.path().is_dir() {
            name.push('/');
        }
        let mut stylizedname = name.white();
        if index == selected {
            stylizedname = stylizedname.red();
        }
        stdout.execute(Print(stylizedname))?;
        stdout.execute(Print("\n"))?;
        len = index + 1;
    }
    Ok(len)
}
