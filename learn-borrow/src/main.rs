fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s { s = r; }
    }

    s
}

fn extend(vec: &mut Vec<f64>, slice: &[f64]){
    for elt in slice {
        vec.push(*elt);
    }
}

fn main() {
    let r;
    {
        let x = 1;
        r = &x;
        assert_eq!(*r, 1);
    }

    let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
        assert_eq!(*s, 0);
    }

    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];

    extend(&mut wave, &head);
    extend(&mut wave, &tail);

    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);    

    //extend(&mut wave, &wave);
    //assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0,
    //                      0.0, 1.0, 0.0, -1.0]);

    let mut x = 10;
    let r1 = &x;
    let r2 = &x;
    //x += 10;  //x は借用されているので代入できない

    assert_eq!(x, 10);
    assert_eq!(*r1, 10);
    assert_eq!(*r2, 10);
}
