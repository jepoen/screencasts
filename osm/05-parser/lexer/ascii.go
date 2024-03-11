package lexer

func isAsciiDigit(r rune) bool {
	return r >= '0' && r <= '9'
}

func isAsciiSign(r rune) bool {
	return r == '+' || r == '-'
}

func isAsciiDigitOrSign(r rune) bool {
	return isAsciiDigit(r) || isAsciiSign(r)
}

func isAsciiLetter(r rune) bool {
	return (r >= 'a' && r <= 'z') || (r >= 'A' && r <= 'Z') || r == '_'
}

func isAsciiStrDelim(r rune) bool {
	return r == '\'' || r == '"' || r == '`'
}
