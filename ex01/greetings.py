"""EX01 Greetings."""

"""
3. GreetingsGreetingsGreetings
Example output:

Enter a greeting: Hello
Enter a recipient: world
How many times to repeat: 3
Hello world! Hello world! Hello world!
"""

greeting = input("Enter a greeting: ")
recipient = input("Enter a recipient: ")
repeat = int(input("How many times to repeat? "))

greetings = (greeting + ' ' + recipient + '!' ' ') * repeat

print(greetings)
