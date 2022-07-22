let types = ./configuration_types.dhall

let database: types.DatabaseSettings =
    { username      = env:POSTGRESQL_ADDON_USER as Text ? "postgres"
    , password      = env:POSTGRESQL_ADDON_PASSWORD as Text ? "password"
    , host          = env:POSTGRESQL_ADDON_HOST as Text ? "127.0.0.1"
    , port          = env:POSTGRESQL_ADDON_PORT ? 5432
    , database_name = env:POSTGRESQL_ADDON_DB as Text ? "jornet"
    }

let github_admin_app: types.OAuth = 
    ./github_admin_app.dhall ?
    { client_id     = env:GITHUB_ADMIN_CLIENT_ID as Text ? "disabled"
    , client_secret = env:GITHUB_ADMIN_CLIENT_SECRET as Text ? "disabled"
    }

in

{ application_host  = env:HOST as Text ? "127.0.0.1"
, application_port  = env:PORT ? 8080
, database          = database
, private_key       = Some (env:BISCUIT_KEY as Text) ? Some (./private_key) ? None Text
, github_admin_app  = github_admin_app
}: types.Settings
