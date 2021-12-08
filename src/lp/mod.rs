pub mod tableau;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum LinearProblemError {
    #[error("Objective function is required")]
    ObjectiveRequired,
    #[error("Problem is unbound")]
    Unbound,
}

#[derive(Debug)]
pub struct LinearProblem {
    constraints: Vec<Constraint>,
    objective: Objective,
}

impl LinearProblem {
    pub fn builder<const NV: usize>() -> LinearProblemBuilder<NV> {
        assert_ne!(NV, 0);
        LinearProblemBuilder::default()
    }

    pub fn n_constraints(&self) -> usize {
        self.constraints.len()
    }
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
        self.objective = Some(Objective::new(coefs.into()));
        self
    }

    pub fn build(self) -> Result<LinearProblem, LinearProblemError> {
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
pub(crate) struct Objective {
    coefs: Vec<f64>,
    rhs: f64,
}

impl Objective {
    fn new(coefs: Vec<f64>) -> Self {
        Self { coefs, rhs: 0.0 }
    }
}
