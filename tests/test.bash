#!/bin/bash

wspace bad_filename.ws
wspace empty_file.ws
wspace implicit_end.ws

wspace encoding_utf8_bom.ws && echo
wspace encoding_invalid_utf8.ws && echo
wspace encoding_invalid_utf8_lazy.ws && echo

wspace unknown_instruction_stt.ws
wspace unknown_instruction_tstl.ws
wspace unknown_instruction_tsl.ws
wspace unknown_instruction_ttl.ws
wspace unknown_instruction_tlsl.ws
wspace unknown_instruction_tltl.ws
wspace unknown_instruction_tll.ws
wspace unknown_instruction_lls.ws
wspace unknown_instruction_llt.ws

wspace unterminated_push.ws
wspace unterminated_copy.ws
wspace unterminated_slide.ws
wspace unterminated_label.ws
wspace unterminated_call.ws
wspace unterminated_jmp.ws
wspace unterminated_jz.ws
wspace unterminated_jn.ws

wspace underflow_dup.ws
wspace underflow_swap.ws
wspace underflow_drop.ws
wspace underflow_slide.ws
wspace underflow_slide_zero.ws
wspace underflow_slide_negative.ws
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
wspace push_zero_empty_unused.ws
wspace push_zero_sign_only.ws

wspace copy_zero_empty.ws
wspace copy_zero_empty_unused.ws
wspace copy_zero_sign_only.ws
wspace copy_negative.ws
wspace copy_negative_unused.ws
wspace copy_error.ws
wspace copy_error_unused.ws

wspace slide_zero.ws && echo
wspace slide_zero_empty.ws && echo
wspace slide_zero_sign_only.ws && echo
wspace slide_negative.ws && echo
wspace slide_less.ws && echo

wspace div_zero.ws
wspace div_zero_unused.ws
wspace mod_zero.ws
wspace mod_zero_unused.ws

wspace retrieve_unset.ws && echo
wspace retrieve_unset_unused.ws
wspace retrieve_unset_less.ws && echo

wspace leading_zero_labels.ws && echo

wspace undefined_label_call.ws
wspace undefined_label_jmp.ws
wspace undefined_label_jz.ws
wspace undefined_label_jn.ws

wspace printc_negative.ws
wspace printc_too_large.ws

wspace printi_negative_zero.ws && echo
wspace printi_long_number_334_bits.ws && echo
wspace printi_long_number_54_bits.ws && echo

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

wspace printc_all_surrogates.ws > /dev/null
# wspace printc_all_codepoints.ws > /dev/null # slow
