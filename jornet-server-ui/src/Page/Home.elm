module Page.Home exposing (Model, init, subscriptions, update, view)

import Browser
import Browser.Navigation as Nav
import Html exposing (a, br, text)
import Html.Attributes exposing (href)
import Messages exposing (Msg(..))
import Url



-- MODEL


type alias Model =
    {}


init : () -> Url.Url -> Nav.Key -> ( Model, Cmd Msg )
init _ _ _ =
    ( Model, Cmd.none )



-- UPDATE


update : Msg -> Model -> ( Model, Cmd Msg )
update _ model =
    ( model, Cmd.none )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none



-- VIEW


view : Model -> Browser.Document Msg
view _ =
    { title = "Jornet"
    , body =
        [ text "Jornet is the social game server made for game jams!"
        , br [] []
        , a [ href "admin/" ] [ text "Connect to Admin Panel" ]
        ]
    }
