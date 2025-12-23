{-# LANGUAGE DataKinds #-}
{-# LANGUAGE TypeOperators #-}
{-# LANGUAGE OverloadedStrings #-}

module Main where

import Gists
import Network.Wai
import Network.Wai.Handler.Warp
import Servant
import Servant.Swagger
import Servant.Swagger.UI
import Data.Time (getCurrentTime)
import Data.HashMap.Strict (fromList)
import Data.Proxy

-- Dummy in-memory data
dummyGists :: IO [Gist]
dummyGists = do
  now <- getCurrentTime
  let file1 = GistFile "hello.hs" (Just "Haskell") "main = putStrLn \"Hello World\""
      file2 = GistFile "readme.md" Nothing "# Example Gist"
      gist = Gist
        { gistId      = "1"
        , description = Just "My first gist"
        , public      = True
        , files       = fromList [("hello.hs", file1), ("readme.md", file2)]
        , createdAt   = now
        , updatedAt   = now
        }
  return [gist]

-- Handlers
gistsHandler :: Handler [Gist]
gistsHandler = liftIO dummyGists

gistByIdHandler :: Text -> Handler Gist
gistByIdHandler _ = do
  gists <- liftIO dummyGists
  case gists of
    (g:_) -> return g
    []    -> throwError err404 { errBody = "Gist not found" }

-- Combine into Servant server
server :: Server (GistsAPI :<|> "swagger.json" :> Get '[JSON] Swagger) 
server = gistsHandler :<|> gistByIdHandler :<|> pure gistsSwagger

apiProxy :: Proxy (GistsAPI :<|> "swagger.json" :> Get '[JSON] Swagger)
apiProxy = Proxy

app :: Application
app = serve apiProxy server

main :: IO ()
main = do
  putStrLn "Starting Gists server on port 8080..."
  run 8080 app
