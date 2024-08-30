#![allow(non_snake_case)]
#![allow(unused_parens)]

use enigo::Mouse as Mouse2;
use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
    {Axis::Horizontal, Axis::Vertical},
    {Coordinate::Abs, Coordinate::Rel},
};
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    // REDOING ALL OF THIS USING ENIGO CRATE
    //https://chatgpt.com/share/b8ee6542-52b3-4046-bedd-abf8856e9667
    // menu();
    // parameter initialization

    // env_logger::init();
    let mut clickFlag = false;
    let mut pressFlag = false;
    let mut userPoints: Vec<(i32, i32)> = vec![];

    (clickFlag, pressFlag, userPoints) = settingParameters(clickFlag, pressFlag, userPoints);

    traceAndExcecutePointPath(clickFlag, pressFlag, userPoints);

    // POKEMON MODE

    // // alt + tab to focus on the game window
    // let mut enigo = Enigo::new(&Settings::default()).unwrap();
    // enigo.key(Key::Alt, Press).unwrap();
    // enigo.key(Key::Tab, Click).unwrap();
    // enigo.key(Key::Control, Release).unwrap();

    // pokemonRoam();

    // pokemonChooseMove(1);
    // thread::sleep(Duration::from_millis(500));

    // pokemonChooseMove(2);
    // thread::sleep(Duration::from_millis(500));

    // pokemonChooseMove(3);
    // thread::sleep(Duration::from_millis(500));

    // pokemonChooseMove(4);
    // thread::sleep(Duration::from_millis(500));

    // pokemonChooseMove(5);

    println!("THE END??");

}

fn settingParameters(
    mut clickFlag: bool,
    mut pressFlag: bool,
    mut userPoints: Vec<(i32, i32)>,
) -> (bool, bool, Vec<(i32, i32)>) {
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

    println!("> Wanna add your own point path?(y/n)");
    // reading input

    io::stdin()
        .read_line(&mut userInput)
        .expect("failed to readline");

    if (userInput.starts_with('y') || userInput.starts_with('Y')) {
        // userPoint setup
        userPoints = makeNewListPoints();
    } else {
        println!("> NO OPTION SPECIFIED");
        println!("> CONTINUING WITHOUT IMPORT");
    }
    (clickFlag, pressFlag, userPoints)
}

fn makeNewListPoints() -> Vec<(i32, i32)> {
    let mut userInput = String::new();

    println!("> How many points are we tracing");

    // input
    io::stdin()
        .read_line(&mut userInput)
        .expect("failed to readline");

    //input parse
    let count: usize = userInput.trim().parse().unwrap();
    // println!("edw count  {} -> {:?}", count, count);
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
    let mut mouse2: Enigo = Enigo::new(&Settings::default()).unwrap();

    //user warning
    println!("> Staring in 3...");
    thread::sleep(Duration::from_secs(1));
    println!("> Staring in 2...");
    thread::sleep(Duration::from_secs(1));
    println!("> Staring in 1...");

    // let mut newListPoints: Vec<Point> = vec![Point { x: 0, y: 0 }];
    let mut newListPoints2: Vec<(i32, i32)> = vec![(0, 0)];
    //point capture with 3sec delay

    for i in 1..count + 1 {
        // newListPoints.push(mouse.get_position().unwrap());
        newListPoints2.push(mouse2.location().unwrap());

        print!("> {:?} new point{:?}\n", i, newListPoints2[i]);
        thread::sleep(Duration::from_secs(traceDelay.try_into().unwrap()));
    }
    newListPoints2
}

fn traceAndExcecutePointPath(clickFlag: bool, pressFlag: bool, userPoints: Vec<(i32, i32)>) {
    let ListPoints: Vec<(i32, i32)>;
    let mut mouse: Enigo = Enigo::new(&Settings::default()).unwrap();

    // Trac Path
    if (userPoints.is_empty()) {
        println!("> NO USER POINTS AVAILABLE ");
        println!("> LET S MAKE ONE NOW");
        ListPoints = makeNewListPoints();
        println!("> {:?}", ListPoints);
    } else {
        //using users positions

        ListPoints = userPoints;
        println!("> User points added {:?}", ListPoints);
    }

    // Excecute Path
    // move to specified positions loop with a 1sec delay

    if (pressFlag == true) {
        let _ = mouse.button(enigo::Button::Left, Press).unwrap();
    }
    for p in ListPoints {
        let _ = mouse.move_mouse(p.0, p.1, Abs).unwrap();
        if (clickFlag == true) {
            let _ = mouse.button(enigo::Button::Left, Click).unwrap();
        }
        thread::sleep(Duration::from_secs(1));
    }
    let _ = mouse.button(enigo::Button::Left, Release).unwrap();
    println!("> PATH EXCECUTED SUCCEFULLY!!!!")
}
fn pokemonRoam() {
    let mut keyboard: Enigo = Enigo::new(&Settings::default()).unwrap();

    keyboard.key(Key::Unicode(('w')), Press).unwrap();
    thread::sleep(Duration::from_millis(500));
    keyboard.key(Key::Unicode(('w')), Release).unwrap();

    keyboard.key(Key::Unicode(('s')), Press).unwrap();
    thread::sleep(Duration::from_millis(500));
    keyboard.key(Key::Unicode(('s')), Release).unwrap();

    keyboard.key(Key::Unicode(('s')), Press).unwrap();
    thread::sleep(Duration::from_millis(500));
    keyboard.key(Key::Unicode(('s')), Release).unwrap();

    keyboard.key(Key::Unicode(('a')), Press).unwrap();
    thread::sleep(Duration::from_millis(500));
    keyboard.key(Key::Unicode(('a')), Release).unwrap();

    keyboard.key(Key::Unicode(('d')), Press).unwrap();
    thread::sleep(Duration::from_millis(500));
    keyboard.key(Key::Unicode(('d')), Release).unwrap();
}
fn pokemonChooseMove(moveNum: i32) {
    let mut mouse: Enigo = Enigo::new(&Settings::default()).unwrap();

    // mouse.move_mouse(100, 100, Abs);
    // thread::sleep(Duration::from_secs(1));
    // mouse.move_mouse(200, 200, Abs);
    // thread::sleep(Duration::from_secs(1));
    // mouse.move_mouse(300, 300, Abs);
    match moveNum {
        1 => mouse.move_mouse(71, 352, Abs).unwrap(),
        2 => mouse.move_mouse(169, 341, Abs).unwrap(),
        3 => mouse.move_mouse(95, 410, Abs).unwrap(),
        4 => mouse.move_mouse(161, 409, Abs).unwrap(),
        5 => mouse.move_mouse(130, 466, Abs).unwrap(),
        _ => mouse.move_mouse(130, 466, Abs).unwrap(),
    }
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
I think thats about it

Methods
    click
    get_position
    move_to
    new
    press
    release
    scroll
    wheel
*/
