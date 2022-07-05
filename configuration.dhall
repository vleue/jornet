let types = ./configuration_types.dhall

let database: types.DatabaseSettings =
    { username      = "postgres"
    , password      = "password"
    , host          = "127.0.0.1"
    , port          = 5432
    , database_name = "jornet"
    }

let configuration: types.Settings = 
    { application_port  = 8080
    , database          = database
    , private_key       = None Text
    }

in configuration
