# slack_http

`slack_http` is an HTTP client for Slack's API. Each endpoint (except `oauth.*`
`openid.*`) is tested against Slack's real servers from issuing the request, to
deserializing its response -- no mocks!

## Test environment

Since this library depends on integration tests, there's some setup required for
you to run them locally.

1. Create a simple Slack [app](https://api.slack.com/apps). See [slack_manifest.json](./slack_manifest.json)
for the exact manifest.

2. Ensure the following users with the following `display_name`s are present

  - `OWNER`: (admin & owner)
  - `MEMBER_1`: (regular member)

Also, I added the _Polly_ bot because bots have missing fields apparently.

3. Ensure the following channels are present in your test WS.

| name                | visibility | members          |
|---------------------|------------|------------------|
| general             | public     | OWNER,slack_http |
| test_invite         | public     | OWNER            |
| test_kick           | public     | OWNER            |
| secret              | private    | OWNER            |
| test_post_message   | public     | OWNER,slack_http |
| test_post_ephemeral | public     | OWNER,slack_http |
