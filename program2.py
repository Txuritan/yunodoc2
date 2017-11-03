# Ian Cronkright
# Program2
# COMS-160
# 09/18/2017
# Calculates restaraunt bill

# ------------------- # -------------- # ----------------------------------------------- #
#     ~Variables~     #     ~Type~     #     ~Purpose~                                   #
# ------------------- # -------------- # ----------------------------------------------- #
#     mSubtotal       #     float      #      to store the bills subtotal                #
#     mSubtotal       #     float      #      to store the bills tip                     #
#     mSubtotal       #     float      #      to store the bills tax                     #
#     mSubtotal       #     float      #      to store the bills grandtotal              #
#     mSubtotal       #     float      #      to store the bills grandtotal with tip     #
# ------------------- # -------------- # ----------------------------------------------- #

# DOC VAR mSubtotal TYP float PUR to store the bills subtotal
mSubtotal = float(input("Please input the subtotal of the bill"))

# DOC VAR mSubtotal TYP float PUR to store the bills tip
mSubtatalTip = (mSubtotal * .15)

# DOC VAR mSubtotal TYP float PUR to store the bills tax
mSubtatalTax = (mSubtotal * .06)

# DOC VAR mSubtotal TYP float PUR to store the bills grandtotal
mGrandTotal = (mSubtotal + mSubtatalTax)

# DOC VAR mSubtotal TYP float PUR to store the bills grandtotal with tip
mGrandTotalWTip = (mGrandTotal + mSubtatalTax)

print("Subtotal:            ", mSubtotal)
print("Subtotal Tip:        ", mSubtatalTip)
print("Subtotal Tax:        ", mSubtatalTax)
print("Grandtotal:          ", mGrandTotal)
print("Grandtotal w/t Tip:  ", mGrandTotalWTip)