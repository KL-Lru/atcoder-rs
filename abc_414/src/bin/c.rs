use proconio::input;

fn main() {
    input! {
        a: usize,
        n: usize,
    }

    let digits = n.to_string().len();
    let mut palindromes = vec![];
    for i in 1..=digits {
        palindromes.append(&mut make_palindromes(i, n, a));
    }

    println!("{}", palindromes.iter().sum::<usize>());
}

fn make_palindromes(digits: usize, max: usize, radix: usize) -> Vec<usize> {
    let mut numbers = vec![0; digits / 2 + digits % 2];
    let len = numbers.len();
    if len != 0 {
        numbers[0] = 1;
    }

    let mut result = vec![];

    loop {
        let num = to_palindrome(&numbers, digits % 2 == 1);

        if num > max {
            return result;
        }

        let ar = to_array(num, radix);
        if is_palindrome(&ar) {
            result.push(num);
        }

        numbers[len - 1] += 1;
        for i in 1..len {
            if numbers[len - i] >= 10 {
                numbers[len - i - 1] += 1;
                numbers[len - i] = 0;
            }
        }

        if numbers[0] == 10 {
            return result;
        }
    }
}

fn to_palindrome(half_numbers: &Vec<usize>, is_odd_digits: bool) -> usize {
    let mut last_half = half_numbers.clone();
    if is_odd_digits {
        last_half.pop();
    }
    last_half.reverse();

    to_number(&[half_numbers.clone(), last_half].concat())
}

fn is_palindrome(numbers: &Vec<usize>) -> bool {
    let len = numbers.len();

    for i in 0..(len / 2) {
        if numbers[i] != numbers[len - 1 - i] {
            return false;
        }
    }

    true
}

fn to_number(numbers: &Vec<usize>) -> usize {
    numbers
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, num)| acc + num * 10usize.pow(idx as u32))
}

fn to_array(mut number: usize, radix: usize) -> Vec<usize> {
    let mut result = vec![];
    while number != 0 {
        result.push(number % radix);
        number /= radix;
    }

    result
}
