use std::env;
use std::io::stdin;

use diesel::ExpressionMethods;
use diesel::Insertable;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use dotenv::dotenv;

use masacarri::models::Page;
use masacarri::schema::pages;
use masacarri::schema::users;
use masacarri::{
    db::{self, *},
    models::User,
};

#[derive(Insertable)]
#[table_name = "users"]
struct NewUser {
    id: uuid::Uuid,
    username: String,
    password_hash: String,
    flags: i32,
}

fn add_user(conn: MainDbConnection, username: &str) {
    let new_id = uuid::Uuid::new_v4();

    let res = diesel::insert_into(users::dsl::users)
        .values(NewUser {
            id: new_id,
            username: username.to_string(),
            password_hash: "-".to_string(),
            flags: 0,
        })
        .execute(&conn);

    match res {
        Ok(_) => {
            println!("user deletion succeeded: {}", username);
        }
        Err(_) => {
            eprintln!("failed to create user");
        }
    };
}

fn delete_user(conn: MainDbConnection, username: &str) {
    let res =
        diesel::delete(users::dsl::users.filter(users::dsl::username.eq(username))).execute(&conn);

    match res {
        Ok(_) => {
            println!("user creation succeeded: {}", username);
        }
        Err(_) => {
            eprintln!("failed to load users");
        }
    };
}

fn list_users(conn: MainDbConnection) {
    let res = users::dsl::users.load::<User>(&conn);

    match res {
        Ok(user_list) => {
            for user in user_list {
                println!("{} (id:{})", user.username, user.id);
            }
        }
        Err(_) => {
            eprintln!("failed to load users");
        }
    };
}

fn list_pages(conn: MainDbConnection) {
    let res = pages::dsl::pages.load::<Page>(&conn);

    match res {
        Ok(page_list) => {
            for page in page_list {
                println!("{} (id:{})", page.title, page.id);
            }
        }
        Err(_) => {
            eprintln!("failed to load users");
        }
    };
}

fn update_password(conn: MainDbConnection, username: &str) {
    let user_count = users::dsl::users
        .filter(users::dsl::username.eq(username))
        .count()
        .execute(&conn);

    match user_count {
        Ok(1) => {}
        Ok(0) => {
            eprintln!("user not found");
            return;
        }
        Ok(_) => {
            eprintln!("unknown error: multiple users");
            return;
        }
        Err(_) => {
            eprintln!("failed to check user existence");
            return;
        }
    }

    println!("input password: ");
    let mut passwd = String::new();
    stdin().read_line(&mut passwd).expect("failed to get input");
    passwd.pop();
    let passwd = passwd;

    println!("retype password: ");
    let mut retyped_passwd = String::new();
    stdin()
        .read_line(&mut retyped_passwd)
        .expect("failed to get input");
    retyped_passwd.pop();
    let retyped_passwd = retyped_passwd;

    if passwd != retyped_passwd {
        eprintln!("password not matched");
        return;
    }

    let hash = bcrypt::hash(passwd, 8);

    let hash = if let Ok(hash) = hash {
        hash
    } else {
        eprintln!("failed to update password");
        return;
    };

    let res = diesel::update(users::dsl::users.filter(users::dsl::username.eq(username)))
        .set((users::dsl::password_hash.eq(hash),))
        .execute(&conn);

    match res {
        Ok(_) => {
            println!("password is successfully updated");
        }
        Err(_) => {
            eprintln!("failed to update password");
        }
    }
}

fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();

    let conn = db::establish_main_db();

    if args.len() <= 1 {
        eprintln!("command needed");
        return;
    }

    match args[1].as_str() {
        "adduser" => {
            if args.len() <= 2 {
                eprintln!("username needed");
                return;
            }

            add_user(conn, args[2].as_str());
        }
        "deluser" => {
            if args.len() <= 2 {
                eprintln!("username needed");
                return;
            }

            delete_user(conn, args[2].as_str());
        }
        "list" => {
            if args.len() <= 2 {
                eprintln!("username needed");
                return;
            }
            match args[2].as_str() {
                "user" => list_users(conn),
                "page" => list_pages(conn),
                item => eprintln!("unknown list item: '{}'", item),
            }
        }
        "passwd" => {
            if args.len() <= 2 {
                eprintln!("username needed");
                return;
            }
            update_password(conn, args[2].as_str());
        }
        cmd => {
            eprintln!("unknown command: '{}'", cmd);
        }
    }
}
