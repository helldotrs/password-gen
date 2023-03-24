import random

chars   = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
pwl     = 16
pw      = ""


i       = 0
while (true)
    pw.append(chars[random.randint(0, len(chars))])
    # ^is this clear enough or should I seperate more steps?
    
    if (i >= pwl):
        break
        
    i += 1

print(pw)
