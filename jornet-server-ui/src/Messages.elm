module Messages exposing (..)

import Browser
import Http
import Json.Decode exposing (Decoder, field, map, string)
import Url


type Msg
    = LinkClicked Browser.UrlRequest
    | UrlChanged Url.Url
    | LoginMsg LoginMsg


type LoginMsg
    = Uuid String
    | LoginWithUuid
    | SetOauthConfig (Result Http.Error OauthConfig)
    | GetToken (Result Http.Error TokenReply)


type alias OauthConfig =
    { github_app_id : String
    }


oauthConfigDecoder : Decoder OauthConfig
oauthConfigDecoder =
    map OauthConfig
        (field "github_app_id" string)


type alias TokenReply =
    { token : String
    }


tokenReplyDecoder : Decoder TokenReply
tokenReplyDecoder =
    map TokenReply
        (field "token" string)
