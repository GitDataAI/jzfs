package utils

func Silent[T any](val T, _ error) T {
	return val
}

func Silent2[T any, T2 any](val1 T, val2 T2, _ error) (T, T2) {
	return val1, val2
}

func Silent3[T any, T2 any, T3 any](val1 T, val2 T2, val3 T3, _ error) (T, T2, T3) {
	return val1, val2, val3
}
