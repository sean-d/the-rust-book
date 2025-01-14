use rand::Rng;
use std::cmp::Ordering;
use std::io;

const FREE_SPACE_DISPLAY: &str = "_";
const COURSE_LEN: usize = 32;
const OBSTACLE_1: &str = "💀";
const OBSTACLE_2: &str = "🥷";
const OBSTACLE_3: &str = "🌋";
const OBSTACLE_4: &str = "🔥";
static OBSTACLES: [&str; 4] = [OBSTACLE_1, OBSTACLE_2, OBSTACLE_3, OBSTACLE_4];

#[derive(Debug)]
struct PlayerTurn {
    roll: usize,
    current_place: usize,
}

enum NextPlace {
    Place(usize),
    GameWon,
}

/*Space represents a space on the board.
It will either be an Obstacle or a FreeSpace.

Obstacle holds a usize which reprsents the penalty the player gets if landing there
*/
#[derive(Clone, Copy, Debug)]
enum Space {
    Obstacle(usize),
    FreeSpace,
}
fn main() {
    // create the course which is a vector of Space.
    let course = make_course(COURSE_LEN);
    /*
    the players place in the course. we start at 0 but also start with the first_roll as true.

    the first roll means the player may not be starting on the first place on the board...this will get toggled false once the first move is done.
    */
    let mut current_place: usize = 0;
    let mut first_roll = true;

    /*
    pass in length of course to avoid printing out the current place of the player
    since the len is greater than the total spaces, when the board gets printed initially, there is no indication where the player is.
    */
    println!("🎬 {:?}", make_board_display(&course.len(), &course));

    // main game loop
    'game: loop {
        /* 1.
        roll the dice:

        two ranges are sent to roll_two_dice.
        two 6-sided die are rolled and saved as a tuple
        that tuple is turned into an array so we can iterate over it, and then take the sum of the two and save that as the roll.

        so if you were to roll a 5 and a 6, your total roll is an 11.
         */
        let roll = roll_two_dice(1..7, 1..7);
        let roll = [roll.0, roll.1].iter().sum();

        /* 2.

         find the next place in the course for the player:

         first with let turn...
         we create an instance of PlayerTurn that contains the roll that was just made and the location of the player...on first go, current_place will be 0.

         then we output the results of the turn with the two print statements

         with let next_place...
         we determine if the next space being moved to is a winning space or not.
         if find_next_place returns NextPlace::GameWon, we break out of the game loop. otherwise we set next_place to be the number returned from find_next_place

        */
        let turn = PlayerTurn {
            roll: roll,
            current_place: current_place,
        };

        println!("⏭ you are currently at spot {}.", turn.current_place);
        println!("you move up {} spaces", turn.roll);

        let next_place = match find_next_place(&turn, COURSE_LEN, first_roll) {
            NextPlace::GameWon => {
                println!("🏆 Finished the course!");
                break 'game;
            }
            NextPlace::Place(p) => p,
        };

        /* 3.
        move the player to the next space

        current_place becomes the returned result of move_player_to_next_place. the description of what that does can be found above the function declaration. it also calls hit_obstacle_next_place and that info is above that function declaration.
         */
        current_place = move_player_to_next_place(&course, next_place);

        /* 4.
        print out the board

        the updated board is created by passing the current_place and the board into Make_board_display

        we print the returned board

        since the first roll has happened in order to reach this point, we ensure it's faulse
        every move going forward.

        */
        let updated_board_view = make_board_display(&current_place, &course);
        println!("🎯 {:?}", updated_board_view);
        first_roll = false;
        println!("======");
    }
}

/* make_board_display takes in the player place, the course, and returns a vector of strings.

because we are returning a vec of strings to represent the board to the player, we create one.

we create an iterator and enumerate over it to get the index and current Space (i: index, spot: space)

we check what the type spot is (Space::Obstacle or Space::FreeSpace) and return the string repsentation of that which gets assigned to item.
-if space is an Obstacle, we randomly assign an Obstacle from the OBSTACLES array

if the location of the player matches the idex value, a + is prepended to the space...
this will make the player visible on the board.
*/
fn make_board_display(place: &usize, course: &Vec<Space>) -> Vec<String> {
    let mut board: Vec<String> = vec![];
    for (i, &spot) in course.iter().enumerate() {
        let item = match spot {
            Space::Obstacle(_) => {
                let obstacle = rand::thread_rng().gen_range(0..3);
                OBSTACLES[obstacle].to_string()
            }
            Space::FreeSpace => FREE_SPACE_DISPLAY.to_string(),
        };
        if i == *place {
            let location = i + 1;
            board.push(format!("+{}", location));
        } else {
            board.push(item);
        }
    }
    board
}

/* make_course takes in what the length of the course should be and returns a vector of Spaces of that length.

the while loop:
- for the length of the course, we loop
- create a space with a randomly selected 0 or 1.
- if the space is 0, it's a FreeSpace and is pushed to the board being created
- if the space is a 1, it's an Obstacle.
-- as an obstacle, the penalty needs to be determined. That is randomly selected to be 2 or 3.
-- the newly created Obstacle is pushed to the board being created
- the course gets returned.
*/
fn make_course(course_len: usize) -> Vec<Space> {
    let mut i = 0;
    let mut course: Vec<Space> = vec![];
    while i < course_len {
        let space: i8 = rand::thread_rng().gen_range(0..=1);
        if space == 0 {
            course.push(Space::FreeSpace);
        } else {
            let penalty: usize = rand::thread_rng().gen_range(2..4);
            course.push(Space::Obstacle(penalty));
        }
        i += 1;
    }
    course
}

/* hit_obstacle_next_place takes in the current place of the player and any penalty they have.
it returns a usize number

tmp represents the current place minus the penalty.

we use match to compare tmp against 0. we use 0 to prevent out of bounds in the board vector (going below 0)

if tmp equals 0: return tmp
if tmp is less than 0: return 0
if tmp is greater than 0: return tmp
*/
fn hit_obstacle_next_place(place: usize, penalty: usize) -> usize {
    let tmp = place - penalty;
    match tmp.cmp(&0) {
        // reference to 0 because cmp requires a reference
        Ordering::Equal => tmp,
        Ordering::Less => 0,
        Ordering::Greater => tmp,
    }
}

/* roll_two_dice takes in two Range<usize> and returns a tuple of two usize.

we wait for user input...does not matter what it is...just any user input.

two random numbers are generated based on the two Range types provided.
-gen_range() will generate a number based on the range ceiling provided...exclusively.

those two numbers are returned as a tuple.
*/
fn roll_two_dice(dice1: std::ops::Range<usize>, dice2: std::ops::Range<usize>) -> (usize, usize) {
    println!("🎲🎲 Roll two dice…");

    io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");

    // returning tuple
    (
        rand::thread_rng().gen_range(dice1),
        rand::thread_rng().gen_range(dice2),
    )
}

/* find_next_place takes in the playerTurn, the course length, and if this is a first roll or not. Returns a NextPlace instance.

next_place is set: the current location of the player + the dice rolled amount

if next_place goes beyond the length of the course, the winning item gets returned.

if this is a first roll, we return where we are moving to minus one since the player starts at spot 0...
-- if you start at 0 and roll a 2...you will wind up at spot 3 [x,x,x]...by removing one: [x,x] which represents 2 as your first roll.

otherwise, return next_place (current spot + roll)
*/
fn find_next_place(turn: &PlayerTurn, course_len: usize, first_roll: bool) -> NextPlace {
    let next_place = turn.current_place + turn.roll;
    if next_place >= course_len {
        return NextPlace::GameWon;
    } else if first_roll {
        return NextPlace::Place(turn.roll - 1);
    } else {
        return NextPlace::Place(next_place);
    }
}

/* move_player_to_next_place takes in the course and the next place the player is moving to. it returns the location the player moved to

space receives the Option returned from getting the Space at the supplied next_place that was passed in

we match on this Option<&Space> and the result is assigned to space.
- if the space is an obstacle, we print that to the screen and let the player know how many spaces they are being moved back.
-- reminder: Space::Obstacle(x)..x was randomly generated to be either 2 or 3 when the space was created.
--- we are simply returning that number as the penalty for spaces to move back
-- next_place and the penalty are sent to hit_obstacle_next_place
-- the value returned from sending those to hit_obstacle_next_place is returned here

- if the space is a freespace, we let the player know and we simply return the space being moved to.

- if the match gets an Option<&Space> that is something else entirely, we return next_place.
*/
fn move_player_to_next_place(course: &Vec<Space>, next_place: usize) -> usize {
    let space = course.get(next_place);

    match space {
        Some(Space::Obstacle(v)) => {
            println!("oops! this space is obstacle. go back {} spaces", v);
            // course.get() returns a reference...so Obstacle(v), v has to be a reference too.
            // therefore, passing the value, v, to another function needs to be derefenced as seen here
            return hit_obstacle_next_place(next_place, *v);
        }
        Some(Space::FreeSpace) => {
            println!("✅ space is not obstacle.");
            return next_place;
        }
        None => next_place,
    }
}
