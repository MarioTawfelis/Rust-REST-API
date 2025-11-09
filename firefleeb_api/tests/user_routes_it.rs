mod common;

use common::setup_postgres;
use firefleeb_api::db::PgPool;
use firefleeb_api::handlers::dtos::UserResponse;
use firefleeb_api::routes::{handle_rejection, user_routes::user_routes};
use serde_json::{Value, json};
use warp::Filter;

fn user_filter(
    pool: PgPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = std::convert::Infallible> + Clone {
    user_routes(pool).recover(handle_rejection)
}

#[tokio::test]
async fn register_and_login_round_trip() {
    let test_db = setup_postgres();
    let filter = user_filter(test_db.pool.clone());

    let registration = json!({
        "email": "route-user@example.com",
        "password": "StrongPass8"
    });

    let register_resp = warp::test::request()
        .method("POST")
        .path("/users")
        .json(&registration)
        .reply(&filter)
        .await;

    assert_eq!(register_resp.status(), 200);
    let created: UserResponse =
        serde_json::from_slice(register_resp.body()).expect("created user response");
    assert_eq!(created.email.as_str(), "route-user@example.com");

    let login_resp = warp::test::request()
        .method("POST")
        .path("/users/login")
        .json(&registration)
        .reply(&filter)
        .await;

    assert_eq!(login_resp.status(), 200);
    let logged_in: UserResponse =
        serde_json::from_slice(login_resp.body()).expect("login response");
    assert_eq!(logged_in.id, created.id);
}

#[tokio::test]
async fn update_user_email_via_route() {
    let test_db = setup_postgres();
    let filter = user_filter(test_db.pool.clone());

    let registration = json!({
        "email": "email-before@example.com",
        "password": "OriginalPass9"
    });

    let register_resp = warp::test::request()
        .method("POST")
        .path("/users")
        .json(&registration)
        .reply(&filter)
        .await;
    assert_eq!(register_resp.status(), 200);
    let created: UserResponse =
        serde_json::from_slice(register_resp.body()).expect("user response");

    let update_payload = json!({
        "email": "email-after@example.com"
    });

    let update_resp = warp::test::request()
        .method("PUT")
        .path(&format!("/users/{}", created.id))
        .json(&update_payload)
        .reply(&filter)
        .await;

    assert_eq!(update_resp.status(), 200);
    let updated: UserResponse =
        serde_json::from_slice(update_resp.body()).expect("updated response");
    assert_eq!(updated.email.as_str(), "email-after@example.com");
}

#[tokio::test]
async fn password_reset_requires_correct_old_password() {
    let test_db = setup_postgres();
    let filter = user_filter(test_db.pool.clone());

    let registration = json!({
        "email": "reset-user@example.com",
        "password": "InitialPwd9"
    });

    let register_resp = warp::test::request()
        .method("POST")
        .path("/users")
        .json(&registration)
        .reply(&filter)
        .await;
    assert_eq!(register_resp.status(), 200);
    let created: UserResponse =
        serde_json::from_slice(register_resp.body()).expect("user response");

    // Attempt password change with wrong current password
    let wrong_reset = json!({
        "old_password": "wrong-pass",
        "new_password": "NewSecurePwd9"
    });
    let wrong_resp = warp::test::request()
        .method("PUT")
        .path(&format!("/users/password-reset/{}", created.id))
        .json(&wrong_reset)
        .reply(&filter)
        .await;
    assert_eq!(wrong_resp.status(), 401);

    // Change password with correct credentials
    let reset_payload = json!({
        "old_password": "InitialPwd9",
        "new_password": "NewSecurePwd9"
    });
    let reset_resp = warp::test::request()
        .method("PUT")
        .path(&format!("/users/password-reset/{}", created.id))
        .json(&reset_payload)
        .reply(&filter)
        .await;
    assert_eq!(reset_resp.status(), 200);

    // Old password should now fail login
    let old_login_resp = warp::test::request()
        .method("POST")
        .path("/users/login")
        .json(&registration)
        .reply(&filter)
        .await;
    assert_eq!(old_login_resp.status(), 401);
    let body: Value = serde_json::from_slice(old_login_resp.body()).expect("body");
    assert_eq!(body.get("status").and_then(|v| v.as_u64()), Some(401));

    // New password logs in successfully
    let new_login = json!({
        "email": "reset-user@example.com",
        "password": "NewSecurePwd9"
    });
    let new_login_resp = warp::test::request()
        .method("POST")
        .path("/users/login")
        .json(&new_login)
        .reply(&filter)
        .await;
    assert_eq!(new_login_resp.status(), 200);
    let logged_in: UserResponse =
        serde_json::from_slice(new_login_resp.body()).expect("login response");
    assert_eq!(logged_in.id, created.id);
}
