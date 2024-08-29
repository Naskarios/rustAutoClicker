#![allow(non_snake_case)]
#![allow(unused_parens)]
use mouse_rs::{types::Point, Mouse};
use std::time::Duration;
use std::thread;

fn main() {
    // impl user input here
    println!("Wanna click or press?");
    let clickFlag = false;
    let pressFlag = false;
    println!("wanna add your own point?");
    let mut  userPoints: Vec<Point>= vec![Point{x:100,y:100},Point{x:200,y:200},Point{x:300,y:300},Point{x:400,y:400}];
    // userPoints.pop();
    // userPoints.pop();
    // userPoints.pop();
    // userPoints.pop();
    // print!("EDW ----> {:?}",userPoints);

    
    traceAndExcecutePointPath(clickFlag, pressFlag,userPoints);


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

fn move_mouse(mouse: &Mouse, pos: Point) {
    mouse.move_to(pos.x, pos.y).expect("Unable to move mouse");
    println!("> MOVED TO POSITION {:?}", pos)
}

fn makeNewListPoints(mouse: &Mouse) -> Vec<Point> {

    // impl user input here
    println!("> How many points are we tracing");
    let count: usize = 3;
    println!("> How much delay (seconds) between each point trace");
    let traceDelay: usize = 3;
    println!(
        "> reading {:?} amount of positions with a 3 sec delay between reads",
        count
    );
    //user warning
    println!("> Staring in 3...");
    thread::sleep(Duration::from_secs(1));
    println!("> Staring in 2...");
    thread::sleep(Duration::from_secs(1));
    println!("> Staring in 1...");

    let mut newListPoints: Vec<Point> = vec![Point { x: 0, y: 0 }];

    //point capture with 3sec delay

    for i in 1..count+1 {
        newListPoints.push(mouse.get_position().unwrap());
        print!("> {:?} new point{:?}\n", i, newListPoints[i]);
        thread::sleep(Duration::from_secs(traceDelay.try_into().unwrap()));
    }
    newListPoints
}

fn traceAndExcecutePointPath(clickFlag: bool, pressFlag: bool,userPoints:Vec<Point>) {
    let mouse = Mouse::new();
    let mut ListPoints: Vec<Point>=vec![Point{x:0,y:0}];

    if(userPoints.is_empty()){

        // tracing the new positions

        println!("NO USER POINTS AVAILABLE");
        ListPoints = makeNewListPoints(&mouse);
        println!("> {:?}", ListPoints);
    }
    else {

        //using users positions

        ListPoints=userPoints;
        println!("> User points added {:?}",ListPoints);
    }

    // move to specified positions loop with a 1sec delay

    if (pressFlag == true) {
        mouse.press(&mouse_rs::types::keys::Keys::LEFT).expect("err msg");
    }
    for p in ListPoints {
        move_mouse(&mouse, p);
        if (clickFlag == true) {
           let _ = mouse.click(&mouse_rs::types::keys::Keys::LEFT);
        }
        thread::sleep(Duration::from_secs(1));
    }
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
