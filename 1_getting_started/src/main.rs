fn main() {
    println!("Hello, world!");
}

/*
    Cargo is Rust's build system and package manager.
    Starting a project with Cargo makes dependency management easier

    Make:
        cargo new hello_cargo
        cd hello_cargo

    Initializes a git repo, creates a main.rs and Cargo.toml.
    Packages of code are called crates.

    To convert and existing project to a Cargo one, just create the files
    as they are here.

    Building and Running a Cargo
    from parent dir:
        cargo build
            creates an executable file in target/debug/project_name.exe
            default is debug build.
        .\target\debug\project_name.exe - to execute

        Also creates a Cargo.lock 
            Keeps track of the exact versions of dependencies in your project

        Do both
            cargo run
        
        Check if code compiles
            cargo check - faster than cargo build
        
        Building for Release
            cargo build --release - compiles with optimizations
          
*/