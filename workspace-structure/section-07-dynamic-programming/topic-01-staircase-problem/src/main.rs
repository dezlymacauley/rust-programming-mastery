/*
    ABOUT: Dynamic Programming

    This is about breaking down complex challenges into simple
    and re-usable sub-problems.

    ___________________________________________________________________________

    E.g. Climbing Stairs Problem

    - There is a stair case with `n` steps.
    - `n` is a placeholder for the number of steps.

    - You are at the bottom of the staircase,
    and the goal is to reach the top of the staircase.

    - Allowed movements:
    - You are allowed to move either 1 step or 2 stps at a time.
    - You don't have to stick to 1 step all the way up,
    or 2 steps all the way up, you can use a combination but you are only
    allowed to choose one at a time.
    E.g. You can climb up 1 step, then your next move can be 1 step again,
    or 2 steps.

    - The problem to solve is: How many different ways
    of reaching the top are there?

    ___________________________________________________________________________

    E.g. A staircase with 3 steps

    - `n` is a placeholder for the number of steps.
    - So `n` = 3

    How many different ways of reaching the top are there?
    - Method 1: Climb 1 step, 3 times
    - Method 2: Climbing 1 step, then climb 2 steps
    - Method 3: Climbing 2 steps, then climb 1 step



    ___________________________________________________________________________
*/

// TODO: Create a version with and Option enum, 
// and another version with an Option enum to account for
// `number_of_steps = 0`

// This is solved with recursion
fn climb_stairs(number_of_steps: usize) -> usize {
    if number_of_steps == 1 {
        return 1;
    }

    if number_of_steps == 2 {
        return 2;
    }

    climb_stairs(number_of_steps - 1) + climb_stairs(number_of_steps - 2)
}

fn main() {
    let number_of_steps: usize = 0;
    let number_of_ways_to_reach_the_top = climb_stairs(number_of_steps);

    println!("If the staircase has {number_of_steps} steps,");
    println!("then the number of ways to reach the top is:");
    println!("{number_of_ways_to_reach_the_top}");
}
