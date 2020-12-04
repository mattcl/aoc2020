use std::collections::HashMap;

pub fn expense_report_slow(nums: &[i32], target: i32) -> Option<i32> {
    for i in nums {
        for j in nums {
            if i != j {
                if i + j == target {
                    return Some(i * j);
                }
            }
        }
    }
    None
}

pub fn expense_report(nums: &[i32], target: i32) -> Option<i32> {
    let mut hash: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
    for i in nums {
        if let Some(j) = hash.get(i) {
            return Some(j * i);
        } else {
            hash.insert(target - i, *i);
        }
    }
    None
}

pub fn triple_expense_slow(nums: &[i32], target: i32) -> Option<i32> {
    for i in nums {
        for j in nums {
            for k in nums {
                if i + j + k == target {
                    return Some(i * j * k);
                }
            }
        }
    }
    None
}

pub fn triple_expense(nums: &[i32], target: i32) -> Option<i32> {
    for i in nums {
        let mut hash: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for j in nums {
            // Technically, we could have just called the expense function,
            // but this would result in allowing the duplicate value unless
            // we modified that function to accept a value to exclude.
            if j == i {
                continue;
            }

            if let Some(k) = hash.get(j) {
                return Some(i * j * k);
            } else {
                hash.insert(target - i - j, *j);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expense_report_satisfies_example() {
        let res = expense_report(&vec![1721, 979, 366, 299, 675, 1456], 2020);
        assert_eq!(res, Some(514579));

        let res = expense_report_slow(&vec![1721, 979, 366, 299, 675, 1456], 2020);
        assert_eq!(res, Some(514579));
    }

    #[test]
    fn expense_report_out_of_3_values() {
        let res = triple_expense(&vec![1721, 979, 366, 299, 675, 1456], 2020);
        assert_eq!(res, Some(241861950));

        let res = triple_expense_slow(&vec![1721, 979, 366, 299, 675, 1456], 2020);
        assert_eq!(res, Some(241861950));
    }
}
