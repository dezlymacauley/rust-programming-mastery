/*
    ABOUT: Closure - Mutable reference

*/

fn main() {
    // These variable will be captured (used) by the closures below,
    // as mutable references
    let mut player_score: i32 = 30;

    let mut ninja_stars: i32 = 8;
    let mut smoke_bombs: i32 = 12;

    let mut style_score: i32 = 10;

    //_________________________________________________________________________

    // SECTION: Example 1 (No parameters, no return types)

    println!();
    println!("player_score: {player_score}");
    // player_score: 30

    let mut add_five_to_player_score = || {
        player_score += 5;
    };

    add_five_to_player_score();

    // Since this was a mutable reference,
    // the variable `player_score` has been modified.
    println!("player_score: {player_score}");
    // player_score: 35

    //_________________________________________________________________________

    // SECTION: Example 2 (Single parameter, no return types)

    println!();
    println!("player_score: {player_score}");
    // player_score: 35

    let mut increase_player_score = |inc_amount: i32| {
        player_score += inc_amount;
    };

    increase_player_score(20);

    println!("player_score: {player_score}");
    // player_score: 55

    //_________________________________________________________________________

    // SECTION: Example 3 (Multiple parameters, no return types)

    println!();
    println!("ninja_stars: {ninja_stars}");
    println!("smoke_bombs: {smoke_bombs}");
    // ninja_stars: 8
    // smoke_bombs: 12

    let mut increase_ninja_tools =
        |ninja_stars_inc: i32, smoke_bombs_inc: i32| {
            ninja_stars += ninja_stars_inc;
            smoke_bombs += smoke_bombs_inc;
        };

    increase_ninja_tools(4, 10);

    println!("ninja_stars: {ninja_stars}");
    println!("smoke_bombs: {smoke_bombs}");
    // ninja_stars: 12
    // smoke_bombs: 22

    //_________________________________________________________________________

    // SECTION: Example 4 (Single return type)

    println!();
    println!("style_score: {style_score}");
    // style_score: 10

    let mut double_style_score = || -> i32 {
        // The variable style_score is captured as a mutable reference
        style_score *= 2;
        return style_score;
    };

    style_score = double_style_score();
    println!("style_score: {style_score}");
    // style_score: 20

    //_________________________________________________________________________

    // SECTION: Example 5 (Multiple return types)

    println!();
    println!("ninja_stars: {ninja_stars}");
    println!("smoke_bombs: {smoke_bombs}");
    // ninja_stars: 12
    // smoke_bombs: 22

    let mut double_ninja_weapons = || -> (i32, i32) {
        // The variables `ninja_stars` and `smoke_bombs` are captured
        // as a mutable references
        ninja_stars *= 2;
        smoke_bombs *= 2;

        return (ninja_stars, smoke_bombs);
    };

    // Call the closure and destructure the returned tuple
    let (double_ninja_stars, double_smoke_bombs) = double_ninja_weapons();

    println!("double_ninja_stars: {double_ninja_stars}");
    println!("double_smoke_bombs: {double_smoke_bombs}");
    // double_ninja_stars: 24
    // double_smoke_bombs: 44

    // The original variables were also modified because the closure captured
    // them by mutable reference
    println!("ninja_stars: {ninja_stars}");
    println!("smoke_bombs: {smoke_bombs}");
    // ninja_stars: 24
    // smoke_bombs: 44

    //_________________________________________________________________________
}
