use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone, PartialEq)]
pub enum CellType {
    Path,
    Wall,
}

pub struct Mazer;

impl Mazer {
    pub fn generate_maze(height: usize, width: usize) -> Vec<Vec<CellType>> {
        let mut maze = Vec::with_capacity(height);
        for i in 0..height {
            maze.push(Vec::with_capacity(width));
            for _ in 0..width {
                maze[i].push(CellType::Wall);
            }
        }
        Self::generate_maze_dfs(0, 0, &mut maze);
        maze
    }

    pub fn generate_solution(maze: &Vec<Vec<CellType>>) -> Option<Vec<(usize, usize)>> {
        let mut solution = Vec::with_capacity(maze.len());
        let n = maze.len();
        let m = maze[0].len();
        for i in 0..n {
            solution.push(Vec::with_capacity(m));
            for _ in 0..m {
                solution[i].push(false);
            }
        }
        Self::generate_solution_dfs(0, 0, &mut maze.clone(), &mut solution);
        if !solution[n - 1][m - 1] {
            return None;
        }
        let cells_in_path = solution.iter().flatten().filter(|&&x| x).count();
        let mut path = Vec::with_capacity(cells_in_path);

        for i in 0..solution.len() {
            for j in 0..solution[0].len() {
                if solution[i][j] {
                    path.push((i, j));
                }
            }
        }
        Some(path)
    }

    fn count_visited_neighbours(x: usize, y: usize, maze: &Vec<Vec<CellType>>) -> usize {
        let n = maze.len();
        let m = maze[0].len();
        let mut count = 0;
        let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        for k in 0..4 {
            let nx = x as i32 + dirs[k].0;
            let ny = y as i32 + dirs[k].1;
            if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 {
                continue;
            }
            if maze[nx as usize][ny as usize] == CellType::Path {
                count += 1;
            }
        }
        count
    }

    fn generate_maze_dfs(x: usize, y: usize, maze: &mut Vec<Vec<CellType>>) {
        let n = maze.len();
        let m = maze[0].len();
        if x >= n || y >= m || maze[x][y] == CellType::Path {
            return;
        }
        if Self::count_visited_neighbours(x, y, maze) > 1 {
            return;
        }
        maze[x][y] = CellType::Path;
        let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut order = vec![0, 1, 2, 3];
        let mut rng = thread_rng();
        order.shuffle(&mut rng);

        for k in 0..4 {
            let nx = x as i32 + dirs[order[k]].0;
            let ny = y as i32 + dirs[order[k]].1;
            if nx >= 0 && ny >= 0 {
                Self::generate_maze_dfs(nx as usize, ny as usize, maze);
            }
        }
    }

    fn generate_solution_dfs(
        x: usize,
        y: usize,
        maze: &mut Vec<Vec<CellType>>,
        solution: &mut Vec<Vec<bool>>,
    ) -> bool {
        let n = maze.len();
        let m = maze[0].len();
        if x >= n || y >= m || maze[x][y] == CellType::Wall || solution[x][y] {
            return false;
        }
        if x == n - 1 && y == m - 1 {
            solution[x][y] = true;
            return true;
        }
        solution[x][y] = true;
        let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        for k in 0..4 {
            let nx = x as i32 + dirs[k].0;
            let ny = y as i32 + dirs[k].1;
            if nx >= 0 && ny >= 0 {
                if Self::generate_solution_dfs(nx as usize, ny as usize, maze, solution) {
                    return true;
                }
            }
        }
        solution[x][y] = false;
        return false;
    }
}
