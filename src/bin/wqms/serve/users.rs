use std::sync::RwLock;
use lazy_static::lazy_static;

lazy_static! {
    static ref USERS : RwLock<Users> = {
        RwLock::new(Users::new())
    };
}

#[derive(Clone)]
pub struct User {
    id: Option<u64>,
    first_name: String,
}

// #[juniper::object]
// #[graphql(description = "A user")]
// impl User {
//     #[graphql(description = "A user id")]
//     pub fn id(&self) -> i32 {
//         self.id.unwrap_or(0) as i32
//     }

//     #[graphql(description = "A user first_name")]
//     pub fn first_name(&self) -> &str {
//         &self.first_name
//     }
// }
pub type Users = Vec<User>;


// #[derive(juniper::GraphQLInputObject)]
pub struct NewUser {
    first_name: String,
}

impl NewUser {
    pub fn to_internal(self) -> User {
        User {
            id: None,
            first_name: self.first_name.to_owned(),
        }
    }
}

pub fn push(nu:NewUser) -> User {
    let mut interns = USERS.write().unwrap();
    let mut user = nu.to_internal();
    user.id = Some((interns.len()+1) as u64 );
    interns.push(user.clone());
    user
}


pub fn list()-> Users {
    let mut users = Users::new();
    let interns = USERS.read().unwrap();
    for u in interns.iter() {
        users.push(u.clone())
    }
    users
}

// pub struct State {
    // users: RwLock<Vec<User>>,
// }