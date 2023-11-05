use std::env;
use std::path::PathBuf;
use std::error::Error;
extern crate winreg;
use winreg::enums::*;
use winreg::RegKey;
use std::process::Command;
use tauri::ipc::Scope;
use oauth2::{AuthUrl, TokenUrl, ClientId, ClientSecret, RedirectUrl, Scope as OtherScope, CsrfToken};
use oauth2::basic::BasicClient;

// Определите свои кастомные команды для Tauri
//#[tauri::command]
async fn full_authenticate_user(window: tauri::Window) -> Result<String, String> {
  request_admin_rights_for_registration();
  if let Err(e) = is_custom_url_scheme_set(){
    authenticate_user()?
  }
}
fn authenticate_user() -> Result<String, String> {
    let client_id = ClientId::new("385801417887-ruqcst9k4tvlifso7k5947bhicc7108e.apps.googleusercontent.com".to_string());
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap();
    let redirect_uri = RedirectUrl::new("http://localehost:5173".to_string()).unwrap();

    // Определите необходимые Scopes
    let scopes = vec![Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string())];

    // Создайте экземпляр OAuth2 клиента
    let client = BasicClient::new(client_id, None, auth_url, None)
        .set_redirect_uri(redirect_uri);

    // Генерируйте URL для авторизации
    let (auth_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(scopes.into_iter())
        .url();

    // Открывайте URL в браузере пользователя
    tauri::api::shell::open(&window, auth_url.to_string(), None).map_err(|e| e.to_string())?;

    Ok("URL opened in browser".to_string())
}

fn is_custom_url_scheme_set() -> Result<bool, Box<dyn std::error::Error>> {
  let hklm = RegKey::predef(HKEY_CLASSES_ROOT);
  match hklm.open_subkey("myscheme") {
      Ok(_) => Ok(true), // Если ключ существует, значит схема уже установлена.
      Err(_) => Ok(false), // Если ключа нет, значит схема не установлена.
  }
}

fn request_admin_rights_for_registration() -> Result<(), Box<dyn std::error::Error>> {
  if !is_custom_url_scheme_set()? {
      // Если схема URL не установлена, запросить административные права
      Command::new("cmd")
          .args(&["/C", "start", "/b", "/wait", "cmd", "/C", "echo", "Запрос административных прав"])
          .spawn()?
          .wait()?;
      // После получения прав, проверяем снова и при необходимости проводим регистрацию
      if !is_custom_url_scheme_set()? {
          // Регистрируем схему URL (вызов функции регистрации)
          set_custom_url_scheme();
      }
  }
  Ok(())
}

fn get_executable_path() -> Result<String, Box<dyn Error>> {
  let exec_path: PathBuf = env::current_exe()?;
  exec_path.to_str()
      .ok_or_else(|| "Could not convert executable path to string.".into())
      .map(|s| s.to_string())
}

fn set_custom_url_scheme() -> Result<(), Box<dyn Error>> {
  let hklm = RegKey::predef(HKEY_CLASSES_ROOT);
  let (key, _) = hklm.create_subkey("myscheme")?;
  key.set_value("", &"URL:My Custom Protocol")?;
  key.set_value("URL Protocol", &"")?;

  let shell_key = key.create_subkey("shell")?;
  let open_key = shell_key.create_subkey("open")?;
  let command_key = open_key.create_subkey("command")?;
  
  // Получаем путь к исполняемому файлу
  let app_path = get_executable_path()?;
  let command = format!("\"{}\" \"%1\"", app_path);

  command_key.set_value("", &command)?;

  Ok(())
}



#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main(){
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![full_authenticate_user])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}



