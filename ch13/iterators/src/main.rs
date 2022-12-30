fn main() {}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoe: Vec<Shoe>, my_size: u32) -> Vec<Shoe> {
    shoe.into_iter().filter(|s| s.size == my_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }

    #[test]
    fn iterator_next() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        println!("{:?}", v1);
        println!("{:?}", v1_iter);

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

        let mut v1_iter_type = v1.into_iter();

        assert_eq!(v1_iter_type.next(), Some(1));
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6)
    }

    #[test]
    fn iterator_map() {
        let v1 = vec![1, 2, 3];

        let v2: Vec<_> = v1.iter().map(|y| y * 100).collect();

        assert_eq!(v2, vec![100, 200, 300])
    }
}
