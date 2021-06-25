#!/bin/bash

wspace bad_filename.ws
wspace empty_file.ws
wspace implicit_end.ws

wspace underflow_dup.ws
wspace underflow_copy.ws
wspace underflow_drop.ws
wspace underflow_slide.ws
wspace call_underflow_ret.ws

wspace push_zero_empty.ws

wspace div_zero.ws
wspace mod_zero.ws
wspace div_zero_unused.ws
wspace mod_zero_unused.ws

wspace printc_negative.ws

wspace leading_zero_labels.ws && echo
wspace printi_negative_zero.ws && echo

echo -n 'ß' | wspace readc_echo.ws && echo
printf '\r\n' | wspace readc_echo.ws && echo

echo '0xff' | wspace readi_echo.ws && echo
echo '0o77' | wspace readi_echo.ws && echo
echo '077' | wspace readi_echo.ws && echo
echo '-5' | wspace readi_echo.ws && echo
printf ' \t\v\f\r- \t\v\f\r5 \t\v\f\r' | wspace readi_echo.ws && echo

printf '\xff' | wspace readc_echo.ws && echo
echo '0b101' | wspace readi_echo.ws && echo
echo '+5' | wspace readi_echo.ws && echo
