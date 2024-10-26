use slack_http::oauth::AccessToken;
use slack_http::{client::AuthClient, team, user, Cursor, Limit};

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
// conversations.list

#[tokio::test]
async fn it_should_list_channels() {
    let test_env = setup();

    let opts = slack_http::conversation::ListOptions::new()
        .include_public(true)
        .include_private(false);

    let channels = slack_http::conversation::list(
        &test_env.authed_bot_client,
        &test_env.team_id,
        &Cursor(None),
        opts,
    )
    .await
    .unwrap();

    for channel in channels.results().iter() {
        assert!(!channel.is_private)
    }

    let channels = slack_http::conversation::list(
        &test_env.authed_user_client,
        &test_env.team_id,
        &Cursor(None),
        opts.include_public(false).include_private(true),
    )
    .await
    .unwrap();

    for channel in channels.results().iter() {
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
        &test_env.team_id,
        &Cursor(None),
        opts,
    )
    .await
    .unwrap();

    assert!(!channels_1.results().is_empty());

    let channels_2 = slack_http::conversation::list(
        &test_env.authed_user_client,
        &test_env.team_id,
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
        &test_env.team_id,
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

    let page = slack_http::user::list(
        &test_env.authed_user_client,
        &test_env.team_id,
        &Cursor(None),
        &Limit::default(),
    )
    .await
    .unwrap();

    let member_1 = page
        .results
        .iter()
        .find(|u| u.profile.display_name == "MEMBER_1")
        .unwrap();

    let channels = slack_http::conversation::list(
        &test_env.authed_user_client,
        &test_env.team_id,
        &Cursor(None),
        Default::default(),
    )
    .await
    .unwrap();

    let test_channel = channels
        .results
        .into_iter()
        .find(|c| c.name == "test_invite")
        .unwrap();

    // Pre-emptively kick
    let _ = slack_http::conversation::kick(
        &test_env.authed_user_client,
        &test_channel.id,
        &member_1.id,
    )
    .await;

    slack_http::conversation::invite(
        &test_env.authed_user_client,
        &test_channel.id,
        vec![member_1.id.clone()],
    )
    .await
    .unwrap()
}

#[tokio::test]
async fn it_should_parse_error() {
    let test_env = setup();

    let page = slack_http::user::list(
        &test_env.authed_user_client,
        &test_env.team_id,
        &Cursor(None),
        &Limit::default(),
    )
    .await
    .unwrap();

    let user = page
        .results
        .iter()
        .find(|u| u.profile.display_name == "MEMBER_1")
        .unwrap();

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
        .find(|c| c.name == "test_invite")
        .unwrap();

    // Pre-emptively kick
    let _ =
        slack_http::conversation::kick(&test_env.authed_user_client, &test_channel.id, &user.id)
            .await;

    let err = slack_http::conversation::invite(
        &test_env.invalid_user_client,
        &test_channel.id,
        vec![user.id.clone()],
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

    let page = slack_http::user::list(
        &test_env.authed_user_client,
        &test_env.team_id,
        &Cursor(None),
        &Limit::default(),
    )
    .await
    .unwrap();

    let user = page
        .results
        .into_iter()
        .find(|u| u.profile.display_name == "MEMBER_1")
        .unwrap();

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
        .find(|c| c.name == "test_kick")
        .unwrap();

    // Pre-emptively invite. It's fine if this fails
    let _ = slack_http::conversation::invite(
        &test_env.authed_user_client,
        &test_channel.id,
        vec![user.id.clone()],
    )
    .await;

    slack_http::conversation::kick(&test_env.authed_user_client, &test_channel.id, &user.id)
        .await
        .unwrap();
}

#[tokio::test]
async fn it_should_parse_kick_error() {
    let test_env = setup();

    let page = slack_http::user::list(
        &test_env.authed_user_client,
        &test_env.team_id,
        &Cursor(None),
        &Limit::default(),
    )
    .await
    .unwrap();

    let user = page
        .results
        .iter()
        .find(|u| u.profile.display_name == "MEMBER_1")
        .unwrap();

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
        .find(|c| c.name == "test_kick")
        .unwrap();

    // Pre-emptively kick
    let _ =
        slack_http::conversation::kick(&test_env.authed_user_client, &test_channel.id, &user.id)
            .await;

    let err = slack_http::conversation::invite(
        &test_env.invalid_user_client,
        &test_channel.id,
        vec![user.id.clone()],
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

    let page = slack_http::user::list(
        &test_env.authed_user_client,
        &test_env.team_id,
        &Cursor(None),
        &Limit::default(),
    )
    .await
    .unwrap();

    let user_ids: Vec<user::Id> = page
        .results
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

    let page = slack_http::user::list(
        &test_env.authed_user_client,
        &test_env.team_id,
        &Cursor(None),
        &Limit::default(),
    )
    .await
    .unwrap();

    let user_ids: Vec<user::Id> = page
        .results
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

    let page = slack_http::user::list(
        &test_env.authed_user_client,
        &test_env.team_id,
        &Cursor(None),
        &Limit::default(),
    )
    .await
    .unwrap();

    let users: Vec<_> = page
        .results
        .iter()
        .filter(|u| u.id.as_str() != "USLACKBOT" && u.profile.real_name.as_str() != "Polly")
        .collect();

    let opts = slack_http::conversation::ListOptions::new()
        .include_public(true)
        .include_private(false);

    let channels = slack_http::conversation::list(
        &test_env.authed_user_client,
        &test_env.team_id,
        &Cursor(None),
        opts,
    )
    .await
    .unwrap();

    let general = channels
        .results()
        .iter()
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

    for user in users.iter() {
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
        &test_env.team_id,
        &Cursor(None),
        opts,
    )
    .await
    .unwrap();

    let general = channels
        .results()
        .iter()
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
        &test_env.team_id,
        &Cursor(None),
        opts,
    )
    .await
    .unwrap_err();

    assert_eq!(err.get_slack_error().unwrap(), "invalid_auth")
}
