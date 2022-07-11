module Main exposing (Model, init, main, subscriptions, update, view)

import Browser
import Browser.Navigation as Nav
import Home
import Http
import Login
import Messages exposing (LoginMsg(..), Msg(..), oauthConfigDecoder)
import Url
import Url.Parser exposing ((</>), Parser, map, oneOf, s)



-- MAIN


main : Program () Model Msg
main =
    Browser.application
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        , onUrlChange = UrlChanged
        , onUrlRequest = LinkClicked
        }



-- ROUTER


type Route
    = Home
    | AdminLogin


routeParser : Parser (Route -> a) a
routeParser =
    oneOf
        [ map Home (s "")
        , map AdminLogin (s "admin")
        ]


parseUrl : Url.Url -> Route
parseUrl url =
    case Url.Parser.parse routeParser url of
        Just route ->
            route

        Nothing ->
            Home



-- MODEL


type alias Model =
    { key : Nav.Key
    , route : Route
    , home : Home.Model
    , login : Login.Model
    }


init : () -> Url.Url -> Nav.Key -> ( Model, Cmd Msg )
init _ url key =
    ( Model key (parseUrl url) Home.Model (Login.Model "" Nothing Nothing)
    , Http.get
        { url = "/api/config/oauth"
        , expect = Http.expectJson (\r -> LoginMsg (SetOauthConfig r)) oauthConfigDecoder
        }
    )


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        LinkClicked urlRequest ->
            case urlRequest of
                Browser.Internal url ->
                    ( model, Nav.pushUrl model.key (Url.toString url) )

                Browser.External href ->
                    ( model, Nav.load href )

        UrlChanged url ->
            ( { model | route = parseUrl url }
            , Cmd.none
            )

        LoginMsg login_msg ->
            Login.update login_msg model.login
                |> (\( login_model, command ) -> ( { model | login = login_model }, command ))



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none



-- VIEW


view : Model -> Browser.Document Msg
view model =
    case model.route of
        AdminLogin ->
            Login.view model.login

        Home ->
            Home.view model.home
