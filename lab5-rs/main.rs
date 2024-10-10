use rand::Rng;
use std::io::{self, Write};

fn read_from_keyboard() -> Option<u32> {
    io::stdout().flush().expect("flush error");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    if let Ok(i) = trimmed.parse::<u32>() {
        Some(i)
    } else {
        println!("\x1b[31m!!!No number in input, установлено значение по умолчанию!!!\x1b[0m");
        None
    }
}

fn create_and_fill_graph(size: usize) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut graf = vec![vec![0u8; size]; size];
    let mut graf_size = 0;

    for i in 0..size {
        for j in i..size {
            if i == j {
                continue;
            }
            graf[i][j] = rng.gen::<u8>() % 2;
            graf[j][i] = graf[i][j];
            if graf[i][j] == 1 {
                graf_size += 1;
            }
        }
    }
    println!("Размер графа == {graf_size}");

    graf
}

fn create_incidence_matrix(adjacency_matrix: &Vec<Vec<u8>>) -> Vec<Vec<i8>> {
    let size = adjacency_matrix.len();
    let mut incidence_matrix = vec![vec![0i8; size * (size - 1) / 2]; size];
    let mut edge_index = 0;

    for i in 0..size {
        for j in i + 1..size {
            if adjacency_matrix[i][j] == 1 {
                incidence_matrix[i][edge_index] = 1;
                incidence_matrix[j][edge_index] = 1;
                edge_index += 1;
            }
        }
    }

    incidence_matrix
}

fn determine_graph_size(incidence_matrix: &Vec<Vec<i8>>) -> usize {
    let mut size = 0;
    for row in incidence_matrix {
        for &value in row {
            if value == 1 {
                size += 1;
                break;
            }
        }
    }
    size
}

fn find_isolated_terminal_dominant_vertices(
    adjacency_matrix: &Vec<Vec<u8>>,
) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let size = adjacency_matrix.len();
    let mut isolated = Vec::new();
    let mut terminal = Vec::new();
    let mut dominant = Vec::new();

    for i in 0..size {
        let mut count = 0;
        for j in 0..size {
            if adjacency_matrix[i][j] == 1 {
                count += 1;
            }
        }
        if count == 0 {
            isolated.push(i);
        } else if count == 1 {
            terminal.push(i);
        } else if count == size - 1 {
            dominant.push(i);
        }
    }

    (isolated, terminal, dominant)
}

fn main() {
    print!("Enter graff size, more then 0 (one number for line and colum) > ");
    let size = read_from_keyboard().unwrap_or(10);
    let graf = create_and_fill_graph(size as usize);
    let incidence_matrix = create_incidence_matrix(&graf);

    println!("Граф");
    for row in &graf {
        println!("{:?}", row);
    }

    println!("Матрица инцидентности:");
    for row in &incidence_matrix {
        println!("{:?}", row);
    }

    let graph_size = determine_graph_size(&incidence_matrix);
    println!("Размер графа (по матрице инцидентности) == {graph_size}");

    let (isolated, terminal, dominant) = find_isolated_terminal_dominant_vertices(&graf);

    println!("Изолированные вершины: {:?}", isolated);
    println!("Концевые вершины: {:?}", terminal);
    println!("Доминирующие вершины: {:?}", dominant);
}
