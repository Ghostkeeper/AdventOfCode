# Advent of code 2022, by Ghostkeeper

def snafu_to_decimal(snafu):
	"""
	Convert a SNAFU number to a decimal number.
	:param snafu: The SNAFU number to convert, as a string.
	:return: The decimal number that the SNAFU number represents.
	"""
	sum = 0
	for i, char in enumerate(snafu):
		place = pow(5, len(snafu) - 1 - i)
		if char == "2":
			sum += 2 * place
		elif char == "1":
			sum += 1 * place
		elif char == "0":
			sum += 0 * place
		elif char == "-":
			sum += -1 * place
		elif char == "=":
			sum += -2 * place
		else:
			raise ValueError(f"Unknown SNAFU character: {char}")
	return sum

def decimal_to_snafu(decimal):
	"""
	Convert a decimal number to a SNAFU number.
	:param decimal: The decimal number to convert, as an integer.
	:return: The SNAFU number representing that decimal number.
	"""
	if decimal == 0:
		return "0"
	result = ""
	place = 1
	while decimal != 0:
		remainder = decimal % (5 * place)
		if remainder == 0 * place:
			result = "0" + result
		elif remainder == 1 * place:
			result = "1" + result
			decimal -= place
		elif remainder == 2 * place:
			result = "2" + result
			decimal -= 2 * place
		elif remainder == 3 * place:
			result = "=" + result
			decimal += 2 * place
		elif remainder == 4 * place:
			result = "-" + result
			decimal += 1 * place
		place *= 5
	return result