# hidestr
---------
I just wanted an excuse to write Rust.

This tool allows you to remove all comments from a file. 
Originally, I intended to only use it for ps1 files, but ended up just extending the flexability of the tokens, so it could also be used for other languages.
At the moment, it's only meant to be used with `.ps1` files.
(It also works with languages that use C-style comments like c++, c#, etc, but this doesn't really make any difference since comments aren't included when compiling, I just felt like adding it lol.

This tool should be able to identify:
- `#` being the start of a comment
- `<#` being the start of a comment block, and `#>` being the end of the block
- `#require` as being a require statement
- When it's currently inside of a string, and shouldn't remove anything

It also works on directories recursively, so you could remove all the comments from an entire project. You can specify where to save the output to using the `-o` flag, as well as `-r` to specify that you want to perform the comment removal recursively on a directory.
You can also use `-e` to specify the file extension to target.
