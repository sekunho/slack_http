use slack_http::{chat::MessageOptions, client::AuthClient, Cursor};

pub struct TestEnv {
    pub authed_bot_client: AuthClient,
    pub authed_user_client: AuthClient,
    pub invalid_bot_client: AuthClient,
    pub invalid_user_client: AuthClient,
    pub team_id: String,
}

fn setup() -> TestEnv {
    let bat = std::env::var("SLACK_BOT_ACCESS_TOKEN").unwrap();
    let uat = std::env::var("SLACK_USER_ACCESS_TOKEN").unwrap();
    let team_id = std::env::var("SLACK_TEAM_ID").unwrap();

    let authed_bot_client = slack_http::client::AuthClient::new(bat).unwrap();
    let authed_user_client = slack_http::client::AuthClient::new(uat).unwrap();

    TestEnv {
        invalid_bot_client: slack_http::client::AuthClient::new("HUHWHATTHISBE".to_string())
            .unwrap(),
        invalid_user_client: slack_http::client::AuthClient::new("HUHWHATTHISBE".to_string())
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
        test_env.team_id.as_str(),
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

    assert_eq!(message.text(), "Hello, world! (from bot)");
    assert_eq!(message.username().unwrap(), "who am i");
}

#[tokio::test]
async fn user_should_post_message() {
    let test_env = setup();
    let opts = MessageOptions::new();

    let channels = slack_http::conversation::list(
        &test_env.authed_user_client,
        test_env.team_id.as_str(),
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

    assert_eq!(message.text(), "Hello, world! (from user)")
}

#[tokio::test]
async fn it_should_parse_post_message_error() {
    let test_env = setup();
    let opts = MessageOptions::new();

    let channels = slack_http::conversation::list(
        &test_env.authed_user_client,
        test_env.team_id.as_str(),
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

    assert_eq!(err.get_slack_error().unwrap().as_str(), "invalid_auth")
}
