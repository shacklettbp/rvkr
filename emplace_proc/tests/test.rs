use emplace_proc::emplace;

struct NewTest {
    pub i : i64
}

#[emplace]
impl NewTest {
    pub fn new(y: i64) -> NewTest {
        NewTest {
            i: y + 2
        }
    }
}

struct NewTest2 {
    pub i : i64
}

#[emplace]
impl NewTest2 {
    pub fn new(y: i64) -> NewTest2 {
        let x = {
            let z = y;
            z + 2
        };

        NewTest2 {
            i: x
        }
    }
}

struct NewTest3 {
    pub i : i64
}

#[emplace]
impl NewTest3 {
    pub fn new(y: i64) -> NewTest3 {
        let z = y + 2;
        return NewTest3 {
            i: z + 2
        }
    }
}

struct NewTest4 {
    pub i : i64
}

#[emplace]
impl NewTest4 {
    pub fn new(y: i64) -> NewTest4 {
        return NewTest4 {
            i: y + 2
        };
    }
}

struct BigStruct {
    pub i : [i64; 1000]
}

#[emplace]
impl BigStruct {
    pub fn new(y: i64) -> BigStruct {
        BigStruct {
            i: [y; 1000]
        }
    }
}

#[test]
fn basic_test() {
    let new = NewTest::new(3);
    assert_eq!(new.i, 5);

    let new2 = NewTest2::new(3);
    assert_eq!(new2.i, 5);

    let new3 = NewTest3::new(1);
    assert_eq!(new3.i, 5);

    let new4 = NewTest4::new(3);
    assert_eq!(new4.i, 5);
}

#[test]
fn vec_test() {
    let mut v : Vec<NewTest3> = vec![];
    v.push_emplace(5);
    assert_eq!(v.last().unwrap().i, 9);
}

#[test]
fn big_test() {
    let mut v : Vec<BigStruct> = vec![];
    v.push(BigStruct::new(5));
    assert_eq!(v.last().unwrap().i[0], 5);
}

#[test]
fn big_test_emplace() {
    let mut v : Vec<BigStruct> = vec![];
    v.push_emplace(5);
    assert_eq!(v.last().unwrap().i[0], 5);
}
