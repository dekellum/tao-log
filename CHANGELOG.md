## 0.2.0 (TBD)
* Add _version_check_ build dependency and build.rs script to fail fast
  when below MSRV (still 1.31.0).

* Allow and ignore a trailing comma in use of -v macros on rustc
  1.32.0+, made reasonably possible via version_check and RFC
  2298. MSRV remains 1.31.0, where N trailing commas are now allowed,
  as a superset.  Thus tao-log 0.2.0 on 1.31.0 is most lenient,
  followed by 0.2.0 on 1.32.0+, and finally by 0.1.0 on all rustc
  1.31.0+.  Since tao-log 0.2.0 is more lenient than 0.1.0, for all
  rustc 1.31.0+, this is nominally a backward compatible change,
  though its included with a non-semver-compatible MINOR bump just in
  case.

## 0.1.0 (2019-3-5)
* Initial release.
