let DatabaseSettings : Type =
      { username        : Text
      , password        : Text
      , host            : Text
      , port            : Natural
      , database_name   : Text
      }

let Settings : Type =
      { application_port    : Natural
      , database            : DatabaseSettings
      , private_key         : Optional Text
      }

in
    { Settings
    , DatabaseSettings
    }
