fn main() {
    println!("Пирамидальная сортировка:");
    let mut arr = [1, 8, 65, 32];
    println!("Исходный массив:");
    arr_display(&arr);
    heap_sort(&mut arr);
    println!("\nОтсортированный массив:");
    arr_display(&arr)
}

// Построение кучи
fn heapify(arr: &mut [i32], n: usize, index: usize) {
    let mut largest = index; // Наибольший элемент - корень
    let left = (2 * index + 1) as usize; // левый элемент = 2 * i + 1
    let right = (2 * index + 2) as usize; // правый элемент = 2 * i + 2

    // Если левый элемент больше корня
    if left < n && arr[left] > arr[largest] {
        largest = left
    }

    // Если правый больше корня
    if right < n && arr[right] > arr[largest] {
        largest = right
    }

    // Если больший элемент не корень
    if largest != index{
        swap(arr, largest, index);
        heapify(arr, n, largest)
    }
}

fn swap(arr: &mut [i32], first: usize, second: usize) {
    let tmp = arr[first];
    arr[first] = arr[second];
    arr[second] = tmp
}

// основная функция сортировки
fn heap_sort(arr: &mut [i32]) {
    // построение кучи
    for i in (0..= arr.len() / 2 - 1).rev() {
        heapify(arr, arr.len(), i)
    }
    // Поэлементное извлечение элементов из кучи
    for i in (0..= arr.len() - 1).rev() {
        // Перемещаем текущий корень в конец
        swap(arr, 0, i);
        // Уменьшаем кучу и перестраиваем
        heapify(arr, i, 0)
    }
}

// Вывод элементов массива
fn arr_display(arr: &[i32]) {
    for element in arr {
        print!("{} ", element)
    }
}