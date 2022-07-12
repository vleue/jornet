module Page.Login exposing (Model, subscriptions, update, view)

import Browser
import Html exposing (a, button, div, hr, input, text)
import Html.Attributes exposing (disabled, href, placeholder, type_, value)
import Html.Events exposing (onClick, onInput)
import Http
import Json.Encode
import Messages exposing (LoginMsg(..), Msg(..), tokenReplyDecoder)
import Result.Extra
import UUID



-- MODEL


type alias Model =
    { uuid : String
    , oauth_github : Maybe String
    , token : Maybe String
    }


update : LoginMsg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Uuid uuid ->
            ( { model | uuid = uuid }, Cmd.none )

        LoginWithUuid ->
            ( model
            , Http.post
                { url = "/oauth/by_uuid"
                , body = Http.jsonBody (uuidAuthRequestEncoder (UuidAuthRequest model.uuid))
                , expect = Http.expectJson (\r -> LoginMsg (GetToken r)) tokenReplyDecoder
                }
            )

        SetOauthConfig response ->
            case response of
                Ok config ->
                    ( { model | oauth_github = Just config.github_app_id }, Cmd.none )

                Err _ ->
                    ( model, Cmd.none )

        GetToken response ->
            case response of
                Ok token ->
                    ( { model | token = Just token.token }, Cmd.none )

                Err _ ->
                    ( model, Cmd.none )


type alias UuidAuthRequest =
    { uuid : String }


uuidAuthRequestEncoder : UuidAuthRequest -> Json.Encode.Value
uuidAuthRequestEncoder request =
    Json.Encode.object [ ( "uuid", Json.Encode.string request.uuid ) ]



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none



-- VIEW


view : Model -> Browser.Document Msg
view model =
    { title = "Jornet Admin Panel"
    , body =
        [ model.oauth_github
            |> Maybe.map (\client_id -> a [ href (String.concat [ "https://github.com/login/oauth/authorize?client_id=", client_id ]) ] [ text "Connect using GitHub" ])
            |> Maybe.withDefault (div [] [ text "Connect using GitHub (disabled)" ])
        , hr [] []
        , div []
            [ input [ type_ "text", placeholder "Uuid", value model.uuid, onInput (\input -> LoginMsg (Uuid input)) ] []
            , button [ onClick (LoginMsg LoginWithUuid), disabled (Result.Extra.isErr (UUID.fromString model.uuid)) ] [ text "Connect with UUID" ]
            ]
        ]
    }
