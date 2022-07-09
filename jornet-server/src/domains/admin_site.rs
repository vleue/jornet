use actix_web::{web, HttpResponse, Responder, Scope};

use crate::configuration::Settings;

pub(crate) fn admin_site() -> Scope {
    web::scope("/admin").route("", web::get().to(index))
}

async fn index(config: web::Data<Settings>) -> impl Responder {
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
