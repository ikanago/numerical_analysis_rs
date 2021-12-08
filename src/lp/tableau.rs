use crate::lp::{Constraint, LinearProblem, LinearProblemError, Objective};

pub struct Simplex {
    constraints: Vec<ConstraintEquation>,
    objective: Objective,
}

impl Simplex {
    pub fn new(problem: LinearProblem) -> Self {
        let n_constraints = problem.n_constraints();
        let LinearProblem {
            constraints,
            mut objective,
        } = problem;
        let constraints = constraints
            .into_iter()
            .enumerate()
            .map(|(i, mut constraint)| {
                for j in 0..n_constraints {
                    if i == j {
                        constraint.coefs.push(1.0);
                    } else {
                        constraint.coefs.push(0.0);
                    }
                }
                ConstraintEquation::new(constraint)
            })
            .collect::<Vec<_>>();
        objective.coefs.extend(vec![0.0; n_constraints].iter());

        Self {
            constraints,
            objective,
        }
    }

    pub fn solve(&mut self) -> Result<(), LinearProblemError> {
        let (pivot_col, &min_coef) = self
            .objective
            .coefs
            .iter()
            .enumerate()
            .min_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
            .expect("At least 1 coefficient in objective function exists");

        if min_coef > 0.0 {
            return Ok(());
        }

        let pivot_row = match (0..self.constraints.len())
            .map(|row| (row, &self.constraints[row].coefs[pivot_col]))
            .filter(|&(_, &x)| x > 0.0)
            .min_by(|&(i, x), &(j, y)| {
                (self.constraints[i].rhs / x)
                    .partial_cmp(&(self.constraints[j].rhs / y))
                    .unwrap()
            }) {
            Some((row, _)) => row,
            None => return Err(LinearProblemError::Unbound),
        };
        Ok(())
    }
}

struct ConstraintEquation {
    coefs: Vec<f64>,
    rhs: f64,
}

impl ConstraintEquation {
    fn new(constraint: Constraint) -> Self {
        Self {
            coefs: constraint.coefs,
            rhs: constraint.rhs,
        }
    }
}
