"""EX01 Poem."""

"""
2. Poem
Example output:

Roses are red,
violets are blue,
I love to code
And so will you!

"""
color = input()
objects = input()
activity = input()

first_line = f"Roses are {color},"
second_line = f"{objects} are blue,"
third_line = f"I love to {activity},"
fourth_line = "And so will you!"

print(f"{first_line}\n"
      f"{second_line}\n"
      f"{third_line}\n"
      f"{fourth_line}\n")
