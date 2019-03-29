## 0.2.0 (TBD)
* Add _version_check_ as build dependency and a build.rs script to fail fast on
  attempt to compile with a rustc below MSRV, which remains 1.31.0.

* Allow and ignore a trailing comma in arguments to _-v_ macros on rustc
  1.32.0+, made reasonable via _version_check_ and _RFC 2298_. On 1.31.0, N
  trailing commas are now allowed, as a superset. Thus tao-log 0.2.0 on 1.31.0
  is most lenient, followed by 0.2.0 on 1.32.0+, and 0.1.0 is least lenient.
  Since tao-log 0.2.0 is more lenient than 0.1.0, for all rustc 1.31.0+, this
  is nominally a backward compatible change, though its included with a
  non-semver-compatible MINOR bump just in case.

## 0.1.0 (2019-3-5)
* Initial release.
