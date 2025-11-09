mod common;
use common::setup_postgres;

use bcrypt::{DEFAULT_COST, hash, verify};
use firefleeb_api::db::get_conn;
use firefleeb_api::db::user_repository;
use firefleeb_api::models::user::{NewUser, UpdateUser, User};
use firefleeb_api::types::email::Email;

#[test]
fn create_and_get_user_by_email_successfully() {
    let test_db = setup_postgres();
    let pool = test_db.pool;

    let mut conn = get_conn(&pool).expect("Failed to get DB connection");

    // Create a new user
    let email = Email::parse("tester@firefleeb.de").expect("valid email");
    let password_hash = hash("validpassword", DEFAULT_COST).expect("hash password");
    let new_user = NewUser {
        email: email.clone(),
        password_hash: password_hash,
    };

    // Test create_user
    let created_user: User =
        user_repository::create_user(&mut conn, &new_user).expect("create user");
    assert_eq!(created_user.email, email);

    // Test get_user_by_email
    let fetched_user: User = user_repository::get_user_by_email(&mut conn, &email)
        .expect("fetch user by email")
        .expect("user exists");
    assert_eq!(fetched_user.id, created_user.id);
    assert_eq!(fetched_user.email, created_user.email);
}

#[test]
fn update_user_reset_password_successfully() {
    let testdb = setup_postgres();
    let mut conn = get_conn(&testdb.pool).expect("conn");

    let email = Email::parse("reset@example.com").expect("valid email");
    let initial_hash = hash("old-password-123", DEFAULT_COST).unwrap();

    let new_user = NewUser {
        email: email.clone(),
        password_hash: initial_hash.clone(),
    };

    let created: User = user_repository::create_user(&mut conn, &new_user).expect("create user");

    // Sanity: the stored hash should verify with the old password
    assert!(verify("old-password-123", &created.password_hash).unwrap());

    let new_hash = hash("new-password-456", DEFAULT_COST).unwrap();

    let patch = UpdateUser {
        email: None,
        password_hash: Some(new_hash.clone()),
    };

    let updated = user_repository::update_user(&mut conn, created.id, &patch).expect("update user");

    assert!(verify("new-password-456", &updated.password_hash).unwrap());
    assert!(!verify("old-password-123", &updated.password_hash).unwrap());

    let fetched = user_repository::get_user_by_email(&mut conn, &email)
        .expect("get by email")
        .expect("user exists");

    assert_eq!(fetched.id, created.id);
    assert!(verify("new-password-456", &fetched.password_hash).unwrap());
}
