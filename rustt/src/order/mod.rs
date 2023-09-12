pub struct Solution;

impl Solution {
    // 冒泡
    pub fn bobble_sort(values: &mut [i32]) -> &[i32] {
        for i in 0..values.len() - 1 {
            let mut cur_index = 1;
            while cur_index < values.len() - i {
                // 如果当前值小于上一个值，就交换
                if values[cur_index - 1] > values[cur_index] {
                    values.swap(cur_index - 1, cur_index);
                }
                cur_index += 1;
            }
        }
        values
    }

    //选择
    pub fn selection_sort(values: &mut [i32]) -> &[i32] {
        for i in 0..values.len() - 1 {
            let (mut min_index, mut cur_index) = (i, i + 1);
            // 找到最小值的索引
            while cur_index < values.len() {
                if values[min_index] > values[cur_index] {
                    min_index = cur_index;
                }
                cur_index += 1;
            }
            values.swap(min_index, i);
        }
        values
    }

    //插入
    pub fn insertion_sort(values: &mut [i32]) -> &[i32] {
        for i in 1..values.len() {
            let mut cur_index = i;
            // 只要当前值小于上一个值，就交换
            while cur_index > 0 && values[cur_index - 1] > values[cur_index] {
                values.swap(cur_index - 1, cur_index);
                cur_index -= 1;
            }
        }
        values
    }

    //希尔
    pub fn shell_sort(values: &mut [i32]) -> &[i32] {
        let mut gap = values.len() / 2;
        while gap > 0 {
            // 分组，并对分完组的每个组进行插入排序
            for i in gap..values.len() {
                let mut cur_index = i;
                while cur_index >= gap && values[cur_index] < values[cur_index - gap] {
                    values.swap(cur_index, cur_index - gap);
                    cur_index -= gap;
                }
            }
            gap /= 2;
        }
        values
    }

    //归并
    pub fn merge_soft(values: &[i32]) -> Vec<i32> {
        if values.len() <= 1 {
            return values.to_vec();
        }
        let mid = values.len() / 2;
        let (left, right) = values.split_at(mid);
        Solution::merge(&Solution::merge_soft(left), &Solution::merge_soft(right))
    }

    pub fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
        let mut result = Vec::with_capacity(left.len() + right.len());
        let (mut left_index, mut right_index) = (0, 0);

        // 比较两个数组的值，小的放入结果数组中
        // 当其中一个放满，则结束循环
        while left_index < left.len() && right_index < right.len() {
            if left[left_index] < right[right_index] {
                result.push(left[left_index]);
                left_index += 1;
            } else {
                result.push(right[right_index]);
                right_index += 1;
            }
        }

        while left_index < left.len() {
            result.push(left[left_index]);
            left_index += 1;
        }

        while right_index < right.len() {
            result.push(right[right_index]);
            right_index += 1;
        }

        result
    }

    // 快速排序
    pub fn quick_sort(values: &mut [i32]) {
        if values.len() <= 1 {
            return;
        }
        let pivot = Self::partition(values);
        Self::quick_sort(&mut values[..pivot]);
        Self::quick_sort(&mut values[pivot + 1..]);
    }

    pub fn partition(arr: &mut [i32]) -> usize {
        let (mut left, mut right) = (0, arr.len() - 1);
        let mut pivot_index = Self::median(&arr, left, right, (left + right) / 2);
        // 交换基准值到最右边
        arr.swap(pivot_index, right);
        let pivot = arr[right];
        // i 用来记录最左侧的大于基准值的索引，如果有的话；
        // 否则，i 与 j 相等
        let mut i = left;
        for j in left..right {
            // 将小于基准值的放到左边
            if arr[j] < pivot {
                arr.swap(i, j);
                i += 1;
            }
        }
        // 将基准值返回到中间
        arr.swap(i, right);
        i
    }

    pub fn median(nums: &[i32], left: usize, right: usize, mid: usize) -> usize {
        if (nums[left] > nums[right]) ^ (nums[left] > nums[mid]) {
            left
        } else if (nums[right] > nums[left]) ^ (nums[right] > nums[mid]) {
            right
        } else {
            mid
        }
    }

    // 堆排序
    pub fn head_sort(values: &mut [i32]) -> &[i32] {
        // 构建完成一个大根堆，从最后一个节点开始
        for i in (0..values.len() / 2).rev() {
            Self::heapify(values, values.len(), i);
        }
        // println!("values: {:?}", values);

        for i in (1..values.len()).rev() {
            values.swap(0, i);

            // 交换后，重新构建大顶堆，将最大的值放到索引 0 位置
            Self::heapify(values, i, 0);
        }

        values
    }

    pub fn heapify(values: &mut [i32], n: usize, i: usize) {
        // 假设最大值的索引为 i，分别为堆堆左子节点和右子节点的索引
        let (mut largest, left, right) = (i, 2 * i + 1, 2 * i + 2);

        if left < n && values[largest] < values[left] {
            largest = left;
        }

        if right < n && values[largest] < values[right] {
            largest = right;
        }

        if largest != i {
            values.swap(largest, i);
            // 递归调用，确保交换后的子节点也是大根堆
            Self::heapify(values, n, largest);
        }
    }

    // 桶排序
    /// 例如，输入数据包含 100 万个元素，由于空间限制，系统内存无法一次性加载所有数据。
    pub fn bucket_sort(values: &mut [i32]) -> &[i32] {
        let (&max, &min) = (values.iter().max().unwrap(), values.iter().min().unwrap());
        // 以 10 为间隔创建桶
        let mut buckets = vec![vec![]; ((max - min) / 10 + 1) as usize];

        // 每两个数创建一个桶
        for &value in values.iter() {
            let index = (value - min) as usize / 10;
            buckets[index].push(value);
        }

        for bucket in buckets.iter_mut() {
            // 降序排列
            bucket.sort_by(|a, b| b.cmp(a));
        }

        let mut i = 0;
        for bucket in buckets.iter_mut() {
            // 重用原数组
            while let Some(num) = bucket.pop() {
                values[i] = num;
                i += 1;
            }
        }
        values
    }

    // 计数排序
    /// 计数排序适用于数据量大但数据范围较小的情况。
    pub fn counting_sort(values: &mut [i32]) -> &[i32] {
        let max_value = *values.iter().max().unwrap() as usize;
        // 创建一个计数数组，索引为值，值为出现的次数
        let mut counter = vec![0; max_value + 1];
        for &value in values.iter() {
            counter[value as usize] += 1;
        }
        let mut j = 0;
        // i 为之前的值，count 为出现的次数
        for (i, &count) in counter.iter().enumerate() {
            for _ in 0..count {
                values[j] = i as i32;
                j += 1;
            }
        }
        values
    }

    // 基数排序
    /// 基数排序可以避免基数排序的缺点，但是需要额外的空间。
    pub fn radix_sort(values: &mut [i32]) -> &[i32] {
        let max_value = *values.iter().max().unwrap();
        let mut exp = 1;
        while exp < max_value {
            Self::counting_sort_for_radix(values, exp);
            exp *= 10;
        }
        values
    }

    pub fn counting_sort_for_radix(values: &mut [i32], position: i32) {
        // 0~9, 总共 10 个桶
        let mut counter = vec![0; 10];
        let mut output = vec![0; values.len()];
        // 计算每个桶的元素个数
        for &value in values.iter() {
            let index = (value / position) % 10;
            counter[index as usize] += 1;
        }

        ///for i in 1..output.len() {
        ///    if output.len() > counter.len() {
        ///        break;
        ///    }
        ///    counter[i] += counter[i - 1];
        ///}
        // 计算小于等于当前索引的元素个数
        // 其实增加到最大的位就可以了，但是这里为了方便，增加到了最大索引
        for i in 1..counter.len() {
            counter[i] += counter[i - 1];
        }

        let mut i = values.len() as isize - 1;
        // 从后往前遍历，保证排序的稳定性
        while i >= 0 {
            // 计算当前值在哪个桶中
            let index = (values[i as usize] / position) % 10;
            // 这里是为什么之前计算所有小于当前索引的关键
            // 这里可以得出 values[i] 在 output 中的索引位置
            output[counter[index as usize] - 1] = values[i as usize];
            counter[index as usize] -= 1;
            i -= 1;
        }

        for i in 0..values.len() {
            values[i] = output[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bobble_sort() {
        let mut values = vec![3, 6, 4, 2, 11, 10, 5];
        assert_eq!(
            Solution::bobble_sort(&mut values),
            vec![2, 3, 4, 5, 6, 10, 11]
        );
    }

    #[test]
    fn test_section_sort() {
        let mut values = vec![3, 6, 4, 2, 11, 10, 5];
        assert_eq!(
            Solution::selection_sort(&mut values),
            vec![2, 3, 4, 5, 6, 10, 11]
        );
    }

    #[test]
    fn test_insection_sort() {
        let mut values = vec![3, 6, 4, 2, 11, 10, 5];
        assert_eq!(
            Solution::selection_sort(&mut values),
            vec![2, 3, 4, 5, 6, 10, 11]
        )
    }

    #[test]
    fn test_shell_sort() {
        let mut values = vec![3, 6, 4, 2, 11, 10, 5];
        assert_eq!(
            Solution::shell_sort(&mut values),
            vec![2, 3, 4, 5, 6, 10, 11]
        );
    }

    #[test]
    fn test_merge_sort() {
        let values = vec![3, 6, 4, 2, 11, 10, 5];
        assert_eq!(Solution::merge_soft(&values), vec![2, 3, 4, 5, 6, 10, 11]);
    }

    #[test]
    fn test_quick_sort() {
        let mut values = vec![3, 6, 4, 2, 11, 10, 5];
        Solution::quick_sort(&mut values);
        assert_eq!(values, vec![2, 3, 4, 5, 6, 10, 11]);
    }

    #[test]
    fn test_heap_sort() {
        let mut values = vec![3, 6, 4, 2, 11, 10, 5];
        assert_eq!(
            Solution::head_sort(&mut values),
            vec![2, 3, 4, 5, 6, 10, 11]
        );
    }

    #[test]
    fn test_bucket_sort() {
        let mut values = vec![3, 6, 4, 2, 11, 10, 5];
        assert_eq!(
            Solution::bucket_sort(&mut values),
            vec![2, 3, 4, 5, 6, 10, 11]
        );
    }

    #[test]
    fn test_counting_sort() {
        let mut values = vec![3, 6, 4, 2, 11, 10, 5];
        assert_eq!(
            Solution::counting_sort(&mut values),
            vec![2, 3, 4, 5, 6, 10, 11]
        );
    }
    #[test]
    fn test_radix_sort() {
        let mut values = vec![3, 6, 4, 2, 11, 10, 5];
        assert_eq!(
            Solution::radix_sort(&mut values),
            vec![2, 3, 4, 5, 6, 10, 11]
        );
    }
}
