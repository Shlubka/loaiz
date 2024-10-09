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

fn create_incidence_matrix(graf: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let size = graf.len();
    let mut matrix = vec![vec![0u8; size * (size - 1) / 2]; size];
    let mut index = 0;

    for i in 0..size {
        for j in i + 1..size {
            if graf[i][j] == 1 {
                matrix[i][index] = 1;
                matrix[j][index] = 1;
                index += 1;
            }
        }
    }

    matrix
}

fn main() {
    print!("Enter graff size, more then 0 (one number for line and colum) > ");
    let size = read_from_keyboard().unwrap_or(10);
    let graf = create_and_fill_graph(size as usize);
    let matrix = create_incidence_matrix(&graf);
    let mut iter = 0;
    for row in &graf {
        iter += 1;
        let mut count = 0;
        for col in row {
            if *col == 1_u8 {
                count += 1;
            }
        }
        print!("{row:?} ");
        print!(" {iter} ");
        if count == 0 {
            println!("Вершина изолированна");
        } else if count == 1 {
            println!("Вершина концевая");
        } else if count == size as usize {
            println!("Доминирующая вершина");
        } else {
            println!();
        }
    }

    println!("matrix");
    for row in &matrix {
        println!("{row:?} ");
    }
}
