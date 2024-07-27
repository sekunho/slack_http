use slack_http::{client::AuthClient, emoji, oauth::AccessToken};
use slack_http_types::team;

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
async fn it_should_list_emojis() {
    let test_env = setup();
    let emojis = emoji::list(&test_env.authed_user_client).await.unwrap();

    assert_eq!(emojis.get("shipit").unwrap(), "alias:squirrel");
}

#[tokio::test]
async fn it_should_parse_list_emojis_error() {
    let test_env = setup();
    let err = emoji::list(&test_env.invalid_user_client)
        .await
        .unwrap_err();

    assert_eq!(err.get_slack_error().unwrap(), "invalid_auth");
}
