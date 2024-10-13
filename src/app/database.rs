#![allow(unused)]
//contains the database functions & connections
// to run database surreal start surrealkv:// --user root --pass root **rem to change the passwords
cfg_if::cfg_if! {
  if #[cfg(feature = "ssr")] {

    use surrealdb::engine::remote::ws::{Client, Ws};
    use surrealdb::opt::auth::Root;
    use surrealdb::{Error, Surreal};
    use once_cell::sync::Lazy;

    use crate::app::{Person, PersonError};

    static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

    pub async fn open_db_connection() {
      DB.connect::<Ws>("127.0.0.1:8000").await;
      DB.signin(Root {
        username: "root",
        password: "root",
      })
      .await;
      DB.use_ns("surreal").use_db("person").await;
    }

    pub async fn get_all_persons() -> Option<Vec<Person>> {
      open_db_connection().await;
      let get_all_persons = DB.query("SELECT * FROM person ORDER BY joined_date DESC").await;
      DB.invalidate().await;

      match get_all_persons {
        Ok(mut res) => {
          let found = res.take(0);
          match found {
            Ok(found_person) => Some(found_person),
            Err(_) => None,
          }
        },
        Err(_) => None
      }
    }
    pub async fn add_person(new_person: Person) -> Option<Person> {
      open_db_connection().await;
      let results = DB.create(("person", new_person.uuid.clone()))
      .content(new_person)
      .await;
      DB.invalidate().await;

      match results {
        Ok(created_person) => created_person,
        Err(e) => {
          println!("Error in adding person: {:?}", e);
          None
        },
      }
    } 

    pub async fn delete_person(person_uuid: String) -> Result<Option<Person>, PersonError> {
      open_db_connection().await;
      let delete_results = DB.delete(("person", person_uuid)).await;
      DB.invalidate().await;

      match delete_results {
        Ok(deleted_person) => Ok(deleted_person),
        Err(_) => Err(PersonError::PersonDeleteFailure)
      }
    }

    pub async fn update_person(uuid: String, title: String, level: String, compensation: i32) -> Result<Option<Person>, PersonError> {
      open_db_connection().await;
      //attempt to find the person in the db
      let find_person: Result<Option<Person>, Error> = DB.select(("person", &uuid)).await;
      match find_person {
        Ok(found) => {
          // we found the person, we update
          match found {
            Some(found_person) => {
              let updated_person: Result<Option<Person>, Error> = DB.update(("person", &uuid))
                .merge(Person::new(
                  uuid, 
                  found_person.name, 
                  title, 
                  level, 
                  compensation, 
                  found_person.joined_date
                )).await;
                let _ = DB.invalidate().await;
                match updated_person {
                  Ok(returned_user) => Ok(returned_user),
                  Err(_) => Err(PersonError::PersonUpdateFailure)
                }
            },
            None => Err(PersonError::PersonUpdateFailure)
          }
        },
        Err(_) => {
          let _ = DB.invalidate().await;
          Err(PersonError::PersonNotFound)
        }
      }
    }
  }
}