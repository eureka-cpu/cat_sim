//! This program is to help Abbigator learn rust, with cats!

// The final project... Make a kitty that you have to take care of!
fn main() {
    let my_kitty = Kitty::new(KittyColor::Black);
    dbg!(&my_kitty);
    if my_kitty.is_hungry() {
        println!("Your kitty is hungy!!");
    }
    println!("your kitty's personality: {}", my_kitty.personality());
    print(&my_kitty);
}
// Function to generate ASCII art for the kitty
fn print(kitty: &Kitty) {
    let hungry_status = if kitty.is_hungry {
        "Hungry"
    } else {
        "Not Hungry"
    };
    let treat_status = if kitty.wants_treat {
        "Wants Treat"
    } else {
        "Doesn't Want Treat"
    };
    let personality = kitty.personality();

    use std::io::Write;
    use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let (bg, fg) = match kitty.color {
        KittyColor::Black => (Color::White, Color::Black),
        KittyColor::Orange => (Color::Black, Color::Rgb(255, 165, 0)),
    };

    stdout
        .set_color(ColorSpec::new().set_bg(Some(bg)).set_fg(Some(fg)))
        .unwrap();
    write!(
        &mut stdout,
        r#"
        /\_/\
       ( o.o )
        > ^ <
        "#
    )
    .unwrap();

    stdout.reset().unwrap();
    let text = format!(
        r#"
Hungry: {}
Wants Treat: {}
Personality: {}"#,
        hungry_status, treat_status, personality
    );
    write!(&mut stdout, "{}", uwuifier::uwuify_str(&text)).unwrap();
    stdout.reset().unwrap();
}
// 1. Make a struct that has some information about a kitty!
#[derive(Debug)]
pub struct Kitty {
    /// The color of your kitty!
    color: KittyColor,
    /// Whether or not your kitty is hungry!
    is_hungry: bool,
    /// Whether your kitty wants a treat!
    wants_treat: bool,
    // TODO: Add fields to the kitty struct!
}

// 2. Add ways to interact with your new kitty!
impl Kitty {
    /// Spawn a new kitty!
    fn new(color: KittyColor) -> Kitty {
        Kitty {
            color,
            is_hungry: true,
            wants_treat: true,
        }
    }
    /// Whether or not your kitty is hungry!
    fn is_hungry(&self) -> bool {
        self.is_hungry
    }

    /// Feed your hungry kitty!
    #[allow(dead_code)]
    fn food_time(&mut self) {
        self.is_hungry = false;
    }
    /// Give your kitty a treat!
    #[allow(dead_code)]
    fn give_treat(&mut self) {
        self.wants_treat = false;
    }
    /// Your kitty's personality!
    fn personality(&self) -> &str {
        self.color.personality()
    }
}

// 3. Add different kitty colors!
#[derive(Debug, PartialEq, Eq)]
pub enum KittyColor {
    Black,
    Orange,
    // TODO: Add other colors!
}

// 4. Add personality traits to your kitty based on color!
// TODO: change the return type from `&str` to a new type!
impl KittyColor {
    /// Your kitty's personality!
    fn personality(&self) -> &str {
        match self {
            KittyColor::Black => "v chill, needs lots of attention",
            KittyColor::Orange => "crazy, silly little bean",
        }
    }
}

// 5. Add tests to showcase your new kitty!
#[cfg(test)]
mod kitty_tests {
    use crate::{Kitty, KittyColor};

    // This is how to make a test!
    #[test]
    fn spawn_hungry_kitty() {
        let my_kitty = Kitty::new(KittyColor::Orange);
        dbg!(&my_kitty); // This is how to see what your kitty looks like!
        assert!(my_kitty.is_hungry()); // This is how to prove your kitty is hungry!
    }
    #[test]
    fn test_new_kitty() {
        let my_kitty = Kitty::new(KittyColor::Black);
        assert_eq!(my_kitty.color, KittyColor::Black);
        assert!(my_kitty.is_hungry);
        assert!(my_kitty.wants_treat);
    }

    #[test]
    fn test_is_hungry() {
        let my_kitty = Kitty::new(KittyColor::Black);
        assert!(my_kitty.is_hungry());
    }

    #[test]
    fn test_food_time() {
        let mut my_kitty = Kitty::new(KittyColor::Black);
        my_kitty.food_time();
        assert!(!my_kitty.is_hungry());
    }

    #[test]
    fn test_give_treat() {
        let mut my_kitty = Kitty::new(KittyColor::Black);
        my_kitty.give_treat();
        assert!(!my_kitty.wants_treat);
    }

    #[test]
    fn test_personality() {
        let my_kitty_black = Kitty::new(KittyColor::Black);
        assert_eq!(
            my_kitty_black.personality(),
            "v chill, needs lots of attention"
        );

        let my_kitty_orange = Kitty::new(KittyColor::Orange);
        assert_eq!(my_kitty_orange.personality(), "crazy, silly little bean");
    }
}
