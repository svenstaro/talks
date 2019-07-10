# Notes

- Traditional tools are great for scripts and you should keep using them
- They are not so great for every day interactive use which is where more modern tools shine

## exa

- Feels a lot like ls and has similar options
- ls
- exa
- ls -l
- exa -l
- exa -l --git
- tree
- tree -Duhp
- exa --tree -l --git

## fd

- Let's find all READMEs in cpython
- find ~/src/cpython -iname '*readme*'
- fd readme ~/src/linux

- Let's find all Python files in cpython
- find ~/src/cpython -name '*parser*.py'
- fd parser -e py ~/src/cpython
- fd parser -e py -e rst ~/src/cpython

- Ignores files ignored by git by default
- fd ignored
- fd ignored -u

## ripgrep

- Ultra fast and convenient replacement for grep
- git-ignore aware
- Ignores binary files by default
- Simple comparison
- grep -r parser
- rg parser

- Let's do a heavy duty comparison
- grep -rIi buffer ~/src/linux
- rg -i buffer ~/src/linux

- Let's only search Python files in linux
- rg -tpy -i buffer ~/src/linux

## tokei

- cloc ~/src/cpython
- tokei ~/src/cpython
- Let's see how much Python code I have in all of my sources
- tokei -t C ~/src

## httpie

- Like curl but much easier to use and much prettier
- curl localhost:8080
- http localhost:8080
- Let's look at headers and stuff
- curl -v localhost:8080
- httpie shows response headers by default
- Let's send some JSON
- curl -d '{"hello":"world"}' -H "Content-Type: application/json" -X POST localhost:8080
- http localhost:8080 hello=world
- http -pHhBb localhost:8080 hello=world

## bat

- Like if cat and less had a magical unicorn baby
- cat apihelper.py
- less apihelper.py
- bat apihelper.py
- It has git support (look at markings in file)
- I won't display binary data by default
- bat taxi-simulation

## sd

- Like sed but simpler for trivial use cases
- bat changed_file
- sed -i "s/replace/lol/g" changed_file
- sd lol rofl -p changed_file
- sd lol rofl changed_file
- bat changed_file

## hyperfine

- Accurately measure short programs
- time taxi-simulation
- Hard to see the accurate actual time
- hyperfine taxi-simulation

## bonus: genact

- Pretend to be compiling the kernel instead of actually doing it
- genact -m cc
- genact -m weblog

## bonus: asciiquarium

## bonus: cmatrix
