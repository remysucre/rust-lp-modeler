extern crate lp_modeler;

use lp_modeler::solvers::*;
use lp_modeler::dsl::*;

fn main() {
    let ref a = LpInteger::new("a");
    let ref b = LpInteger::new("b");

    let mut problem = LpProblem::new("Problem", LpObjective::Maximize);

    problem += 10.0 * a + 20.0 * b;

    problem += (300 * (a - b)).ge(100);
    problem += (300 * (-a + b)).le(100);
    problem += (a + b).le(10);

    problem += sum(&vec!(a,b), |&v| v * 3.0 ).equal(10.0);

    //let _ = problem.write_lp("toto.lp");

//    let solver = GurobiSolver::new();
    let solver = CbcSolver::new();

    match solver.run(&problem) {
        Ok((status, res)) => {
            println!("Status {:?}", status);
            for (name, value) in res.iter() {
                println!("value of {} = {}", name, value);
            }
        }
        Err(msg) => println!("{}", msg),
    }
}
