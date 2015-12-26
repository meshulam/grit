# grit
Minimal Git-like file store in Rust. I wanted to learn Rust and also learn more about the 
internals of Git, so I decided to kill two birds with one stone. And here we are.

Commands:

- grit init -- initialize the object db in `./.grit`
- grit hash-object <file> -- Add the file to the store and print the SHA hash
- grit cat-file <sha> -- Print the raw representation of the object with the given SHA hash
- grit help -- print these commands
