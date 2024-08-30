#![allow(non_snake_case)]
#![allow(unused_parens)]
use mouse_rs::{types::Point, Mouse};
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    // menu();
    // parameter initialization
    let mut clickFlag = false;
    let mut pressFlag = false;
    let mut userInput = String::new();
    let mut userPoints: Vec<Point> = vec![];
    // let mut userPoints: Vec<Point> = vec![
    //     Point { x: 100, y: 100 },
    //     Point { x: 200, y: 200 },
    //     Point { x: 300, y: 300 },
    //     Point { x: 400, y: 400 },
    // ];
    let mouse = Mouse::new();

    // PARAMETER SETTING *********************************************************************************************************

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
        userPoints = makeNewListPoints(&Mouse::new())
    } else {
        println!("> NO OPTION SPECIFIED");
        println!("> CONTINUING WITHOUT IMPORT");
    }

    // PARAMETER SETTING DONE****************************************************************************************************

    // io::stdin()
    //     .read_line(&mut userInput)
    //     .expect("failed to readline");

    // userPoints.pop();
    // userPoints.pop();
    // userPoints.pop();
    // userPoints.pop();
    // print!("EDW ----> {:?}",userPoints);

    traceAndExcecutePointPath(clickFlag, pressFlag, userPoints);

    // let mouse = Mouse::new();

    // let positionGiven = get_post(&mouse);
    // println!("> POSITION GIVEN {:?}", positionGiven);

    // // let listOfPoints = [(1308, 24), (989, 100), (1344, 97), (1899, 18)];
    // let newListPoints = makeNewListPoints(&mouse, 3);
    // println!("> {:?}", newListPoints);

    // // move to specified positions loop

    // // for p in listOfPoints {
    // //initializing based on the Point struct
    // // let positionFromList = Point { x: p.0, y: p.1 };
    // for p in newListPoints {
    //     move_mouse(&mouse, p);
    //     println!("> now sleep");
    //     thread::sleep(Duration::from_secs(2));
    // }
}

fn makeNewListPoints(mouse: &Mouse) -> Vec<Point> {
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

    //user warning
    println!("> Staring in 3...");
    thread::sleep(Duration::from_secs(1));
    println!("> Staring in 2...");
    thread::sleep(Duration::from_secs(1));
    println!("> Staring in 1...");

    let mut newListPoints: Vec<Point> = vec![Point { x: 0, y: 0 }];

    //point capture with 3sec delay

    for i in 1..count + 1 {
        newListPoints.push(mouse.get_position().unwrap());
        print!("> {:?} new point{:?}\n", i, newListPoints[i]);
        thread::sleep(Duration::from_secs(traceDelay.try_into().unwrap()));
    }
    newListPoints
}

fn traceAndExcecutePointPath(clickFlag: bool, pressFlag: bool, userPoints: Vec<Point>) {
    let mouse = Mouse::new();
    let ListPoints: Vec<Point>;

    if (userPoints.is_empty()) {
        // tracing the new positions

        println!("> NO USER POINTS AVAILABLE ");
        println!("> LET S MAKE ONE NOW");
        ListPoints = makeNewListPoints(&mouse);
        println!("> {:?}", ListPoints);
    } else {
        //using users positions

        ListPoints = userPoints;
        println!("> User points added {:?}", ListPoints);
    }

    // move to specified positions loop with a 1sec delay

    if (pressFlag == true) {
        mouse
            .press(&mouse_rs::types::keys::Keys::LEFT)
            .expect("err msg");
    }
    for p in ListPoints {
        let _ = mouse.move_to(p.x, p.y);
        if (clickFlag == true) {
            let _ = mouse.click(&mouse_rs::types::keys::Keys::LEFT);
        }
        thread::sleep(Duration::from_secs(1));
    }
    mouse.release(&mouse_rs::types::keys::Keys::LEFT);
    println!("> PATH EXCECUTED SUCCEFULLY!!!!")
}

/*
HOW WE GON DO IT:
Goal: play pokemon

 (1308, 24) new tab
 (989, 100) menu
 1344, 97) next page
 (1899, 18) close


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
