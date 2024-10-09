# temperature = 35

# if temperature > 30:
#     print("It's a hot day")
#     print("Drink plenty of water")

temperature = 25

if temperature > 30:
    print("It's a hot day")
    print("Drink plenty of water")
elif temperature > 20: # (20, 30]
    print("It's a nice day")
elif temperature > 10: # If the condition is true then the temperature is greater than 10 and less than or equal to 20 (10, 20]
    print("It's a bit cold")
else:
    print("It's cold")
print("Done")