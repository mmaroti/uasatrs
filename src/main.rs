/*
* Copyright (C) 2019-2020, Miklos Maroti
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

//! A SAT based discrete mathematics and universal algebra calculator.

pub mod binary;
pub mod boolean;
pub mod genvec;
pub mod lexer;
pub mod parser;
pub mod relation;
pub mod solver;
pub mod tensor;
mod testing;

use solver::*;
#[cfg(feature = "console_error_panic_hook")]
use std::panic;
use std::time::Instant;
use wasm_bindgen::prelude::*;

use tensor::{Shape, Solver, TensorAlg, TensorSat};

#[wasm_bindgen(start)]
pub fn uasat_init() {
    #[cfg(feature = "console_error_panic_hook")]
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

pub fn test_solver(solver_name: &str, size: usize) -> String {
    let start = Instant::now();

    let mut sol = create_solver(solver_name);
    let mut table: Vec<Literal> = Vec::with_capacity(size * size);

    // create literals
    for _ in 0..(size * size) {
        table.push(sol.add_variable());
    }

    // reflexive
    for i in 0..size {
        sol.add_clause(&[table[i * size + i]]);
    }

    // symmetric
    for i in 0..size {
        for j in 0..size {
            sol.add_clause(&[sol.negate(table[i * size + j]), table[j * size + i]]);
        }
    }

    // transitive
    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                sol.add_clause(&[
                    sol.negate(table[i * size + j]),
                    sol.negate(table[j * size + k]),
                    table[i * size + k],
                ]);
            }
        }
    }

    // find all solutions
    let mut count = 0;
    while sol.solve() {
        count += 1;
        let lits: Vec<Literal> = table
            .iter()
            .map(|lit| {
                if sol.get_value(*lit) {
                    sol.negate(*lit)
                } else {
                    *lit
                }
            })
            .collect();
        sol.add_clause(&lits);
    }

    let duration = Instant::now().duration_since(start);
    format!(
        "Test1 {} result {} in {:?}",
        sol.get_name(),
        count,
        duration
    )
}

pub fn test_solver2(solver_name: &str, size: usize) -> String {
    let start = Instant::now();

    let mut sol = Solver::new(solver_name);
    let name = sol.get_name();

    // relation
    let rel = sol.tensor_add_variable(Shape::new(vec![size, size]));

    // reflexive
    let tmp = sol.tensor_polymer(rel.clone(), Shape::new(vec![size]), &[0, 0]);
    sol.tensor_add_clause(&[tmp]);

    // symmetric
    let tmp = sol.tensor_polymer(rel.clone(), Shape::new(vec![size, size]), &[1, 0]);
    let tmp = sol.tensor_not(tmp);
    sol.tensor_add_clause(&[rel.clone(), tmp]);

    // transitive
    let r01 = sol.tensor_polymer(rel.clone(), Shape::new(vec![size, size, size]), &[0, 1]);
    let r01 = sol.tensor_not(r01);
    let r12 = sol.tensor_polymer(rel.clone(), Shape::new(vec![size, size, size]), &[1, 2]);
    let r12 = sol.tensor_not(r12);
    let r02 = sol.tensor_polymer(rel.clone(), Shape::new(vec![size, size, size]), &[0, 2]);
    sol.tensor_add_clause(&[r01, r12, r02]);

    // find all solutions
    let count = sol.tensor_find_num_models(&[rel]);

    let duration = Instant::now().duration_since(start);
    format!("Test2 {} result {} in {:?}", name, count, duration)
}

#[wasm_bindgen]
pub fn test(input: String) -> String {
    let lexer = lexer::Lexer::new(input.as_str());
    let mut output = String::new();
    for token in lexer {
        output.push_str(format!("{}\n", token).as_str());
    }
    #[cfg(feature = "varisat")]
    output.push_str(&test_solver("varisat", 7));
    // output.push_str(&format!("{:?}\n", parser::parse(&input)));
    output
}

fn main() {
    let size = 8;
    #[cfg(feature = "minisat")]
    println!("{}", test_solver("minisat", size));
    #[cfg(feature = "minisat")]
    println!("{}", test_solver2("minisat", size));
    #[cfg(feature = "varisat")]
    println!("{}", test_solver("varisat", size));
    #[cfg(feature = "varisat")]
    println!("{}", test_solver2("varisat", size));
    #[cfg(feature = "cryptominisat")]
    println!("{}", test_solver("cryptominisat", size));
    #[cfg(feature = "cryptominisat")]
    println!("{}", test_solver2("cryptominisat", size));
    #[cfg(feature = "batsat")]
    println!("{}", test_solver("batsat", size));
    #[cfg(feature = "batsat")]
    println!("{}", test_solver2("batsat", size));
    #[cfg(feature = "splr")]
    println!("{}", test_solver("splr", size));
    #[cfg(feature = "splr")]
    println!("{}", test_solver2("splr", size));
    #[cfg(feature = "cadical")]
    println!("{}", test_solver("cadical", size));
    #[cfg(feature = "cadical")]
    println!("{}", test_solver2("cadical", size));
}
