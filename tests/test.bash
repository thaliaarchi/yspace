#!/bin/bash

wspace bad_filename.ws
wspace empty_file.ws
wspace implicit_end.ws

wspace underflow_dup.ws
wspace underflow_copy.ws
wspace underflow_copy_unused.ws
wspace underflow_swap.ws
wspace underflow_drop.ws
wspace underflow_slide.ws
wspace underflow_add.ws
wspace underflow_sub.ws
wspace underflow_mul.ws
wspace underflow_div.ws
wspace underflow_mod.ws
wspace underflow_store.ws
wspace underflow_retrieve.ws
wspace underflow_jz.ws
wspace underflow_jn.ws
wspace underflow_ret.ws
wspace underflow_printc.ws
wspace underflow_printi.ws
wspace underflow_readc.ws
wspace underflow_readi.ws

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
