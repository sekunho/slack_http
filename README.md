# slack_http

`slack_http` is an HTTP client for Slack's API. Each endpoint (except `oauth.*`
`openid.*`) is tested against Slack's real servers from issuing the request, to
deserializing its response -- no mocks!

## Test environment

Since this library depends on integration tests, there's some setup required for
you to run them locally.

1. Create a simple Slack [app](https://api.slack.com/apps) with the following
manifest:

```yaml
display_information:
  name: slack_http
features:
  bot_user:
    display_name: slack_http
    always_online: false
oauth_config:
  scopes:
    user:
      - channels:read
      - groups:read
      - im:read
      - mpim:read
      - channels:write.invites
      - groups:write.invites
      - mpim:write.invites
      - im:write.invites
      - channels:write
      - groups:write
      - im:write
      - mpim:write
      - users:read
      - users:read.email
      - chat:write
      - team:read
    bot:
      - channels:read
      - groups:read
      - im:read
      - mpim:read
      - channels:write.invites
      - groups:write.invites
      - mpim:write.invites
      - im:write.invites
      - channels:manage
      - groups:write
      - im:write
      - mpim:write
      - users:read
      - users:read.email
      - chat:write
      - chat:write.customize
      - team:read
settings:
  org_deploy_enabled: false
  socket_mode_enabled: false
  token_rotation_enabled: false
```

2. Ensure the following users with the following `display_name`s are present

  - `OWNER`: (admin & owner)
  - `MEMBER_1`: (regular member)

3. Ensure the following channels are present in your test WS.

| name                | visibility | members          |
|---------------------|------------|------------------|
| general             | public     | OWNER,slack_http |
| test_invite         | public     | OWNER            |
| test_kick           | public     | OWNER            |
| secret              | private    | OWNER            |
| test_post_message   | public     | OWNER,slack_http |
| test_post_ephemeral | public     | OWNER,slack_http |
