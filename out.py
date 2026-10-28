from functools import cache
@cache
def fibonacci(number):
	if number == 0:
		return 1

	if number == 1:
		return 1

	return (fibonacci((number-1))+fibonacci((number-2)))

number=int(input())
print(fibonacci(number))

