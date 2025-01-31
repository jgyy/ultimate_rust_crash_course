#![allow(unused_mut, unused_variables)]

fn main() {
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    inspect(&arg);

    change(&mut arg);
    println!("I have many {}", arg);

    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
}

fn inspect(s: &String) {
	if s.ends_with("s") {
		println!("{} is plural", s);
	} else {
		println!("{} is singular", s);
	}
}

fn change(s: &mut String) {
	if !s.ends_with("s") {
		s.push_str("s");
	}
}

fn eat(s: String) -> bool {
	s.starts_with("b") && s.contains("a")
}

fn bedazzle(material: &mut String) {
    *material = "sparkly".to_string();
}
