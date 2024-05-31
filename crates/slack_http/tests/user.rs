use slack_http::oauth::AccessToken;
use slack_http::{client::AuthClient, team, user};
use slack_http::{Cursor, Limit};

pub struct TestEnv {
    pub authed_bot_client: AuthClient,
    pub authed_user_client: AuthClient,
    pub invalid_bot_client: AuthClient,
    pub invalid_user_client: AuthClient,
    pub team_id: team::Id,
}

fn setup() -> TestEnv {
    let bat = AccessToken(std::env::var("SLACK_BOT_ACCESS_TOKEN").unwrap());
    let uat = AccessToken(std::env::var("SLACK_USER_ACCESS_TOKEN").unwrap());
    let team_id = team::Id(std::env::var("SLACK_TEAM_ID").unwrap());

    let authed_bot_client = slack_http::client::AuthClient::new(bat).unwrap();
    let authed_user_client = slack_http::client::AuthClient::new(uat).unwrap();

    TestEnv {
        invalid_bot_client: slack_http::client::AuthClient::new(AccessToken(
            "HUHWHATTHISBE".to_string(),
        ))
        .unwrap(),
        invalid_user_client: slack_http::client::AuthClient::new(AccessToken(
            "HUHWHATTHISBE".to_string(),
        ))
        .unwrap(),
        authed_bot_client,
        authed_user_client,
        team_id,
    }
}

#[tokio::test]
async fn it_should_list_users() {
    let test_env = setup();

    let _users = user::list(
        &test_env.authed_user_client,
        &test_env.team_id,
        &Cursor(None),
        &Limit::default(),
    )
    .await
    .unwrap();

    let _users = user::list(
        &test_env.authed_bot_client,
        &test_env.team_id,
        &Cursor(None),
        &Limit::default(),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn it_should_parse_list_users_error() {
    let test_env = setup();

    let err = user::list(
        &test_env.invalid_user_client,
        &test_env.team_id,
        &Cursor(None),
        &Limit::default(),
    )
    .await
    .unwrap_err();

    assert_eq!(err.get_slack_error().unwrap(), "invalid_auth")
}
