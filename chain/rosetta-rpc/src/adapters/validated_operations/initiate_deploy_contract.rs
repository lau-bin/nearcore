use super::ValidatedOperation;

pub(crate) struct InitiateDeployContractOperation {
    pub(crate) sender_account: crate::models::AccountIdentifier,
}

impl ValidatedOperation for InitiateDeployContractOperation {
    const OPERATION_TYPE: crate::models::OperationType =
        crate::models::OperationType::InitiateDeployContract;

    fn into_operation(
        self,
        operation_identifier: crate::models::OperationIdentifier,
    ) -> crate::models::Operation {
        crate::models::Operation {
            operation_identifier,

            account: self.sender_account,
            amount: None,
            metadata: None,

            related_operations: None,
            type_: Self::OPERATION_TYPE,
            status: None,
        }
    }
}

impl TryFrom<crate::models::Operation> for InitiateDeployContractOperation {
    type Error = crate::errors::ErrorKind;

    fn try_from(operation: crate::models::Operation) -> Result<Self, Self::Error> {
        Self::validate_operation_type(operation.type_)?;
        Ok(Self { sender_account: operation.account })
    }
}
