# unarj-rs
A library for rust which supports reading of ARJ files.

Supportede compression methods:

* STORE
* Method 1-3

Not supported:
Method 4 (which is fastest compression).

This library was written for my bbs project as part of the file analyzation.
ARJ was popular in the BBS scene in the 90' before RAR showed up.

All advanced ARJ features are not supported like multiple archives, password protection etc.
The scope is limited to what I need. Feel free to add features you need.
(In case I overlook  the issues/PRs here contact me on https://github.com/mkrueger/icy_board or per mail)

# LICENSE

MIT or Apache-2.0 but I don't really care :)