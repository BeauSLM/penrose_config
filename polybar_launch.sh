#!/bin/sh

polybar-msg cmd quit                   # close all polybars

# IFS='<line feed>'
IFS='
'
for m in $(polybar --list-monitors); do
  n=$(echo "$m" | cut -d ":" -f1)
  if echo "$m" | grep -q "primary"
  then
    bar="tray"
    logfile="/polybar_tray.log"
  else
    bar="normal"
    logfile="/polybar_normal.log"
  fi
  MONITOR=$n polybar -r "$bar" 2>&1 | tee -a /tmp/"$logfile" &
done
