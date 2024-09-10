use rand::Rng;
use std::io;
use std::io::Write;

#[derive(Debug)]
struct Student {
    famil: String,
    name: String,
    facult: String,
    nomzach: i32,
}

trait SearchCriterion {
    fn matches(&self, student: &Student) -> bool;
}

macro_rules! define_search_criterion {
    ($name:ident, $field:ident, $type:ty) => {
        struct $name {
            $field: $type,
        }

        impl SearchCriterion for $name {
            fn matches(&self, student: &Student) -> bool {
                student.$field == self.$field
            }
        }
    };
}

define_search_criterion!(SearchByName, name, String);
define_search_criterion!(SearchByFamil, famil, String);
define_search_criterion!(SearchByFacult, facult, String);
define_search_criterion!(SearchByNumber, nomzach, i32);

fn search<T: SearchCriterion>(criterion: T, students: &[Student]) {
    for student in students {
        if criterion.matches(student) {
            println!("Студент {} {} обучается на факультете {}, номер зачётной книжки {}", student.famil, student.name, student.facult, student.nomzach);
            return;
        }
    }
    println!("Студент не найден.");
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut size_mass_col = String::new();
    let mut size_mass_row = String::new();

    print!("Введите кол-во столбцов и через пробел кол-во строк > ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut size_mass_col).unwrap();
    io::stdin().read_line(&mut size_mass_row).unwrap();

    let size_mass_col: usize = size_mass_col.trim().parse().unwrap();
    let size_mass_row: usize = size_mass_row.trim().parse().unwrap();

    let mut mass = vec![vec![0; size_mass_row]; size_mass_col];
    let mut max_val = -40;
    let mut min_val = 100;

    for i in 0..size_mass_col {
        for j in 0..size_mass_row {
            mass[i][j] = rng.gen_range(-40..100);
            print!("{} ", mass[i][j]);
            if mass[i][j] > max_val {
                max_val = mass[i][j];
            }
            if mass[i][j] < min_val {
                min_val = mass[i][j];
            }
        }
        println!();
    }

    println!("\nразница = {}", max_val - min_val);

    let mut students = Vec::new();
    loop {
        let mut name = String::new();
        print!("Введите имя студента > ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim().to_string();

        if name == "*" {
            break;
        }

        let mut famil = String::new();
        print!("Введите фамилию студента > ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut famil).unwrap();
        let famil = famil.trim().to_string();

        let mut facult = String::new();
        print!("Введите название факультета студента {} {} > ", famil, name);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut facult).unwrap();
        let facult = facult.trim().to_string();

        let mut nomzach = String::new();
        print!("Введите номер зачётной книжки студента {} {} > ", famil, name);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut nomzach).unwrap();
        let nomzach: i32 = nomzach.trim().parse().unwrap();

        let student = Student {
            famil,
            name,
            facult,
            nomzach,
        };

        students.push(student);

        println!("Студент {} {} обучается на факультете {}, номер зачётной книжки {}", famil, name, facult, nomzach);
    }

    print!("По какому параметру ищем?\n1 - имя\n2 - фамилия\n3 - название факультета\n4 - номер зачётной книжки\n> ");
    io::stdout().flush().unwrap();
    let mut search = String::new();
    io::stdin().read_line(&mut search).unwrap();
    let search: i32 = search.trim().parse().unwrap();

    match search {
        1 => {
            let mut name_stud = String::new();
            print!("Введите имя > ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut name_stud).unwrap();
            let name_stud = name_stud.trim().to_string();
            search(SearchByName { name: name_stud }, &students);
        }
        2 => {
            let mut sname_stud = String::new();
            print!("Введите фамилию > ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut sname_stud).unwrap();
            let sname_stud = sname_stud.trim().to_string();
            search(SearchByFamil { famil: sname_stud }, &students);
        }
        3 => {
            let mut facultet = String::new();
            print!("Введите название факультета > ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut facultet).unwrap();
            let facultet = facultet.trim().to_string();
            search(SearchByFacult { facult: facultet }, &students);
        }
        4 => {
            let mut number = String::new();
            print!("Введите номер книжки > ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut number).unwrap();
            let number: i32 = number.trim().parse().unwrap();
            search(SearchByNumber { nomzach: number }, &students);
        }
        _ => println!("Неправильный ввод"),
    }
}
