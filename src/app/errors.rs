use thiserror::Error;

#[derive(Error, Debug)]
pub enum PersonError {
  #[error("member not found")]
  PersonNotFound,
  #[error("failed to update member")]
  PersonUpdateFailure,
  #[error("failed to create member")]
  PersonCreationFailure,
  #[error("failed to delete member")]
  PersonDeleteFailure
}

pub type ErrorMessage = String;

pub trait ResponseErrorTrait {
  fn create(person_error: PersonError) -> ErrorMessage;
}

impl ResponseErrorTrait for ErrorMessage {
  fn create(person_error: PersonError) -> ErrorMessage {
    match person_error {
      PersonError::PersonNotFound => ErrorMessage::from(" member not found"),
      PersonError::PersonUpdateFailure => ErrorMessage::from("failed to update PersonError"),
      PersonError::PersonCreationFailure => ErrorMessage::from("failed to create person"),
      PersonError::PersonDeleteFailure => ErrorMessage::from("failure to delete member"),
    }
  }
}