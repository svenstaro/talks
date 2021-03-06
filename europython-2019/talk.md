%title: Become a command line wizard
%author: Sven-Hendrik Haase
%date: 2019-07-10

-> Become a command line wizard
===============================

-> Learn modern terminal tools for a faster CLI experience
----------------------------------------------------------

---

-> This talk
============

^
* I'll show you a comparison of old and new tools
^
* You don't need to know any of the tools
^
* Basically only live demos, *what could go wrong*
^
* Final take-away for you: cool new tools

---

-> About me
===========

* Command line enthusiast   *\\o
* Arch Linux developer      * |\\
* DevOps consultant         */ \\

---

-> Why bother with the command line in 2019?
============================================

^
* Command line is not going away
^
* It's efficient for many kinds of tasks
^
* Sometimes you can't use a GUI
^
* Text is still the only truly universal exchange format
^

-> *These slides themselves are on the command line!* :D

---

-> Old tools and new tools
==========================

^
Many fancy modern tools

^
_*Task                    Traditional     Modern*_
Listing files           ls/tree         exa
Finding files           find            fd
Searching in fles       grep            ripgrep
Counting LoC            cloc            tokei
Making HTTP reqs        curl            httpie
Show file contents      cat/less        bat
Replace text            sed             sd
Benchmarking/timing     time            hyperfine
Presenting/slides       PowerPoint      mdp
Pretending to do work   compiling Linux genact

^
Let's have a look!

---

-> exa - Listing files
======================

[exa](https://github.com/ogham/exa)

Like `ls` or `tree`, but
* Pretty
* git support

---

-> fd - Finding files
=====================

[fd](https://github.com/sharkdp/fd)

Like `find`, but
* Fast
* User friendly
* Good default ignores

---

-> ripgrep - Searching in files
===============================

[ripgrep](https://github.com/BurntSushi/ripgrep)

Like `grep`, but
* *Insanely* fast
* User friendly
* Good default ignores

---

-> tokei - Counting lines of code
=================================

[tokei](https://github.com/XAMPPRocky/tokei)

Like `cloc`, but
* Fast
* User friendly
* Good default ignores

---

-> httpie - Making HTTP requests
================================

[httpie](https://httpie.org/)

Like `curl`, but
* *Very* user friendly
* Pretty output
* Automatically formats responses

---

-> bat - Show file contents
===========================

[bat](https://github.com/sharkdp/bat)

Like `cat` or `less`, but has
* Syntax highlighting support
* Automatic pager for long files
* git support

---

-> sd - Replace text
====================

[sd](https://github.com/chmln/sd)

Like `sed`, but
* User friendly

---

-> hyperfine - Benchmarking
===========================

Like `time`, but has
* Initial warm-up phase
* Multiple runs for better overall measurements
* Also pretty *:D*

[hyperfine](https://github.com/sharkdp/hyperfine)

---

-> Bonus: mdp - Presentation tool
=================================

Like PowerPoint, but
* with 1% of the features
* on the command line

---

-> Bonus: genact - Pretending to do work
========================================

* Pretends to be a legitimate tool
* Actually doesn't really do anything

[genact](https://github.com/svenstaro/genact)

---

-> That's pretty much it
========================

Thanks! \\(^-^)/

If you want to reach out:
svenstaro@gmail.com
https://svenstaro.org/
