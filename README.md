# YUNODOC 2

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