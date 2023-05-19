"""Cha-ching."""

amount = int(input())
coins = 0

count_of_50 = amount // 50
amount -= 50 * count_of_50

count_of_20 = amount // 20
amount -= 20 * count_of_20

count_of_10 = amount // 10
amount -= 10 * count_of_10

count_of_5 = amount // 5
amount -= 5 * count_of_5

count_of_1 = amount // 1
amount -= 1 * count_of_1

coins = count_of_50 + count_of_20 + count_of_10 + count_of_5 + count_of_1

print(f"Amount of coins needed: {coins}")
