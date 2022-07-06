let DatabaseSettings : Type =
      { username        : Text
      , password        : Text
      , host            : Text
      , port            : Natural
      , database_name   : Text
      }

let OAuth : Type = 
      { client_id       : Text
      , client_secret   : Text
      }

let Settings : Type =
      { application_host    : Text
      , application_port    : Natural
      , database            : DatabaseSettings
      , private_key         : Optional Text
      , github_admin_app    : OAuth
      }

in
    { Settings
    , DatabaseSettings
    , OAuth
    }
