use crossterm::{
  cursor, event::{self , Event ,KeyCode, KeyEvent }, execute, style::{self, Print, Color, SetForegroundColor}, terminal::{disable_raw_mode, enable_raw_mode, ClearType }
}; 
use std::thread;
use crossterm::queue;

use rand::seq::SliceRandom;
use std::{
    io::{Write , stdout},
    time::{Duration, Instant}
};

mod suggestions;
use suggestions::SUGGESTIONS;

mod difficulty;
use difficulty::Difficulty;

fn get_suggestions(difficulty:Difficulty) -> &'static str {
  let mut rng = rand::thread_rng();
    let filtered_suggestions: Vec<&'static str> = SUGGESTIONS.iter()
        .filter(|&s| match difficulty {
            Difficulty::Easy => s.len() < 30,
            Difficulty::Medium => s.len() >= 30 && s.len() <= 60,
            Difficulty::Hard => s.len() > 60,
        })
        .copied()
        .collect();
    
    filtered_suggestions.choose(&mut rng).unwrap_or(&SUGGESTIONS[0])
}

fn select_difficulty(stdout: &mut std::io::Stdout) -> std::io::Result<Difficulty> {
    execute!(
        stdout,
        crossterm::terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        SetForegroundColor(Color::Yellow),
        Print("Select Difficulty:\n\n"),
        SetForegroundColor(Color::Green),
        Print("1. Easy    - Short snippets\n"),
        SetForegroundColor(Color::Yellow),
        Print("2. Medium  - Medium length code\n"),
        SetForegroundColor(Color::Red),
        Print("3. Hard    - Complex snippets\n\n"),
        SetForegroundColor(Color::Reset),
        Print("Press 1, 2, or 3 to select: ")
    )?;

        enable_raw_mode()?;
    let difficulty = loop {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('1') => break Difficulty::Easy,
                KeyCode::Char('2') => break Difficulty::Medium,
                KeyCode::Char('3') => break Difficulty::Hard,
                _ => continue,
            }
        }
    };
    disable_raw_mode()?;    Ok(difficulty)
  }

//this the logic for the lgbtq lights
fn rgb_wave(pos: f32) -> Color {
    let r = ((std::f32::consts::PI * 2.0 * pos).sin() * 127.0 + 128.0) as u8;
    let g = ((std::f32::consts::PI * 2.0 * pos + 2.0).sin() * 127.0 + 128.0) as u8;
    let b = ((std::f32::consts::PI * 2.0 * pos + 4.0).sin() * 127.0 + 128.0) as u8;
    Color::Rgb { r, g, b }
}

fn print_animated_header(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
 let header_lines = [
        "██████╗ ██╗   ██╗███████╗████████╗██╗   ██╗      ████████╗██╗   ██╗██████╗ ███████╗",
        "██╔══██╗██║   ██║██╔════╝╚══██╔══╝╚██╗ ██╔╝      ╚══██╔══╝╚██╗ ██╔╝██╔══██╗██╔════╝",
        "██████╔╝██║   ██║███████╗   ██║    ╚████╔╝ █████╗   ██║    ╚████╔╝ ██████╔╝█████╗  ",
        "██╔══██╗██║   ██║╚════██║   ██║     ╚██╔╝  ╚════╝   ██║     ╚██╔╝  ██╔═══╝ ██╔══╝  ",
        "██║  ██║╚██████╔╝███████║   ██║      ██║            ██║      ██║   ██║     ███████╗",
        "╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝      ╚═╝            ╚═╝      ╚═╝   ╚═╝     ╚══════╝"
    ];

    for offset in 0..30 {
        queue!(stdout, cursor::MoveTo(0, 0))?;
        
        for (line_idx, line) in header_lines.iter().enumerate() {
            let pos = (line_idx as f32 + offset as f32) / 6.0; 
            queue!(stdout, SetForegroundColor(rgb_wave(pos)))?;
            
            queue!(stdout, Print(line), Print("\n"))?;
        }
        
        stdout.flush()?;
        thread::sleep(std::time::Duration::from_millis(70));
    }


    queue!(
        stdout,
        SetForegroundColor(Color::Magenta),
        Print("\nWelcome to RUSTY-TYPES: Test your Rust coding speed and accuracy\n"),
        Print("Press 'Esc' to exit anytime. :)\n\n")
    )?;

    Ok(())
}

fn main() -> std::io::Result<()> {
  let mut stdout = stdout();
  

 execute!(
        stdout,
        crossterm::terminal::Clear(ClearType::All)
    )?;

    print_animated_header(&mut stdout)?;
    
    let difficulty = select_difficulty(&mut stdout)?;
    let suggestions = get_suggestions(difficulty);

    execute!(
        stdout,
        crossterm::terminal::Clear(ClearType::All)
    )?;

    print_animated_header(&mut stdout)?;


    execute!(
        stdout,
        SetForegroundColor(Color::Reset),
        style::Print("Type the following Rust code:\n"),
        SetForegroundColor(Color::Cyan),
        style::Print("━".repeat(50)),
        style::Print(format!("\n\n{}\n\n", suggestions)),
        style::Print("━".repeat(50)),
        SetForegroundColor(Color::Reset),
        style::Print("\n\nStart typing here:\n\n> ")
    )?;

  enable_raw_mode()?;

  let mut typed = String::new();
  let mut starts = false;
  let mut start_time = Instant::now();

//while loop
//pool 500 second
//check event to see if keyboard only not mouse 
//start time 
//match 
//backspace
//esc
//unnecesary key prevention

while typed != suggestions {
if event::poll(Duration::from_millis(500))?{
    if let Event::Key(key_event) = event::read()?{
      if !starts{
        starts = true;
        start_time = Instant::now();
      }
    
      match key_event.code {
         KeyCode::Char(char) => {
          typed.push(char);
          print!("{}", char);
          stdout.flush()?;
         }
         KeyCode::Backspace => {
          if !typed.is_empty(){
          typed.pop();
          execute!(
            stdout ,
            cursor::MoveLeft(1),
            style::Print(" "),
            cursor::MoveLeft(1)
          )?;
         }}
         KeyCode::Esc => {
          disable_raw_mode()?;
          println!("\nExited");
          return Ok(());
         }
         _ => {}
      }
    }
  }
}


let time_taken = start_time.elapsed().as_secs_f64();
let char_count = suggestions.chars().count() as f64;
let minutes = time_taken / 60.0;
let cpm = char_count / minutes;  
let wpm = cpm / 5.0;  

disable_raw_mode()?;

execute!(
    stdout,
    Print("\n\n"),
    SetForegroundColor(Color::Yellow),
    Print("╔════════════ RESULTS ════════════╗\n"),
    Print("║                                 ║\n"),
    SetForegroundColor(Color::Magenta),
    Print(format!("║   Difficulty: {:<15}    ║\n", difficulty.as_str())),
    SetForegroundColor(Color::Cyan),
    Print(format!("║   Time: {:.2} seconds          ║\n", time_taken)),
    SetForegroundColor(Color::Green),
    Print(format!("║   WPM: {:.2}                  ║\n", wpm)),
    Print(format!("║   CPM: {:.2}                  ║\n", cpm)),
    SetForegroundColor(Color::Yellow),
    Print("║                                 ║\n"),
    Print("╚═════════════════════════════════╝\n"),
    SetForegroundColor(Color::Reset)
)?;


Ok(())

}