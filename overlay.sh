#!/bin/bash
# Launch the overlay
alacritty --config-file ~/.config/alacritty/overlay.yml -e top &
# Wait for window to appear and make it click-through
sleep 1
WINDOW_ID=$(xdotool search --class Overlay | head -1)
if [ -n "$WINDOW_ID" ]; then
    echo $WINDOW_ID
    # Make window click-through
    xdotool windowfocus $WINDOW_ID
    xprop -id $WINDOW_ID -f _NET_WM_STATE 32a -set _NET_WM_STATE _NET_WM_STATE_BELOW
    
    # Alternative method using xwininfo and xprop
    # xprop -id $WINDOW_ID -f _NET_WM_WINDOW_TYPE 32a -set _NET_WM_WINDOW_TYPE _NET_WM_WINDOW_TYPE_DESKTOP
fi
