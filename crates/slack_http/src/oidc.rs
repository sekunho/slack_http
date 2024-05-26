use crate::{
    client::{AuthClient, BasicClient},
    team,
};
use reqwest::Url;
use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

const GET_OIDC_TOKEN: &str = "https://slack.com/api/openid.connect.token";
const GET_USER_INFO: &str = "https://slack.com/api/openid.connect.userInfo";

#[derive(Debug)]
pub struct Token(pub String);

#[derive(Deserialize)]
#[serde(untagged)]
enum TokenResponse {
    Ok { access_token: String },
    Error { error: String },
}

#[derive(Debug, Error)]
pub enum GetTokenError {
    #[error("")]
    Request(reqwest::Error),
    #[error("")]
    Deserialize(reqwest::Error),
    #[error("")]
    Slack(String),
}

pub async fn get_token(
    basic_client: &BasicClient,
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &str,
) -> Result<Token, GetTokenError> {
    let mut params = HashMap::new();

    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);
    params.insert("code", code);
    params.insert("redirect_uri", redirect_uri);

    let url = Url::parse(GET_OIDC_TOKEN).expect("not a URL lol");

    let res = basic_client
        .0
        .post(url)
        .form(&params)
        .send()
        .await
        .map_err(GetTokenError::Request)?;

    let json = res
        .json::<TokenResponse>()
        .await
        .map_err(GetTokenError::Deserialize)?;

    match json {
        TokenResponse::Ok { access_token } => Ok(Token(access_token)),
        TokenResponse::Error { error } => Err(GetTokenError::Slack(error)),
    }
}

#[derive(Debug, Deserialize)]
pub struct User {
    #[serde(rename = "https://slack.com/user_id")]
    pub id: String,
    #[serde(rename = "https://slack.com/team_id")]
    pub team_id: team::Id,
    pub picture: Url,
    pub given_name: String,
    pub family_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum GetUserInfoResponse {
    Ok(User),
    Error { error: SlackGetUserInfoError },
}

#[derive(Debug, Error)]
pub enum GetUserInfoError {
    #[error("slack failed to fulfill the request")]
    Slack(#[from] SlackGetUserInfoError),
    #[error("failed to deserialize response data")]
    Deserialize(reqwest::Error),
    #[error("failed to send request data")]
    Request(reqwest::Error),
}

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "snake_case")]
pub enum SlackGetUserInfoError {
    #[error("access to a resource specified in the request is denied")]
    AccessDenied,
    #[error("authentication token is for a deleted user or workspace when using a bot token")]
    AccountInactive,
    #[error("the endpoint has been deprecated")]
    DeprecatedEndpoint,
    #[error("administrators have suspended the ability to post a message")]
    EkmAccessDenied,
    #[error("the method cannot be called from an Enterprise")]
    EnterpriseIsRestricted,
    #[error("some aspect of authentication cannot be validated. either the provided token is invalid or the request originates from an IP address disallowed from making the request")]
    InvalidAuth,
    #[error("this method cannot be called by a legacy bot")]
    IsBot,
    #[error("the method has been deprecated")]
    MethodDeprecated,
    #[error("the token used is not granted the specific scope permissions required to complete this request")]
    MissingScope,
    #[error("the token type used in this request is not allowed")]
    NotAllowedTokenType,
    #[error("no authentication token provided")]
    NotAuthed,
    #[error("the workspace token used in this request does not have the permissions necessary to complete the request. make sure your app is a member of the conversation it's attempting to post a message to")]
    NoPermission,
    #[error("the workspace is undergoing an enterprise migration and will not be available until migration is complete")]
    OrgLoginRequired,
    #[error("authentication token has expired")]
    TokenExpired,
    #[error("authentication token is for a deleted user or workspace or the app has been removed when using a user token")]
    TokenRevoked,
    #[error("two factor setup is required")]
    TwoFactorSetupRequired,
    #[error("access to this method is limited on the current network")]
    AccessLimited,
    #[error("the server could not complete your operation(s) without encountering a catastrophic error. it's possible some aspect of the operation succeeded before the error was raised")]
    FatalError,
    #[error("the server could not complete your operation(s) without encountering an error, likely due to a transient issue on our end. it's possible some aspect of the operation succeeded before the error was raised")]
    InternalError,
    #[error("the method was passed an argument whose name falls outside the bounds of accepted or expected values. This includes very long names and names with non-alphanumeric characters other than _. if you get this error, it is typically an indication that you have made a very malformed API call")]
    InvalidArgName,
    #[error("the method was either called with invalid arguments or some detail about the arguments passed is invalid, which is more likely when using complex arguments like blocks or attachments")]
    InvalidArguments,
    #[error("the method was passed an array as an argument. please only input valid strings")]
    InvalidArrayArg,
    #[error("the method was called via a POST request, but the charset specified in the Content-Type header was invalid. valid charset names are: utf-8 iso-8859-1")]
    InvalidCharset,
    #[error("the method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid")]
    InvalidFormData,
    #[error("the method was called via a POST request, but the specified Content-Type was invalid. valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain")]
    InvalidPostType,
    #[error("the method was called via a POST request and included a data payload, but the request did not include a Content-Type header")]
    MissingPostType,
    #[error("the request has been ratelimited. Refer to the Retry-After header for when to retry the request")]
    RateLimited,
    #[error("the method was called via a POST request, but the POST data was either missing or truncated")]
    RequestTimeout,
    #[error("the service is temporarily unavailable")]
    ServiceUnavailable,
    #[error("the workspace associated with your request is currently undergoing migration to an Enterprise Organization. web API and other platform operations will be intermittently unavailable until the transition is complete")]
    TeamAddedToOrg,
}

pub async fn get_user_info(auth_client: &AuthClient) -> Result<User, GetUserInfoError> {
    let params: HashMap<String, String> = HashMap::new();
    let url = Url::parse(GET_USER_INFO).expect("not a URL lol");

    let res = auth_client
        .0
        .post(url)
        .form(&params)
        .send()
        .await
        .map_err(GetUserInfoError::Request)?;

    let json = res
        .json::<GetUserInfoResponse>()
        .await
        .map_err(GetUserInfoError::Deserialize)?;

    match json {
        GetUserInfoResponse::Ok(user) => Ok(user),
        GetUserInfoResponse::Error { error } => {
            println!("{:#?}", error);
            Err(GetUserInfoError::Slack(error))
        }
    }
}
