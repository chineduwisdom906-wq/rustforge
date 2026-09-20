use std::fmt;

#[derive(Debug, PartialEq)]
pub enum OperationError {
    InvalidTransition {
        from: OperationStatus,
        to: OperationStatus,
    },
}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OperationError::InvalidTransition { from, to } => {
                write!(f, "Cannot transition operation from {} to {}", from, to)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OperationStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl fmt::Display for OperationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OperationStatus::Pending => write!(f, "Pending"),
            OperationStatus::Running => write!(f, "Running"),
            OperationStatus::Completed => write!(f, "Completed"),
            OperationStatus::Failed => write!(f, "Failed"),
        }
    }
}

pub struct Operation {
    id: u64,
    name: String,
    status: OperationStatus,
}

impl Operation {
    pub fn new(id: u64, name: String) -> Operation {
        Operation {
            id,
            name,
            status: OperationStatus::Pending,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn status(&self) -> OperationStatus {
        self.status
    }

    pub fn start(&mut self) -> Result<(), OperationError> {
        match self.status {
            OperationStatus::Pending => {
                self.status = OperationStatus::Running;
                Ok(())
            }
            current => Err(OperationError::InvalidTransition {
                from: current,
                to: OperationStatus::Running,
            }),
        }
    }

    pub fn complete(&mut self) -> Result<(), OperationError> {
        match self.status {
            OperationStatus::Running => {
                self.status = OperationStatus::Completed;
                Ok(())
            }
            current => Err(OperationError::InvalidTransition {
                from: current,
                to: OperationStatus::Completed,
            }),
        }
    }

    pub fn fail(&mut self) -> Result<(), OperationError> {
        match self.status {
            OperationStatus::Running => {
                self.status = OperationStatus::Failed;
                Ok(())
            }
            current => Err(OperationError::InvalidTransition {
                from: current,
                to: OperationStatus::Failed,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_can_start() {
        let mut op = Operation::new(1001, "identity".to_string());

        let result = op.start();

        assert!(result.is_ok());
        assert_eq!(op.status, OperationStatus::Running);
    }

    #[test]
    fn operation_cannot_complete_before_starting() {
        let mut op = Operation::new(1002, "identity".to_string());

        let result = op.complete();

        assert_eq!(
            result,
            Err(OperationError::InvalidTransition {
                from: OperationStatus::Pending,
                to: OperationStatus::Completed,
            })
        );

        assert_eq!(op.status, OperationStatus::Pending);
    }

    #[test]
    fn operation_can_complete_after_starting() {
        let mut op = Operation::new(1003, "identity".to_string());

        op.start().unwrap();
        let result = op.complete();

        assert_eq!(result, Ok(()));
        assert_eq!(op.status, OperationStatus::Completed);
    }

    #[test]
    fn operation_can_fail_after_starting() {
        let mut op = Operation::new(1004, "identity".to_string());

        op.start().unwrap();
        let result = op.fail();

        assert_eq!(result, Ok(()));
        assert_eq!(op.status, OperationStatus::Failed);
    }

    #[test]
    fn completed_operation_cannot_start_again() {
        let mut op = Operation::new(1005, "identity".to_string());

        op.start().unwrap();
        op.complete().unwrap();

        let result = op.start();

        assert_eq!(
            result,
            Err(OperationError::InvalidTransition {
                from: OperationStatus::Completed,
                to: OperationStatus::Running,
            })
        );

        assert_eq!(op.status, OperationStatus::Completed);
    }

    #[test]
    fn failed_operation_cannot_start_again() {
        let mut op = Operation::new(1006, "identity".to_string());

        op.start().unwrap();
        op.fail().unwrap();

        let result = op.start();

        assert_eq!(
            result,
            Err(OperationError::InvalidTransition {
                from: OperationStatus::Failed,
                to: OperationStatus::Running,
            })
        );

        assert_eq!(op.status, OperationStatus::Failed);
    }
}
