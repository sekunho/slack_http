# slack_http

`slack_http` is an HTTP client for Slack's API. Each endpoint (except `oauth.*`
`openid.*`) is tested against Slack's real servers from issuing the request, to
deserializing its response -- no mocks!
