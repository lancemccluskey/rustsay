use clap::{ArgEnum, Parser};

/// Yet another cowsay program
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Rustsay {
    /// Message for the animal to say
    #[clap(required = true, min_values = 1, multiple_values = true)]
    message: Vec<String>,
    /// The animal that is speaking
    #[clap(arg_enum, short, long, default_value = "cow")]
    animal: Animal,
}

#[derive(ArgEnum, Clone, Debug)]
enum Animal {
    Cow,
    Tux,
}

fn main() {
    let args = Rustsay::parse();

    let message = args.message.join(" ");

    let messages = sub_strings(&message, 35);
    let animal = args.animal;

    let spaces = " ".repeat(37);
    let top = "_".repeat(37);
    let bottom = "-".repeat(37);
    println!(" {top}");
    if messages.len() == 1 {
        let formatted_message = format!(" {:width$} ", message, width = 35);
        println!("/{formatted_message}\\");
        println!("\\{spaces}/");
    } else {
        for i in 0..messages.len() {
            if i == 0 {
                let formatted_first_line = format!(" {:width$} ", messages[i], width = 35);
                println!("/{formatted_first_line}\\");
            } else if i == messages.len() - 1 {
                let formatted_last_line = format!(" {:width$} ", messages[i], width = 35);
                println!("\\{formatted_last_line}/");
            } else {
                let formatted_middle_line = format!(" {:width$} ", messages[i], width = 35);
                println!("|{formatted_middle_line}|");
            }
        }
    }
    println!(" {bottom}");
    match animal {
        Animal::Cow => print_cow(),
        Animal::Tux => print_tux(),
    }
}

fn print_cow() {
    println!("       \\   ^__^");
    println!("        \\  (oo)\\_______");
    println!("           (__)\\       )\\/\\");
    println!("               ||----w |");
    println!("               ||     ||");
}

fn print_tux() {
    println!("   \\");
    println!("    \\");
    println!("       .--.");
    println!("      |o_o |");
    println!("      |:_/ |");
    println!("     //   \\ \\");
    println!("    (|     | )");
    println!("   /'\\_   _/`\\");
    println!("   \\___)=(___/");
}

fn sub_strings(string: &str, sub_len: usize) -> Vec<&str> {
    // println!("original string length: {}", string.len());
    let mut subs = Vec::with_capacity(string.len() / sub_len);
    let mut iter = string.chars();
    let mut position = 0;

    // println!("initial position: {position}");
    while position < string.len() {
        let mut len = 0;

        let mut foo = sub_len;
        let nth_char = string.chars().nth(position + foo);

        if let Some(x) = nth_char {
            if x != ' ' {
                foo -= 1;
                let mut prev_char = string.chars().nth(position + foo);
                while prev_char != Some(' ') && prev_char != None {
                    foo -= 1;
                    prev_char = string.chars().nth(position + foo);
                }
            }
        }
        // println!("foo: {foo}");

        for ch in iter.by_ref().take(foo) {
            len += ch.len_utf8();
        }

        let sub_string = &string[position..position + len];
        subs.push(sub_string.trim());
        position += len;
        // println!("position: {position}");
    }
    subs
}
