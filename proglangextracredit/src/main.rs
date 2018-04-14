use std::fs::File;
use std::env;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();  // puts arguments in vector

    let filename = parser(&args);   // assigns filename second element in vector

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();

    f.read_to_string(&mut contents)         // reads the file to a string
        .expect("something went wrong reading the file");

    let mut numbers: Vec<usize> =       // converts string into vector of ints
        contents.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

    let mut meatcounter: usize = 0;    // made a counter for each type of dish
    let mut vegcounter = 0;
    let mut orgcounter = 0;
    let mut dessertcounter = 0;
    let mut dairycounter = 0;
    let mut peanutscounter = 0;
    let mut i: usize = 1;
    let size: usize = numbers[0] * 7
    println!("total dishes {}", totaldishes);
    let mut done = false;
    numbers.push(0);      // I kept getting an index out of bounds error
    numbers.push(0);      // so I added a couple more idk if this is frowned upon
    while !done {

        meatcounter += numbers[i+1];
        vegcounter += numbers[i+2];
        orgcounter += numbers[i+3];
        dessertcounter += numbers[i+4];
        dairycounter += numbers[i+5];
        peanutscounter += numbers[i+6];
        i = i + 7;             // i goes to the first value of the next meal
        if i >= size {
            done = true;
        } else {
            done = false;
        }

    }

    println!("meatcounter {}", meatcounter); // just printed them to make sure it worked
    println!("vegcounter {}", vegcounter);
    println!("orgcounter {}", orgcounter);
    println!("dessertcounter {}", dessertcounter);
    println!("dairycounter {}", dairycounter);
    println!("peanutscounter {}", peanutscounter);
    println!("number of dishes {}", numbers[0]);

    let numdishes = numbers[0];

    if numdishes - meatcounter > 2 && numdishes - vegcounter > 2 && // check if each one satisfies
         orgcounter > 2 && numdishes - dessertcounter > 2 &&
        numdishes - dairycounter > 2 && numdishes - peanutscounter > 2 {
            println!("Satisfactory Meal");
        } else {
            println!("Unsatisfactory Meal");
        }

}


fn parser(args: &[String]) -> (&str) {
    let filename = &args[1];
    (filename)
}
