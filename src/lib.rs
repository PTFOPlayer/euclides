#[derive(Debug)]
pub struct EucRes {
    pub d: i32,
    pub s: i32,
    pub t: i32
}

pub fn euc_ext(d1:i32, d2:i32) -> EucRes {
    let mut d = vec![d1, d2];
    let mut s = vec![1,0];
    let mut t = vec![0,1];
    let mut qp = d1/d2;
    
    let mut i = 2;
    loop {
        d.append(&mut vec![ d[i-2] % d[i-1] ]);
        if d[i] == 0 {break;}
        let q = d[i-1] / d[i]; 
        s.append(&mut vec![ s[i-2] - (qp* s[i-1]) ]);
        t.append(&mut vec![ t[i-2] - (qp* t[i-1]) ]);
        qp = q;
        i += 1;
    }

    return EucRes{
        d: d[i-1],
        s: s[i-1],
        t: t[i-1]
    };
}

pub fn euc(d1:i32, d2:i32) -> i32 {
    let mut d = vec![d1, d2];
    let mut i = 2;
    loop {
        d.append(&mut vec![ d[i-2] % d[i-1] ]);
        if d[i] == 0 {break;};
        i += 1;
    }
    i-=1;
    d[i]
}