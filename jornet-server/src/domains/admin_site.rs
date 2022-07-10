use actix_web::{cookie::Cookie, dev::Service, web, HttpRequest, HttpResponse, Responder, Scope};
use biscuit_auth::{Biscuit, KeyPair};

use crate::configuration::Settings;

use super::admins::authorize;

pub const AUTH_COOKIE_KEY: &str = "AUTH";

pub(crate) fn admin_site() -> Scope {
    web::scope("/admin").service(
        web::scope("")
            .wrap_fn(|req, srv| {
                let is_login_page = req.path() == "/admin/";
                req.cookie(AUTH_COOKIE_KEY);
                let root = req.app_data::<web::Data<KeyPair>>().unwrap();

                let auth_valid = |cookie: &Cookie| {
                    Biscuit::from_base64(cookie.value(), |_| root.public())
                        .ok()
                        .and_then(|b| authorize(&b))
                        .is_some()
                };

                match (is_login_page, req.cookie(AUTH_COOKIE_KEY)) {
                    (_, Some(cookie)) => {
                        if auth_valid(&cookie) {
                            if is_login_page {
                                // logged in and accessing login page, redirect to hello
                                Box::pin(async move {
                                    Ok(req.into_response(
                                        HttpResponse::Found()
                                            .insert_header(("Location", "/admin/hello"))
                                            .finish(),
                                    ))
                                })
                            } else {
                                srv.call(req)
                            }
                        } else {
                            // invalid cookie, logout to remove it
                            Box::pin(async move {
                                Ok(req.into_response(
                                    HttpResponse::Found()
                                        .insert_header(("Location", "/admin/logout"))
                                        .finish(),
                                ))
                            })
                        }
                    }
                    // not logged and login page
                    (true, None) => srv.call(req),
                    // not logged and any page, redirect to login
                    (false, None) => Box::pin(async move {
                        Ok(req.into_response(
                            HttpResponse::Found()
                                .insert_header(("Location", "/admin/"))
                                .finish(),
                        ))
                    }),
                }
            })
            .route("/", web::get().to(index))
            .route("hello", web::get().to(hello))
            .route("logout", web::get().to(logout)),
    )
}

async fn index(config: web::Data<Settings>) -> impl Responder {
    HttpResponse::Ok().content_type("Text/Html").body(format!(
        r#"
<html>

<head>
    <title>Jornet Admin Panel</title>
</head>

<body>
<a href="https://github.com/login/oauth/authorize?client_id={}">Authenticate with GitHub</a>
<hr />
<form action="/auth/by_uuid">
<label for="uuid">UUID:</label>
<input type="text" id="uuid" name="uuid">
<input type="submit" value="Authenticate with UUID">
</form>
</body>

</html>
    "#,
        config.github_admin_app.client_id
    ))
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().content_type("Text/Html").body(
        r#"
<html>

<head>
    <title>Jornet Admin Panel</title>
</head>

<body>HELLO
<a href="/admin/logout">logout</a>
<script>
let biscuit = document.cookie.split('; ').find(row => row.startsWith('AUTH='))?.split('=')[1];
fetch(
    '/api/admin/whoami',
    {headers: {
        'Authorization': 'Bearer ' + biscuit
    }}
)
  .then(response => response.json())
  .then(data => console.log(data));
</script>
</body>

</html>
    "#,
    )
}

async fn logout(req: HttpRequest) -> impl Responder {
    let mut cookie = req.cookie(AUTH_COOKIE_KEY).unwrap();
    cookie.set_path("/");
    let mut response = HttpResponse::Found()
        .insert_header(("Location", "/admin/"))
        .finish();
    response.add_removal_cookie(&cookie).unwrap();
    response
}
