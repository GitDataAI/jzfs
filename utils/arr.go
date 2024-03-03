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

func Reverse[T any](items []T) []T {
	for i, j := 0, len(items)-1; i < j; i, j = i+1, j-1 {
		items[i], items[j] = items[j], items[i]
	}
	return items
}

func ArrMap[T any, T2 any](items []T, mapFn func(T) (T2, error)) ([]T2, error) {
	newItems := make([]T2, len(items))
	for index, item := range items {
		newItem, err := mapFn(item)
		if err != nil {
			return nil, err
		}
		newItems[index] = newItem
	}
	return newItems, nil
}
