#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(clippy::bool_comparison)]
use enigo::Mouse as Mouse2;
use enigo::{
    // {Axis::Horizontal, Axis::Vertical},
    // {
    Coordinate::Abs,
    //  Coordinate::Rel},
    Direction::{Click, Press, Release},
    Enigo,
    Key,
    Keyboard,
    Settings,
};
use std::io;
use std::thread;
use std::time::Duration;

//**********************//**********************//**********************//**********************//**********************//**********************//**********************//**********************
//https://stackoverflow.com/questions/57454887/how-do-i-append-to-a-tuple

trait TupleAppend<T> {
    type ResultType;

    fn append(self, t: T) -> Self::ResultType;
}

impl<T> TupleAppend<T> for () {
    type ResultType = (T,);

    fn append(self, t: T) -> Self::ResultType {
        (t,)
    }
}

macro_rules! impl_tuple_append {
    ( () ) => {};
    ( ( $t0:ident $(, $types:ident)* ) ) => {
        impl<$t0, $($types,)* T> TupleAppend<T> for ($t0, $($types,)*) {
            // Trailing comma, just to be extra sure we are dealing
            // with a tuple and not a parenthesized type/expr.
            type ResultType = ($t0, $($types,)* T,);

            fn append(self, t: T) -> Self::ResultType {
                // Reuse the type identifiers to destructure ourselves:
                let ($t0, $($types,)*) = self;
                // Create a new tuple with the original elements, plus the new one:
                ($t0, $($types,)* t,)
            }
        }

        // Recurse for one smaller size:
        impl_tuple_append! { ($($types),*) }
    };
}

impl_tuple_append! {
    // Supports tuples up to size 10:
    (_1, _2, _3, _4, _5, _6, _7, _8, _9, _10)
}

//**********************//**********************//**********************//**********************//**********************//**********************//**********************

fn main() {
    // REDOING ALL OF THIS USING ENIGO CRATE
    //https://chatgpt.com/share/b8ee6542-52b3-4046-bedd-abf8856e9667

    let mut clickFlag = false;
    let mut pressFlag = false;
    let mut userPoints: Vec<(i32, i32, String)> = vec![];
    //mut because there will be a loop later
    let mut menuOption: i32;
    let mut excecuteDelay: u64;
    // all branches of the match must have the same output
    // what if every funtion returns an result/option/err?
    // match menu{
    //     1 =>
    // }

    // loop {
    menuOption = menu();

    if menuOption == 1 {
        (clickFlag, pressFlag, excecuteDelay) = settingParameters(clickFlag, pressFlag);
        if userPoints.is_empty() {
            userPoints = makeNewListPoints();
        }
        excecutePointPath(clickFlag, pressFlag, &userPoints, excecuteDelay);
    } else if menuOption == 2 {
        pokemonMode()
    } else if menuOption == 3 {
        (clickFlag, pressFlag, excecuteDelay) = settingParameters(clickFlag, pressFlag);
        if userPoints.is_empty() {
            userPoints = makeNewListPoints();
        }
        // println!("> How much delay (seconds) between each point trace");
        // let mut userInput = String::new();
        // // input
        // io::stdin()
        //     .read_line(&mut userInput)
        //     .expect("failed to readline");
        // excecuteDelay = userInput.trim().parse().unwrap();

        loop {
            excecutePointPath(clickFlag, pressFlag, &userPoints, excecuteDelay)
        }
    } else if menuOption == 4 {
        userPoints = importPath();
    }
    //      else {
    //         println!("invalid option");
    //         break;
    //     }
    // }
    println!("> THE END??? <");
}
// Import:
// x,y
// x1,y2

// OR
// x,y,click
// x1,y1,press
// x2,y2,move
// x3,y3,release

fn importPath() -> Vec<((i32, i32, String))> {
    // let mut input = String::new();
    // let mut temp: std::str::Split<'_, char>;
    // io::stdin().read_line(&mut input).unwrap();
    // temp = input.trim().split(' ');
    // // // user input a b
    // println!(
    //     "this is 1 {:?} this is 2 {:?} ",
    //     temp.next().unwrap(),
    //     temp.next().unwrap()
    // );
    // // this is 1 "a" this is 2 "b"
    /*
    String that have been split cannot be indexed
    use SPLITTED.next().unwrap() to take the value
    can I get the len using temp.count()???
    */

    let mut importedPath = vec![];
    let mut input = String::new();
    let givenData;
    println!("> Quick (q) or Detailed (D) import");
    io::stdin().read_line(&mut input).unwrap();

    if input.starts_with('q') || input.starts_with('Q') {
        let mut temp: std::str::Split<'_, char>;
        input.clear();
        println!("> Please give points (x,y x1,y1 x2,y2) theres a whitespace between each point:");
        io::stdin().read_line(&mut input).unwrap();
        givenData = input.trim().split(' ');
        // println!("GIVEN {:?}", givenData);
        for point in givenData {
            temp = point.split(',');
            importedPath.push((
                temp.next().unwrap().parse().unwrap(),
                temp.next().unwrap().parse().unwrap(),
                "nope".to_owned(),
            ))
        }
    }

    if input.starts_with('d') || input.starts_with('D') {
        //detailed import impl
        //later....
    }

    println!("PATH GIVEN {:?}", importedPath);
    importedPath
}

fn menu() -> (i32) {
    // impl the import option-------------------------------------------------------------

    // I got lazy thats why theres a option2 I ll fix it later
    let mut menuOption = String::new();
    let mut menuOption2: i32 = 0;

    println!("Choose an option:");
    print!(
        " 1.Simple trace
    \n 2.Pokemon mode 
    \n 3.Trace loop
    \n 4.Import?\n"
    );

    io::stdin()
        .read_line(&mut menuOption)
        .expect("failed to readline");

    if menuOption.starts_with('1') {
        menuOption2 = menuOption.trim().parse().unwrap();
    }
    if menuOption.starts_with('2') {
        menuOption2 = menuOption.trim().parse().unwrap();
    }
    if menuOption.starts_with('3') {
        menuOption2 = menuOption.trim().parse().unwrap();
    }
    if menuOption.starts_with('4') {
        menuOption2 = menuOption.trim().parse().unwrap();
    }
    menuOption2
}

fn settingParameters(mut clickFlag: bool, mut pressFlag: bool) -> (bool, bool, u64) {
    let mut userInput = String::new();

    println!("> Wanna click or press? (c/p)");
    // reading input

    io::stdin()
        .read_line(&mut userInput)
        .expect("failed to readline");

    if (userInput.starts_with('c') || userInput.starts_with('C')) {
        clickFlag = true;
    } else if userInput.starts_with('p') || userInput.starts_with('P') {
        pressFlag = true;
    } else {
        println!("> NO OPTION SPECIFIED");
        println!("> CONTINUING WITHOUT OPTIONS");
    }
    userInput.clear();
    println!("> How much delay (seconds) between each point excecution");

    // input
    io::stdin()
        .read_line(&mut userInput)
        .expect("failed to readline");
    let excecutionDelay: u64 = userInput.trim().parse().unwrap();

    (clickFlag, pressFlag, excecutionDelay)
}

fn makeNewListPoints() -> Vec<(i32, i32, String)> {
    let mut userInput = String::new();
    let mut newListPoints2: Vec<(i32, i32, String)> = vec![];

    println!("> How many points are we tracing");

    // input
    io::stdin()
        .read_line(&mut userInput)
        .expect("failed to readline");

    //input parse
    let count: usize = userInput.trim().parse().unwrap();
    userInput.clear();

    println!("> How much delay (seconds) between each point trace");

    // input
    io::stdin()
        .read_line(&mut userInput)
        .expect("failed to readline");

    //input parse

    let traceDelay: usize = userInput.trim().parse().unwrap();
    println!(
        "> reading {:?} amount of positions with a {:?} sec delay between reads",
        count, traceDelay
    );
    let mouse2: Enigo = Enigo::new(&Settings::default()).unwrap();

    //user warning
    println!("> Staring in 3...");
    thread::sleep(Duration::from_secs(1));
    println!("> Staring in 2...");
    thread::sleep(Duration::from_secs(1));
    println!("> Staring in 1...");

    //tracing the current position of the mouse with each interval having a 'traceDelay' delay
    for i in 0..count {
        newListPoints2.push(mouse2.location().unwrap().append("nope".to_owned()));

        println!("> {:?} new point{:?}", i, newListPoints2[i]);
        thread::sleep(Duration::from_secs(traceDelay.try_into().unwrap()));
    }
    newListPoints2
}

fn excecutePointPath(
    clickFlag: bool,
    pressFlag: bool,
    userPoints: &Vec<(i32, i32, String)>,
    excecuteDelay: u64,
) {
    let ListPoints: &Vec<(i32, i32, String)> = userPoints;
    let mut mouse: Enigo = Enigo::new(&Settings::default()).unwrap();

    println!("> User points added {:?}", ListPoints);

    // Excecute Path
    // move to specified positions loop with a 1sec delay

    if (pressFlag == true) {
        mouse.button(enigo::Button::Left, Press).unwrap();
    }
    for p in ListPoints {
        mouse.move_mouse(p.0, p.1, Abs).unwrap();
        if (clickFlag == true) {
            mouse.button(enigo::Button::Left, Click).unwrap();
        }
        if excecuteDelay > 0 {
            thread::sleep(Duration::from_secs(excecuteDelay));
        }
    }
    mouse.button(enigo::Button::Left, Release).unwrap();
    println!("> PATH EXCECUTED SUCCEFULLY!!!!")
}

fn pokemonMode() {
    // // alt + tab to focus on the game window
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let mut moveNum: i32;
    let mut userInput = String::new();
    enigo.key(Key::Alt, Press).unwrap();
    enigo.key(Key::Tab, Click).unwrap();
    enigo.key(Key::Alt, Release).unwrap();
    // game loop{}
    // must have a command terminal
    // that means asynch programming.....
    //big of
    pokemonRoam();

    loop {
        println!("> Which move 1-4 or 5 which is flee ,6 is break");
        // input
        io::stdin()
            .read_line(&mut userInput)
            .expect("failed to readline");
        moveNum = userInput.trim().parse().unwrap();
        userInput.clear();
        if (moveNum == 5) {
            pokemonChooseMove(5);
            pokemonChooseMove(5);
            break;
        }
        pokemonChooseMove(moveNum);
        thread::sleep(Duration::from_millis(500));

        enigo.key(Key::Alt, Press).unwrap();
        enigo.key(Key::Tab, Click).unwrap();
        enigo.key(Key::Alt, Release).unwrap();
    }
}
fn pokemonRoam() {
    let mut keyboard: Enigo = Enigo::new(&Settings::default()).unwrap();

    keyboard.key(Key::Unicode('w'), Press).unwrap();
    thread::sleep(Duration::from_millis(600));
    keyboard.key(Key::Unicode('w'), Release).unwrap();

    keyboard.key(Key::Unicode('s'), Press).unwrap();
    thread::sleep(Duration::from_millis(600));
    keyboard.key(Key::Unicode('s'), Release).unwrap();

    keyboard.key(Key::Unicode('a'), Press).unwrap();
    thread::sleep(Duration::from_millis(600));
    keyboard.key(Key::Unicode('a'), Release).unwrap();

    keyboard.key(Key::Unicode('d'), Press).unwrap();
    thread::sleep(Duration::from_millis(600));
    keyboard.key(Key::Unicode('d'), Release).unwrap();
}
fn pokemonChooseMove(moveNum: i32) {
    let mut mouse: Enigo = Enigo::new(&Settings::default()).unwrap();

    match moveNum {
        1 => mouse.move_mouse(71, 352, Abs).unwrap(),
        2 => mouse.move_mouse(169, 341, Abs).unwrap(),
        3 => mouse.move_mouse(95, 410, Abs).unwrap(),
        4 => mouse.move_mouse(161, 409, Abs).unwrap(),
        5 => mouse.move_mouse(130, 466, Abs).unwrap(),
        _ => mouse.move_mouse(130, 466, Abs).unwrap(),
    }
    mouse.button(enigo::Button::Left, Click).unwrap();
}

/*
HOW WE GON DO IT:
Goal: play pokemon

 (1308, 24) new tab
 (989, 100) menu
 1344, 97) next page
 (1899, 18) close

 POKEMON ME DESMUNE PANW ARISTERA
 > [(0, 0), (71, 352), (169, 341), (95, 410), (161, 409), (130, 466)]
moveTL
moveTR
moveBL
moveBR
cancel

1.
The plan divide the screen into squares
 where the width and height of the squares is global and a variable
test the squares by looping through them
adjust them to pokemon and then select which squares you re gonna loop through

OR alternatively
I can get the position of the mouse in the positions I want
and programm it to go there

2.
The programm must also receive commands like stop or retrace for moments
where I am not in battle


2,5.
POKEMON MODE

5 modes
1-4 are the moves
5 is roam for 3 secs


3.
Import:
x,y
x1,y2

OR
x,y,click
x1,y1,press
x2,y2,move
x3,y3,release

for the spiderman game



4.
I think thats about it


*/
