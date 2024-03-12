// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod database;
mod graph;
use axum::response::Html;
use axum::{extract::Query, routing::get, Extension, Router};
use database::User;
use oauth2::basic::BasicClient;
use oauth2::TokenResponse;
use oauth2::{
    reqwest::Error, AuthUrl, AuthorizationCode, ClientId, CsrfToken, HttpRequest, HttpResponse,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenUrl,
};
use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tauri::Manager;

#[derive(Clone)]
struct AuthState {
    csrf_token: CsrfToken,
    pkce: Arc<(PkceCodeChallenge, String)>,
    client: Arc<BasicClient>,
    socket_addr: SocketAddr,
}

#[tauri::command]
async fn authenticate(handle: tauri::AppHandle) -> bool {
    println!("Authenticating");
    let auth = handle.state::<AuthState>();
    let (auth_url, _) = auth
        .client
        .authorize_url(|| auth.csrf_token.clone())
        .add_scope(Scope::new("user.read".to_string()))
        .add_scope(Scope::new("offline_access".to_string()))
        .set_pkce_challenge(auth.pkce.0.clone())
        .url();
    println!("Opening {}", auth_url);
    open::that(auth_url.to_string()).unwrap();
    println!("Done");
    true
}

#[tauri::command]
async fn refresh(handle: tauri::AppHandle) -> bool {
    let user = get_latest_token().await;
    if user.is_none() {
        return false;
    }
    let user = user.unwrap();
    let refresh_token = user.token.refresh_token().unwrap();
    let auth = handle.state::<AuthState>();
    let token = auth
        .client
        .exchange_refresh_token(&refresh_token)
        .request_async(local_async_http_client)
        .await;
    if token.is_err() {
        println!("Error: {}", token.unwrap_err());
        return false;
    }
    let token = token.unwrap();
    let refresh_state = database::save_token(token, user.name).await;
    if refresh_state.is_err() {
        return false;
    }
    true
}

#[tauri::command]
async fn get_latest_token() -> Option<User> {
    let user = database::get_last_user().await;
    if user.is_err() {
        println!("Error: {}", user.unwrap_err());
        return None;
    }
    let user = user.unwrap();
    if user.is_none() {
        println!("Error: No user found {:?}", user);
        return None;
    }
    Some(user.unwrap())
}
fn main() -> surrealdb::Result<()> {
    let port = 9197;
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port); // or any other port
    let redirect_url = format!("http://localhost:{port}/callback").to_string();
    let state = AuthState {
        csrf_token: CsrfToken::new_random(),
        pkce: Arc::new((
            pkce_code_challenge,
            PkceCodeVerifier::secret(&pkce_code_verifier).to_string(),
        )),
        client: Arc::new(create_client(RedirectUrl::new(redirect_url).unwrap())),
        socket_addr,
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_latest_token,
            authenticate,
            refresh
        ])
        .setup(|app| {
            let handle = app.handle();
            tauri::async_runtime::spawn(async move {
                let res = database::initialize_database(&handle).await;
                println!("Database initialized successfully {}", res.is_ok());
                tauri::async_runtime::spawn(async move { run_server(handle).await });
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

fn create_client(redirect_url: RedirectUrl) -> BasicClient {
    let client_id = ClientId::new("9b2eb93e-6c34-4ae3-be46-cca67beafc7d".to_string());

    let auth_url =
        AuthUrl::new("https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string());

    let token_url =
        TokenUrl::new("https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string());

    BasicClient::new(client_id, None, auth_url.unwrap(), token_url.ok())
        .set_redirect_uri(redirect_url)
}
#[derive(Deserialize)]
struct CallbackQuery {
    code: AuthorizationCode,
    state: CsrfToken,
}
async fn authorize(
    handle: Extension<tauri::AppHandle>,
    query: Query<CallbackQuery>,
) -> Html<&'static str> {
    let auth = handle.state::<AuthState>();

    if query.state.secret() != auth.csrf_token.secret() {
        println!("Suspected Man in the Middle attack!");
        return Html(include_str!("../error.html"));
    }
    let oauth_http_client = local_async_http_client;

    let token = auth
        .client
        .exchange_code(query.code.clone())
        .set_pkce_verifier(PkceCodeVerifier::new(auth.pkce.1.clone()))
        .request_async(oauth_http_client)
        .await;

    if let Err(e) = token {
        println!("Failed to get token: {}", e);
        return Html(include_str!("../error.html"));
    }

    let token = token.unwrap();
    let client = graph::GraphClient::new(token.clone());
    let user_details = client.get_user().await.unwrap();

    let resp = database::save_token(token, user_details.user_principal_name).await;
    println!("{:?}", resp);
    Html(&include_str!("../redirect.html"))
}

async fn run_server(handle: tauri::AppHandle) -> Result<(), axum::Error> {
    let app = Router::new()
        .route("/callback", get(authorize))
        .layer(Extension(handle.clone()));

    let listener = tokio::net::TcpListener::bind(&handle.state::<AuthState>().socket_addr.clone())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn local_async_http_client(
    request: HttpRequest,
) -> Result<HttpResponse, Error<reqwest::Error>> {
    let client = {
        let builder = reqwest::Client::builder();

        // Following redirects opens the client up to SSRF vulnerabilities.
        // but this is not possible to prevent on wasm targets
        #[cfg(not(target_arch = "wasm32"))]
        let builder = builder.redirect(reqwest::redirect::Policy::none());

        builder.build().map_err(Error::Reqwest)?
    };

    let mut request_builder = client
        .request(request.method, request.url.as_str())
        .body(request.body);
    for (name, value) in &request.headers {
        request_builder = request_builder.header(name.as_str(), value.as_bytes());
    }
    request_builder = request_builder.header("Origin", "localhost:9197".as_bytes());
    let request = request_builder.build().map_err(Error::Reqwest)?;

    let response = client.execute(request).await.map_err(Error::Reqwest)?;

    let status_code = response.status();
    let headers = response.headers().to_owned();
    let chunks = response.bytes().await.map_err(Error::Reqwest)?;
    Ok(HttpResponse {
        status_code,
        headers,
        body: chunks.to_vec(),
    })
}
