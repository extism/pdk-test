module KitchenSink where

import Data.Int
import Extism.PDK
import Extism.PDK.HTTP
import Extism.PDK.Memory

fail' :: String -> IO a
fail' msg = do
  setError msg
  error msg

kitchenSink :: IO Int32
kitchenSink = do
  pluginInput <- inputString
  -- Config
  _ <- getConfig "test"

  setVar "test_var" (Just "something")

  v <- getVar "test_var"

  if v /= Just "something"
    then fail' "Invalid var"
    else pure ()

  res <- sendRequest (newRequest "https://extism.org") (Nothing :: Maybe String)
  let rc = statusCode res
  if rc == 0
    then fail' "HTTP failed"
    else pure ()

  free (responseMemory res)

  logInfo "INFO"
  logDebug "DEBUG"
  logWarn "WARN"
  logError "ERROR"

  output pluginInput
  pure 0

foreign export ccall "kitchen_sink" kitchenSink :: IO Int32
