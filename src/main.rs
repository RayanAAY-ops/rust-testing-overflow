use std::i8;

#[derive(Debug, Clone, Copy)]
struct MyNumber {
    value: i8,
    max_value: i8,
    min_value: i8,
}

impl MyNumber {
    fn wrapping_add(self, other: i8) -> MyNumber {
        let new_value = self.value.wrapping_add(other);
        MyNumber {
            value: new_value,
            ..self
        }
    }

    fn checked_add(self, other: i8) -> Option<MyNumber> {
        self.value.checked_add(other).map(|new_value| MyNumber {
            value: new_value,
            ..self
        })
    }

    fn overflowing_add(self, other: i8) -> (MyNumber, bool) {
        let (new_value, overflowed) = self.value.overflowing_add(other);
        (
            MyNumber {
                value: new_value,
                ..self
            },
            overflowed,
        )
    }

    fn saturating_add(self, other: i8) -> MyNumber {
        let new_value = self.value.saturating_add(other);
        MyNumber {
            value: new_value,
            ..self
        }
    }
}

fn main() {
    let num = MyNumber {
        value: 120,
        max_value: i8::MAX,
        min_value: i8::MIN,
    };

    let wrapped = num.wrapping_add(120);
    println!("Wrapping add {:?}", wrapped);

    let checked = num.checked_add(120);
    println!("checked add with overflow {:?}", checked);

    let checked = num.checked_add(2);
    println!("checked add without overflow {:?}", checked);

    let overflowed = num.overflowing_add(100);
    println!("Overflowed add{:?}", overflowed);

    let saturated = num.saturating_add(100);
    println!("Saturated add{:?}", saturated);
}
