#!/bin/bash

SCRIPT_FOLDER=$( dirname -- "$0"; )
IJHTTP=$SCRIPT_FOLDER/ijhttp/ijhttp

if [[ ! -f $IJHTTP ]]; then
  echo "Cannot find http client. Downloading..."
  cd "$SCRIPT_FOLDER"
  curl -L -f -o tmp.zip https://jb.gg/ijhttp/latest
  unzip tmp.zip > /dev/null
  rm tmp.zip
fi

$IJHTTP -e advent -p "$SCRIPT_FOLDER"/http-client.private.env.json -v "$SCRIPT_FOLDER"/http-client.env.json \
  -L HEADERS "$SCRIPT_FOLDER"/download.http