# README.md -- Scripture reference expander

This is a text expander that takes a scripture reference and expands it to a markdown link.

e.g.

```
cargo run -- "Matthew 5:48"
```

Will expand to [Matthew 5:48](https://www.churchofjesuschrist.org/study/scriptures/nt/matt/5?lang=eng&id=p48#p48)

You can install the expander and then setup a shortcut to pass selected text to the expander and the expander will output the markdown inline, replacing the selected text.  I use Keyboard Maestro to setup my shortcuts, but you could use the text expander that ships with MacOS.  

I am sure that the same could be achieved on windows with AutoHotkey or with espanso on Linux, though I have not tried it on those platforms.  Let me know if you have success on Windows and Linux.

The library assumes that the URL will point to `www.churchofjesuschrist.org/study/scriptures`, but other base URLs could be added.

Further, the books database assumes scripture references, but the library could be modified to use other lookup tables.