use std::collections::HashMap;

pub fn expense_report(nums: &[i64], target: i64) -> Option<i64> {
    let mut hash: HashMap<i64, i64> = HashMap::new();
    for i in nums {
        if let Some(j) = hash.get(i) {
            return Some(j * i);
        } else {
            hash.insert(target - i, *i);
        }
    }
    None
}

pub fn triple_expense(nums: &[i64], target: i64) -> Option<i64> {
    for i in nums {
        let mut hash: HashMap<i64, i64> = HashMap::new();
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
    }

    #[test]
    fn expense_report_out_of_3_values() {
        let res = triple_expense(&vec![1721, 979, 366, 299, 675, 1456], 2020);
        assert_eq!(res, Some(241861950));
    }
}
