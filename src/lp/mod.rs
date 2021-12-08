use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum LinearProblemError {
    #[error("Objective function is required")]
    ObjectiveRequired,
}

#[derive(Debug)]
pub struct LinearProblem<const NV: usize> {
    constraints: Vec<Constraint>,
    objective: Objective,
}

#[derive(Debug, Default)]
pub struct LinearProblemBuilder<const NV: usize> {
    constraints: Vec<Constraint>,
    objective: Option<Objective>,
}

impl<const NV: usize> LinearProblemBuilder<NV> {
    pub fn constraint(
        mut self,
        coefs: [f64; NV],
        kind: ConstraintKind,
        rhs: f64,
    ) -> LinearProblemBuilder<NV> {
        let constraint = Constraint {
            coefs: coefs.into(),
            kind,
            rhs,
        };
        self.constraints.push(constraint);
        self
    }

    pub fn objective(mut self, coefs: [f64; NV]) -> LinearProblemBuilder<NV> {
        self.objective = Some(Objective {
            coefs: coefs.into(),
        });
        self
    }

    pub fn build(self) -> Result<LinearProblem<NV>, LinearProblemError> {
        match self.objective {
            Some(objective) => Ok(LinearProblem {
                constraints: self.constraints,
                objective,
            }),
            None => Err(LinearProblemError::ObjectiveRequired),
        }
    }
}

#[derive(Debug)]
struct Constraint {
    coefs: Vec<f64>,
    kind: ConstraintKind,
    rhs: f64,
}

#[derive(Debug)]
pub enum ConstraintKind {
    Less,
}

#[derive(Debug)]
struct Objective {
    coefs: Vec<f64>,
}
