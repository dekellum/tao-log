## 0.2.0 (TBD)
* Add version_check build dependency and build.rs script to fail fast
  when below MSRV.
* Allow and ignore a trailing comma in use of -v macros on rustc
  1.32.0+, made possible via RFC 2298. MSRV remains 1.31.0, where a
  trailing comma remains a compile error.

## 0.1.0 (2019-3-5)
* Initial release.
