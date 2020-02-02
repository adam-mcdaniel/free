use free::{Simplify, C, Error, Program};
use clap::{clap_app, crate_version,  AppSettings};
use std::{
	fs::{read_to_string, write},
	process::exit
};

enum Target {
	C, BrainFuck
}


fn main() -> Result<(), Error> {
	let matches = clap_app!(fr =>
		(version: crate_version!())
		(author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
		(about: "Compiles code written in the Free programming language")
		(@arg input: +takes_value +required "Path to free file to compile")
		(@arg output: +takes_value "Path to output file")
		// (@arg leak_check: --leakcheck "Add memory leak checks")
		(@group target => 
			(@arg c: -c --c "Compile to C")
			(@arg bf: -b --bf "Compile to SMPL/Brainfuck")
		)
	)
    .setting(AppSettings::ArgRequiredElseHelp)
    .get_matches();


	let target = if matches.is_present("bf") { Target::BrainFuck }
				 else { Target::C };

	let optimization = 10;

	// let leak_check = matches.is_present("leak_check");

	let output_file = match matches.value_of("output") {
		Some(file) => file,
		None => match target {
			Target::C => "out.c",
			Target::BrainFuck => "out.smpl",
		}
	};


	if let Some(file) = matches.value_of("input") {
		if let Ok(contents) = read_to_string(file) {
			let compiled = optimize(match Program::from(contents).compile() {
				Ok(c) => c,
				Err(e) => {
					println!("Could not compile program: {:#?}", e);
					exit(1);
				}
			}, optimization);

			let output_contents = match target {
				Target::C => C::simplify(compiled),
				Target::BrainFuck => compiled
			};

			if let Ok(_) = write(&output_file, &output_contents) {
				println!("Successfully compiled program to {}", output_file);
			}
		}
	}
	
    Ok(())
}


pub fn optimize(s: impl ToString, level: usize) -> String {
    let mut compiled = s.to_string().chars().filter(|ch| ['>', '<', ',', '.', '[', ']', '+', '-', '*', '?', '&'].contains(ch)).collect::<String>();
    let original_len = compiled.len();

    for n in 1..level+1 {
        let to = ">".repeat(n);
        let back = "<".repeat(n);

        let move1 = to.clone() + &back;
        let move2 = back + &to;
        for _ in 0..10 {       	
	        compiled = compiled.replace(&move1, "").replace(&move2, "");
        }
    }

    compiled
}