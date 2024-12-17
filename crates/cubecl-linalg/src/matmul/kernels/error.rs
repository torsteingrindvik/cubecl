use cubecl_core::ir::Elem;
use std::fmt::Debug;

pub enum MatmulLaunchError {
    Unavailable(MatmulAvailabilityError),
    InvalidProblem(MatmulInvalidProblem),
}

pub enum MatmulAvailabilityError {
    PlaneOperationsUnavailable,
    TypesUnavailable {
        input: Elem,
        output: Elem,
    },
    CmmaInstructionUnavailable {
        input: Elem,
        output: Elem,
        m: u32,
        n: u32,
        k: u32,
    },
}

pub enum MatmulInvalidProblem {
    ExceededMSize { m: u32, max_m: u32 },
    ExceededNSize { n: u32, max_n: u32 },
    ExceededBatchSize { b: u32, max_b: u32 },
    InvalidLineSizeLhs { size: u32, line_size: u8 },
    InvalidLineSizeRhs { size: u32, line_size: u8 },
    InvalidLineSizeOut { size: u32, line_size: u8 },
}

impl From<MatmulInvalidProblem> for MatmulLaunchError {
    fn from(value: MatmulInvalidProblem) -> Self {
        Self::InvalidProblem(value)
    }
}

impl From<MatmulAvailabilityError> for MatmulLaunchError {
    fn from(value: MatmulAvailabilityError) -> Self {
        Self::Unavailable(value)
    }
}

impl Debug for MatmulLaunchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatmulLaunchError::Unavailable(err) => {
                writeln!(
                    f,
                    "Unable to launch matmul because a required feature is unavailable: {:?}",
                    err
                )
            }
            MatmulLaunchError::InvalidProblem(err) => {
                writeln!(
                    f,
                    "Unable to launch matmul because the problem isn't correctly defined: {:?}",
                    err
                )
            }
        }
    }
}

impl Debug for MatmulInvalidProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatmulInvalidProblem::ExceededMSize { m, max_m } => write!(
                f,
                "Problem has m={} but these configs can only have m<={}",
                m, max_m
            ),
            MatmulInvalidProblem::ExceededNSize { n, max_n } => write!(
                f,
                "Problem has n={} but these configs can only have n<={}",
                n, max_n,
            ),
            MatmulInvalidProblem::ExceededBatchSize { b, max_b } => write!(
                f,
                "Problem has {} batches but these configs can only have batches<={}",
                b, max_b,
            ),
            MatmulInvalidProblem::InvalidLineSizeLhs { size, line_size } => write!(
                f,
                "the lhs tensor can't be read with line size={line_size} and dimension={size}"
            ),
            MatmulInvalidProblem::InvalidLineSizeRhs { size, line_size } => write!(
                f,
                "The rhs tensor can't be read with line size={line_size} and dimension={size}"
            ),
            MatmulInvalidProblem::InvalidLineSizeOut { size, line_size } => write!(
                f,
                "The out tensor can't be written with line size={line_size} and dimension={size}"
            ),
        }
    }
}

impl Debug for MatmulAvailabilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatmulAvailabilityError::PlaneOperationsUnavailable => {
                writeln!(f, "Plane operations not supported.")
            }
            MatmulAvailabilityError::TypesUnavailable { input, output } => {
                writeln!(
                    f,
                    "Types input={:?} and/or output={:?} not supported.",
                    input, output,
                )
            }
            MatmulAvailabilityError::CmmaInstructionUnavailable {
                input,
                output,
                m,
                n,
                k,
            } => writeln!(
                f,
                "Cmma on inputs {:?} and outputs {:?} with shape m={:?}, n={:?}, k={:?} not supported.",
                input,
                output, m, n, k
            ),
        }
    }
}
