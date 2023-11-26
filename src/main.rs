mod mazer;
use mazer::Mazer;

fn main() {
    let maze = Mazer::generate_maze(10, 10);
    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            print!(
                "{}",
                if maze[i][j] == mazer::CellType::Path {
                    "0 "
                } else {
                    "1 "
                }
            );
        }
        println!();
    }
    println!("");
    let solution = Mazer::generate_solution(&maze);
    match solution {
        Some(path) => {
            for i in 0..maze.len() {
                for j in 0..maze[0].len() {
                    if path.contains(&(i, j)) {
                        print!("+");
                    } else {
                        print!("{}", if maze[i][j] == mazer::CellType::Path { "0" } else { "1" });
                    }
                    print!(" ");
                }
                println!();
            }
        }
        None => {
            println!("No solution found!");
        }
    }
    println!("Hello, world!");
}
