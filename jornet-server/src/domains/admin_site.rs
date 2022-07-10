use actix_session::{storage::CookieSessionStore, Session, SessionExt, SessionMiddleware};
use actix_web::{cookie::Key, dev::Service, web, HttpResponse, Responder, Scope};

use crate::configuration::Settings;

const AUTH_COOKIE_KEY: &str = "biscuit";

pub(crate) fn admin_site() -> Scope {
    web::scope("/admin").service(
        web::scope("")
            .wrap_fn(|req, srv| {
                let session = req.get_session();

                let login_page = req.path() == "/admin/";

                match (login_page, session.get::<i32>(AUTH_COOKIE_KEY)) {
                    (false, Ok(Some(_))) => srv.call(req),
                    (true, Ok(Some(_))) => todo!("redirect to welcome page"),
                    (true, _) => srv.call(req),
                    _ => Box::pin(async move {
                        Ok(req.into_response(
                            HttpResponse::Found()
                                .insert_header(("Location", "/admin/"))
                                .finish(),
                        ))
                    }),
                }
            })
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                Key::from(&[0; 64]),
            ))
            .route("/", web::get().to(index))
            .route("hello", web::get().to(hello)),
    )
}

async fn index(session: Session, config: web::Data<Settings>) -> impl Responder {
    let mut counter = 1;
    if let Some(count) = session.get::<i32>(AUTH_COOKIE_KEY).unwrap() {
        counter = count + 1;
        session.insert(AUTH_COOKIE_KEY, counter).unwrap();
    } else {
        session.insert(AUTH_COOKIE_KEY, counter).unwrap();
    }
    eprintln!("-> {:?}", counter);

    HttpResponse::Ok().content_type("Text/Html").body(format!(
        r#"
<html>

<head>
    <title>Jornet Admin Panel</title>
</head>

<body><a href="https://github.com/login/oauth/authorize?client_id={}">Authenticate with GitHub</a>
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
</body>

</html>
    "#,
    )
}
