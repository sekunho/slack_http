use slack_http::oauth::AccessToken;
use slack_http::{chat::MessageOptions, client::AuthClient, team, Cursor};

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

///////////////////////////////////////////////////////////////////////////////
// chat.postMessage

#[tokio::test]
async fn bot_should_post_message() {
    let test_env = setup();
    let opts = MessageOptions::new()
        .set_icon_emoji("taco".to_string())
        .set_username("who am i".to_string());

    let channels = slack_http::conversation::list(
        &test_env.authed_user_client,
        &test_env.team_id,
        &Cursor(None),
        Default::default(),
    )
    .await
    .unwrap();

    let test_channel = channels
        .results()
        .iter()
        .find(|c| c.name == "test_post_message")
        .unwrap();

    let message = slack_http::chat::post_message(
        &test_env.authed_bot_client,
        &test_channel.id,
        "Hello, world! (from bot)",
        &opts,
    )
    .await
    .unwrap();

    assert_eq!(message.text.as_str(), "Hello, world! (from bot)");
    assert_eq!(message.username.unwrap().as_str(), "who am i");
}

#[tokio::test]
async fn user_should_post_message() {
    let test_env = setup();
    let opts = MessageOptions::new();

    let channels = slack_http::conversation::list(
        &test_env.authed_user_client,
        &test_env.team_id,
        &Cursor(None),
        Default::default(),
    )
    .await
    .unwrap();

    let test_channel = channels
        .results()
        .iter()
        .find(|c| c.name == "test_post_message")
        .unwrap();

    let message = slack_http::chat::post_message(
        &test_env.authed_user_client,
        &test_channel.id,
        "Hello, world! (from user)",
        &opts,
    )
    .await
    .unwrap();

    assert_eq!(message.text.as_str(), "Hello, world! (from user)")
}

#[tokio::test]
async fn it_should_parse_post_message_error() {
    let test_env = setup();
    let opts = MessageOptions::new();

    let channels = slack_http::conversation::list(
        &test_env.authed_user_client,
        &test_env.team_id,
        &Cursor(None),
        Default::default(),
    )
    .await
    .unwrap();

    let test_channel = channels
        .results()
        .iter()
        .find(|c| c.name == "test_post_message")
        .unwrap();

    let err = slack_http::chat::post_message(
        &test_env.invalid_bot_client,
        &test_channel.id,
        "Hello, world!",
        &opts,
    )
    .await
    .unwrap_err();

    assert_eq!(err.get_slack_error().unwrap(), "invalid_auth")
}

///////////////////////////////////////////////////////////////////////////////
// chat.postEphemeral

#[tokio::test]
async fn bot_should_post_ephemeral_message() {
    let test_env = setup();

    let opts = MessageOptions::new()
        .set_icon_emoji("taco".to_string())
        .set_username("who am i".to_string());

    let channels = slack_http::conversation::list(
        &test_env.authed_user_client,
        &test_env.team_id,
        &Cursor(None),
        Default::default(),
    )
    .await
    .unwrap();

    let test_channel = channels
        .results()
        .iter()
        .find(|c| c.name == "test_post_ephemeral")
        .unwrap();

    let (users, _) =
        slack_http::user::list_active_users(&test_env.authed_user_client, &test_env.team_id, None)
            .await
            .unwrap();

    let user = users.iter().find(|u| u.name == "OWNER").unwrap();

    let _message = slack_http::chat::post_ephemeral(
        &test_env.authed_bot_client,
        &test_channel.id,
        &user.id,
        "Hello, world! (from bot)",
        &opts,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn user_should_post_ephemeral_message() {
    let test_env = setup();
    let opts = MessageOptions::new();

    let channels = slack_http::conversation::list(
        &test_env.authed_user_client,
        &test_env.team_id,
        &Cursor(None),
        Default::default(),
    )
    .await
    .unwrap();

    let test_channel = channels
        .results()
        .iter()
        .find(|c| c.name == "test_post_ephemeral")
        .unwrap();

    let (users, _) =
        slack_http::user::list_active_users(&test_env.authed_user_client, &test_env.team_id, None)
            .await
            .unwrap();

    let user = users.iter().find(|u| u.name == "OWNER").unwrap();

    let _message = slack_http::chat::post_ephemeral(
        &test_env.authed_user_client,
        &test_channel.id,
        &user.id,
        "Hello, world! (from user)",
        &opts,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn it_should_parse_post_ephemeral_message_error() {
    let test_env = setup();
    let opts = MessageOptions::new();

    let channels = slack_http::conversation::list(
        &test_env.authed_user_client,
        &test_env.team_id,
        &Cursor(None),
        Default::default(),
    )
    .await
    .unwrap();

    let test_channel = channels
        .results()
        .iter()
        .find(|c| c.name == "test_post_ephemeral")
        .unwrap();

    let (users, _) =
        slack_http::user::list_active_users(&test_env.authed_user_client, &test_env.team_id, None)
            .await
            .unwrap();

    let user = users.iter().find(|u| u.name == "OWNER").unwrap();

    let err = slack_http::chat::post_ephemeral(
        &test_env.invalid_bot_client,
        &test_channel.id,
        &user.id,
        "Hello, world!",
        &opts,
    )
    .await
    .unwrap_err();

    assert_eq!(err.get_slack_error().unwrap(), "invalid_auth")
}
