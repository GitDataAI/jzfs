#!/bin/bash

regex="--config[[:space:]]+([^[:space:]]+)"
if [[ $@ =~ $regex ]]; then
  config_value="--config ${BASH_REMATCH[1]}"
  echo "Config value: $config_value"
fi

regex="--log-level[[:space:]]+([^[:space:]]+)"
if [[ $@ =~ $regex ]]; then
  loglevel="--log-level ${BASH_REMATCH[1]}"
  echo "Log level: $loglevel"
fi

/jzfs init $@

/jzfs daemon $config_value $loglevel
