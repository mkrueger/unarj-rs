# unarj-rs
A library for rust which supports reading of ARJ files.

Supported compression methods:

* STORE
* Method 1-3
* Method 4 (fastest)

This library was written for my bbs project as part of the file analyzation.
ARJ was popular in the BBS scene in the 90' before RAR showed up.

All advanced ARJ features are not supported like multiple archives, password protection etc.
The scope is limited to what I need. Feel free to add features you need.
(In case I overlook  the issues/PRs here contact me on https://github.com/mkrueger/icy_board or per mail)

With version 0.2.0 the project is likely to be 'finished' from my side. I don't need decryption or mulitple archives/chapters. 
But I'll surely take contributions.

# USAGE

See examples. 

``` cargo run --example view <ARJ_FILE> ```

or

``` cargo run --example extract <ARJ_FILE> ```

# LICENSE

MIT or Apache-2.0 but I don't really care :)