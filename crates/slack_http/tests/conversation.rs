use slack_http::{client::AuthClient, team};
use slack_http_types::{option::Limit, page::Cursor, user};

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
// conversations.list

#[tokio::test]
async fn it_should_list_channels() {
    let test_env = setup();

    let opts = slack_http::conversation::ListOptions::new()
        .include_public(true)
        .include_private(false);

    let channels = slack_http::conversation::list(
        &test_env.authed_bot_client,
        test_env.team_id.as_str(),
        &Cursor(None),
        opts,
    )
    .await
    .unwrap();

    for channel in channels.results().into_iter() {
        assert!(!channel.is_private)
    }

    let channels = slack_http::conversation::list(
        &test_env.authed_user_client,
        test_env.team_id.as_str(),
        &Cursor(None),
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
    let test_env = setup();
    let limit = Limit::new(2).unwrap();

    let opts = slack_http::conversation::ListOptions::new()
        .include_public(true)
        .include_private(false)
        .set_limit(limit);

    let channels_1 = slack_http::conversation::list(
        &test_env.authed_user_client,
        test_env.team_id.as_str(),
        &Cursor(None),
        opts,
    )
    .await
    .unwrap();

    assert!(!channels_1.results().is_empty());

    let channels_2 = slack_http::conversation::list(
        &test_env.authed_user_client,
        test_env.team_id.as_str(),
        channels_1.cursor(),
        opts,
    )
    .await
    .unwrap();

    assert!(!channels_2.results().is_empty());
    assert_ne!(channels_1.results(), channels_2.results());
}

#[tokio::test]
async fn it_should_parse_list_error() {
    let test_env = setup();

    let opts = slack_http::conversation::ListOptions::new()
        .include_public(true)
        .include_private(false);

    let err = slack_http::conversation::list(
        &test_env.invalid_bot_client,
        test_env.team_id.as_str(),
        &Cursor(None),
        opts,
    )
    .await
    .unwrap_err();

    assert_eq!(err.get_slack_error().unwrap(), "invalid_auth")
}

///////////////////////////////////////////////////////////////////////////////
// conversations.invite

#[tokio::test]
async fn it_should_invite_user() {
    let test_env = setup();

    let (users, _) = slack_http::user::list_active_users(
        &test_env.authed_user_client,
        team::Id(test_env.team_id.clone()),
        None,
    )
    .await
    .unwrap();

    let user = users.into_iter().find(|u| u.name == "SOCIAL").unwrap();

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
        .find(|c| c.name == "test_invite")
        .unwrap();

    // Pre-emptively kick
    let _ =
        slack_http::conversation::kick(&test_env.authed_user_client, &test_channel.id, &user.id)
            .await;

    slack_http::conversation::invite(
        &test_env.authed_user_client,
        &test_channel.id,
        vec![user.clone().id],
    )
    .await
    .unwrap()
}

#[tokio::test]
async fn it_should_parse_error() {
    let test_env = setup();

    let (users, _) = slack_http::user::list_active_users(
        &test_env.authed_user_client,
        team::Id(test_env.team_id.clone()),
        None,
    )
    .await
    .unwrap();

    let user = users.into_iter().find(|u| u.name == "SOCIAL").unwrap();

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
        .find(|c| c.name == "test_invite")
        .unwrap();

    // Pre-emptively kick
    let _ =
        slack_http::conversation::kick(&test_env.authed_user_client, &test_channel.id, &user.id)
            .await;

    let err = slack_http::conversation::invite(
        &test_env.invalid_user_client,
        &test_channel.id,
        vec![user.clone().id],
    )
    .await
    .unwrap_err();

    assert_eq!(err.get_slack_error().unwrap(), "invalid_auth");
}

///////////////////////////////////////////////////////////////////////////////
// conversations.kick

#[tokio::test]
async fn it_should_kick_user() {
    let test_env = setup();

    let (users, _) = slack_http::user::list_active_users(
        &test_env.authed_user_client,
        team::Id(test_env.team_id.clone()),
        None,
    )
    .await
    .unwrap();

    let user = users.into_iter().find(|u| u.name == "SOCIAL").unwrap();

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
        .find(|c| c.name == "test_kick")
        .unwrap();

    // Pre-emptively invite. It's fine if this fails
    let _ = slack_http::conversation::invite(
        &test_env.authed_user_client,
        &test_channel.id,
        vec![user.clone().id],
    )
    .await;

    slack_http::conversation::kick(
        &test_env.authed_user_client,
        &test_channel.id,
        &user.clone().id,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn it_should_parse_kick_error() {
    let test_env = setup();

    let (users, _) = slack_http::user::list_active_users(
        &test_env.authed_user_client,
        team::Id(test_env.team_id.clone()),
        None,
    )
    .await
    .unwrap();

    let user = users.into_iter().find(|u| u.name == "SOCIAL").unwrap();

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
        .find(|c| c.name == "test_kick")
        .unwrap();

    // Pre-emptively kick
    let _ =
        slack_http::conversation::kick(&test_env.authed_user_client, &test_channel.id, &user.id)
            .await;

    let err = slack_http::conversation::invite(
        &test_env.invalid_user_client,
        &test_channel.id,
        vec![user.clone().id],
    )
    .await
    .unwrap_err();

    assert_eq!(err.get_slack_error().unwrap(), "invalid_auth");
}

///////////////////////////////////////////////////////////////////////////////
// conversations.open

#[tokio::test]
async fn it_should_open_conversation_with_users() {
    let test_env = setup();

    let (users, _) = slack_http::user::list_active_users(
        &test_env.authed_user_client,
        team::Id(test_env.team_id.clone()),
        None,
    )
    .await
    .unwrap();

    let user_ids: Vec<user::Id> = users
        .into_iter()
        .filter(|u| u.id.0.as_str() != "USLACKBOT")
        .map(|u| u.id)
        .collect();

    let _conversation_id = slack_http::conversation::open(&test_env.authed_bot_client, user_ids)
        .await
        .unwrap();
}

#[tokio::test]
async fn it_should_parse_open_conversation_error() {
    let test_env = setup();

    let (users, _) = slack_http::user::list_active_users(
        &test_env.authed_user_client,
        team::Id(test_env.team_id.clone()),
        None,
    )
    .await
    .unwrap();

    let user_ids: Vec<user::Id> = users
        .into_iter()
        .filter(|u| u.id.0.as_str() != "USLACKBOT")
        .map(|u| u.id)
        .collect();

    let err = slack_http::conversation::open(&test_env.invalid_bot_client, user_ids)
        .await
        .unwrap_err();

    assert_eq!(err.get_slack_error().unwrap(), "invalid_auth");
}

///////////////////////////////////////////////////////////////////////////////
// conversations.members

#[tokio::test]
async fn it_should_list_members() {
    let test_env = setup();

    let (users, _) = slack_http::user::list_active_users(
        &test_env.authed_user_client,
        team::Id(test_env.team_id.clone()),
        None,
    )
    .await
    .unwrap();

    let users: Vec<_> = users
        .into_iter()
        .filter(|u| u.id.as_str() != "USLACKBOT")
        .collect();

    let opts = slack_http::conversation::ListOptions::new()
        .include_public(true)
        .include_private(false);

    let channels = slack_http::conversation::list(
        &test_env.authed_user_client,
        test_env.team_id.as_str(),
        &Cursor(None),
        opts,
    )
    .await
    .unwrap();

    let general = channels
        .results()
        .into_iter()
        .find(|c| c.name.as_str() == "general")
        .unwrap();

    let members = slack_http::conversation::members(
        &test_env.authed_bot_client,
        &general.id,
        &Cursor(None),
        Default::default(),
    )
    .await
    .unwrap();

    for user in users.into_iter() {
        assert!(members.results().contains(&user.id))
    }
}

#[tokio::test]
async fn it_should_paginate_members() {
    let test_env = setup();

    let opts = slack_http::conversation::ListOptions::new()
        .include_public(true)
        .include_private(false);

    let channels = slack_http::conversation::list(
        &test_env.authed_user_client,
        test_env.team_id.as_str(),
        &Cursor(None),
        opts,
    )
    .await
    .unwrap();

    let general = channels
        .results()
        .into_iter()
        .find(|c| c.name.as_str() == "general")
        .unwrap();

    let members_1 = slack_http::conversation::members(
        &test_env.authed_bot_client,
        &general.id,
        &Cursor(None),
        Limit::new(1).unwrap(),
    )
    .await
    .unwrap();

    assert!(members_1.results().len() == 1);

    let members_2 = slack_http::conversation::members(
        &test_env.authed_bot_client,
        &general.id,
        members_1.cursor(),
        Limit::new(1).unwrap(),
    )
    .await
    .unwrap();

    assert!(members_2.results().len() == 1);
    assert_ne!(members_1.results(), members_2.results());
}

#[tokio::test]
async fn it_should_parse_members_error() {
    let test_env = setup();

    let opts = slack_http::conversation::ListOptions::new()
        .include_public(true)
        .include_private(false);

    let err = slack_http::conversation::list(
        &test_env.invalid_bot_client,
        test_env.team_id.as_str(),
        &Cursor(None),
        opts,
    )
    .await
    .unwrap_err();

    assert_eq!(err.get_slack_error().unwrap(), "invalid_auth")
}
