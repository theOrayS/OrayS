# Blacklist change report

No blacklist changes were made for stable956.

- Promotion evidence came from targeted RV/LA x musl/glibc parser-clean LTP runs only.
- No `LTP_BLACKLIST`, guest `/ltp_blacklist.txt`, or `/tmp/ltp_blacklist.txt` change contributed to the 50 promoted cases.
- Cases left out because of visible blockers (for example `mq_notify01`, `mq_notify03`, `pidfd_send_signal03`, `tgkill01`, `rt_sigqueueinfo01`) remain ordinary blockers, not blacklist PASS.
