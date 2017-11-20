# YUNODOC 2(.1.0)

[![Build status](https://ci.appveyor.com/api/projects/status/a007mp9tx2i1y6gh?svg=true)](https://ci.appveyor.com/project/Txuritan/yunodoc2)
[![Build Status](https://travis-ci.org/Txuritan/yunodoc2.svg?branch=v1.0.0)](https://travis-ci.org/Txuritan/yunodoc2)

Turns doc comments, ```# DOC VAR ... TYP ... PUR ...```, into a doc table.

Used during my time a college. for my programming class.

## Example

```python
# DOC VAR mFltTemp TYP float PUR a temp variable
mFltTemp = 0.0
```

with

```./yunodoc --style ian ./example.py```

to

```python
# ------------------- # -------------- # ---------------------------------- #
#     ~Variables~     #     ~Type~     #     ~Purpose~                      #
# ------------------- # -------------- # ---------------------------------- #
#     mFltTemp        #     float      #     a temp variable                #
# ------------------- # -------------- # ---------------------------------- #

# DOC VAR mFltTemp TYP float PUR a temp variable
mFltTemp = 0.0
```

Please note this will not append the table to the file its not that complex yet.

## TODO
  - [ ] Append table to file
  - [x] Detect Python vs C based (C/C++, C#, Java)
