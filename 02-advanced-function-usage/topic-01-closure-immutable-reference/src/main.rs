/*
    ABOUT: Closure - Immutable reference

    A closure is a function that can automatically use variables from
    the scope that it was declared in,
    without having to pass in those variables as parameters.

    Unlike a standard function, a closure is declared inside another function
    like `fn main()`

    //_________________________________________________________________________

    There are three ways that a closure can automatically use variables
    from the scope that is was declared in:

    Note: `T` is a placeholder for whatever variable type
    you need to work with.

    1. By reference (&T) → immutable borrow → Fn
    2. By mutable reference (&mut T) → mutable borrow → FnMut
    3. By move (ownership) (T) → takes ownership → FnOnce

    Note: For FnOnce, that closure can only be called once.

    //_________________________________________________________________________

    Closures are essential to understanding intermediate Rust topics,
    advanced Rust topics like concurrency,

    and working with external packages and frameworks which use closures.

    // The syntax is:
    // let name_of_closure = | parameters | -> return types {};
    // How to use a closure

    //_________________________________________________________________________
*/

fn main() {
    // These variable will be captured (used) by the closures below,
    // as immutable references.
    let player_score: i32 = 30;
    let style_score: i32 = 10;

    let ninja_stars: i32 = 8;
    let smoke_bombs: i32 = 12;

    //_________________________________________________________________________

    // SECTION: Example 1 (No parameters, no return types)

    let print_player_score = || {
        println!("player_score: {player_score}");
    };

    print_player_score();
    // player_score: 30

    //_________________________________________________________________________

    // SECTION: Example 2 (Single parameter, no return types)

    let print_total_score = |bonus: i32| {
        let total_score: i32 = player_score + bonus;
        println!("total_score: {total_score}");
    };

    println!("player_score: {player_score}");
    print_total_score(10);
    // player_score: 30
    // total_score: 40

    //_________________________________________________________________________

    // SECTION: Example 3 (Multiple parameters, no return types)

    let print_total_weapons = |num_swords: i32, num_guns: i32| {
        // The variable ninja_stars is captured as an immutable reference
        let total_weapons: i32 = ninja_stars + num_swords + num_guns;
        println!("total_weapons: {total_weapons}");
    };

    println!("ninja_stars: {ninja_stars}");
    print_total_weapons(2, 4);
    // ninja_stars: 8
    // total_weapons: 14

    //_________________________________________________________________________

    // SECTION: Example 4 (Single return type)

    let double_style_score = || -> i32 {
        // The variable style_score is captured as an immutable reference
        let doubled_styled_score = style_score * 2;
        return doubled_styled_score;
    };

    let variable_a: i32 = double_style_score();
    println!("variable_a: {variable_a}");
    // variable_a: 20

    // Since style_score was capturee  as an immutable reference,
    // the variable  `style_score` was not be modified.
    println!("style_score: {style_score}");
    // style_score: 10

    //_________________________________________________________________________

    // SECTION: Example 5 (Multiple return types)

    let double_ninja_weapons = || -> (i32, i32) {
        // The variables `ninja_stars` and `smoke_bombs`
        // are captured as an immutable references

        let values: (i32, i32) = (ninja_stars * 2, smoke_bombs * 2);

        return values;
    };

    let (double_ninja_stars, double_smoke_bombs) = double_ninja_weapons();

    println!("double_ninja_stars: {double_ninja_stars}");
    println!("double_smoke_bombs: {double_smoke_bombs}");
    // double_ninja_stars: 16
    // double_smoke_bombs: 24

    // Since the `ninja_stars` and `smoke_bombs` were captured as immutable
    // references, the original values were not modified by the closure.

    println!("ninja_stars: {ninja_stars}");
    println!("smoke_bombs: {smoke_bombs}");
    // ninja_stars: 8
    // smoke_bombs: 12

    //_________________________________________________________________________
}
