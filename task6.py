def number_of_positive_divisors(n):
	counter = 0
	for i in range(n, 0, -1):
		if (n % i == 0):
			counter+=1
	return counter

n = input("Enter a positive integer: ")
n = int(n)
print(number_of_positive_divisors(n))