package utils

type numbers interface {
	int | int8 | int16 | int32 | int64 | float32 | float64
}

func Contain[T interface {
	numbers | ~string | ~bool
}](items []T, ele T) bool {
	for _, v := range items {
		if v == ele {
			return true
		}
	}
	return false
}
