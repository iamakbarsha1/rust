Here are the Python programs for each problem statement:

1. Python Program to Implement Arithmetic Operations

a = float(input("Enter first number: "))
b = float(input("Enter second number: "))

print("Addition:", a + b)
print("Subtraction:", a - b)
print("Multiplication:", a * b)
print("Division:", a / b if b != 0 else "Undefined (division by zero)")
print("Modulus:", a % b if b != 0 else "Undefined (modulus by zero)")

2. Python Program to Check if a Number is Even

num = int(input("Enter a number: "))
if num % 2 == 0:
    print(f"{num} is even")
else:
    print(f"{num} is odd")

3. Python Program for Fibonacci Series using Conditional Statements

n = int(input("Enter the number of terms: "))
a, b = 0, 1

if n <= 0:
    print("Please enter a positive integer.")
elif n == 1:
    print("Fibonacci Series:", a)
else:
    print("Fibonacci Series:", end=" ")
    for _ in range(n):
        print(a, end=" ")
        a, b = b, a + b

4. Python Program to Convert Celsius to Fahrenheit

celsius = float(input("Enter temperature in Celsius: "))
fahrenheit = (celsius * 9/5) + 32
print(f"Temperature in Fahrenheit: {fahrenheit}")

5. Python Program to Reverse a String using a User-Defined Function

def reverse_string(s):
    return s[::-1]

text = input("Enter a string: ")
print("Reversed string:", reverse_string(text))

6. Python Program to Check if a Number is a Multiple of Both 5 and 7

num = int(input("Enter a number: "))
if num % 5 == 0 and num % 7 == 0:
    print(f"{num} is a multiple of both 5 and 7.")
else:
    print(f"{num} is not a multiple of both 5 and 7.")

7. Python Program to Implement FIZZBUZZ Logic

num = int(input("Enter a number: "))
if num % 3 == 0 and num % 5 == 0:
    print("FIZZBUZZ")
elif num % 3 == 0:
    print("FIZZ")
elif num % 5 == 0:
    print("BUZZ")
else:
    print(num)

8. Python Program to Find the Geometric Mean of n Numbers

import math

n = int(input("Enter the number of elements: "))
nums = [float(input(f"Enter number {i+1}: ")) for i in range(n)]

product = math.prod(nums)
geometric_mean = product ** (1/n)

print("Geometric Mean:", geometric_mean)

9. Python Program to Compute Mean After Removing First and Last 10% of Data

import numpy as np

costs = sorted([float(input(f"Enter cost {i+1}: ")) for i in range(int(input("Enter number of costs: ")))])

trim = int(len(costs) * 0.1)  
filtered_costs = costs[trim:-trim] if trim > 0 else costs  

mean_value = np.mean(filtered_costs)
print("Mean after trimming:", mean_value)

10. Python Program to Find Sum of Digits of an Integer using While Loop

num = int(input("Enter an integer: "))
sum_digits = 0

while num > 0:
    sum_digits += num % 10
    num //= 10

print("Sum of digits:", sum_digits)

11. Python Program to Generate 50 Random Birth Dates and Find Duplicates

import random
from collections import Counter

birthdays = [random.randint(1, 365) for _ in range(50)]
count = Counter(birthdays)

duplicates = {day: freq for day, freq in count.items() if freq > 1}
print(f"Duplicate birthdays: {duplicates}")
print(f"Total duplicate days: {len(duplicates)}")

12. Python Program to Display Multiples of 3 within Range 10 to 15

print("Multiples of 3 in range 10 to 15:")
for i in range(10, 16):
    if i % 3 == 0:
        print(i)
