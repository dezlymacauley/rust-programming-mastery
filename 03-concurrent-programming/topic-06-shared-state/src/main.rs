/*
    ABOUT: Shared State

    Mutex = Mutual Exclusion Lock

    A mutex is a tool that you use when you want multiple parts of your
    program to safety share the same data. E.g. Multiple parts of the program
    reading a value, or being able to modify that value.

    Think of a mutex as a digital security guard that only allows
    one thread to access the data at a given time.

    That thread must first signal to the mutext that
    it wants to access some specifice data,
    and then the mutex will give that thread a lock.

    The lock is a data structure that is part of the mutex.
    The lock keeps track of which thread has exclusive access to the data.

    When a thread is done accessing the data,
    the lock is given back to the mutex so that other parts of the program
    can access the data.

    //_________________________________________________________________________

*/

use std::sync::Mutex;

fn main() {
    // It is common for a single letter variable like `m` to be used.
    // So guarded_data is a Mutex that holds an i32.
    let guarded_data: Mutex<i32> = Mutex::new(5);

    //_________________________________________________________________________

    // SECTION: How to read data guarded by a mutex

    // Create a temporary variable to access the data by requesting a lock
    // from the mutex
    let my_data = guarded_data.lock().unwrap();

    println!("my_data: {}", *my_data);
    // my_data: 5

    // NOTE: `println!()` automatically dereferences a value for you
    // So println!("my_data: {my_data}"); will work as well...
    // I just prefer to be explicit and consistent.

    // Remember to release the lock after each access,
    // it does not matter if you are reading or writing from the data.
    drop(my_data);

    //_________________________________________________________________________

    // SECTION: How to modify the data guarded by a mutex

    // How to read data guarded by a mutex
    let mut my_data = guarded_data.lock().unwrap();
    *my_data = 6;
    drop(my_data);
    
    //_________________________________________________________________________

    // SECTION: How to print the entire Mutex

    println!("guarded_data = {guarded_data:?}");
    // guarded_data = Mutex { data: 6, poisoned: false, .. }
    
    //_________________________________________________________________________
}
