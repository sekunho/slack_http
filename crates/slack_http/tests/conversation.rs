use slack_http::{client::AuthClient, team};
use slack_http_types::option::Limit;

fn setup() -> (AuthClient, AuthClient, String) {
    let bat = std::env::var("SLACK_BOT_ACCESS_TOKEN").unwrap();
    let uat = std::env::var("SLACK_USER_ACCESS_TOKEN").unwrap();
    let team_id = std::env::var("SLACK_TEAM_ID").unwrap();

    let bot_client = slack_http::client::AuthClient::new(bat).unwrap();
    let user_client = slack_http::client::AuthClient::new(uat).unwrap();

    (bot_client, user_client, team_id)
}

#[tokio::test]
async fn it_should_list_channels() {
    let (bot_client, user_client, team_id) = setup();

    let opts = slack_http::conversation::ListOptions::new()
        .include_public(true)
        .include_private(false);

    let channels = slack_http::conversation::list(&bot_client, team_id.as_str(), None, opts)
        .await
        .unwrap();

    for channel in channels.results().into_iter() {
        assert!(!channel.is_private)
    }

    let channels = slack_http::conversation::list(
        &user_client,
        team_id.as_str(),
        None,
        opts.include_public(false).include_private(true),
    )
    .await
    .unwrap();

    for channel in channels.results().into_iter() {
        assert!(channel.is_private)
    }
}

#[tokio::test]
async fn it_should_paginate_channels() {
    let (_bot_client, user_client, team_id) = setup();
    let limit = Limit::new(2).unwrap();

    let opts = slack_http::conversation::ListOptions::new()
        .include_public(true)
        .include_private(false)
        .set_limit(limit);

    let channels_1 = slack_http::conversation::list(&user_client, team_id.as_str(), None, opts)
        .await
        .unwrap();

    assert!(!channels_1.results().is_empty());

    let channels_2 =
        slack_http::conversation::list(&user_client, team_id.as_str(), channels_1.cursor(), opts)
            .await
            .unwrap();

    assert!(!channels_2.results().is_empty());
    assert_ne!(channels_1.results(), channels_2.results());
}

#[tokio::test]
async fn it_should_invite_user() {
    let (_bot_client, user_client, team_id) = setup();

    let (users, _) =
        slack_http::user::list_active_users(&user_client, team::Id(team_id.clone()), None)
            .await
            .unwrap();

    let user = users.into_iter().find(|u| u.name == "SOCIAL").unwrap();

    let channels =
        slack_http::conversation::list(&user_client, team_id.as_str(), None, Default::default())
            .await
            .unwrap();

    let test_channel = channels
        .results()
        .iter()
        .find(|c| c.name == "test")
        .unwrap();

    // Pre-emptively kick
    let _ = slack_http::conversation::kick(
        &user_client,
        test_channel.id.0.as_str(),
        user.id.0.as_str(),
    )
    .await;

    slack_http::conversation::invite(&user_client, &test_channel.id, vec![user.clone().id])
        .await
        .unwrap()
}
