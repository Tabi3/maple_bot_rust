use num::complex::Complex;

pub fn f_from_vec(input: Vec<f64>) -> Box<dyn Fn(f64) -> f64 + 'static> {
    Box::new(move |x: f64| -> f64 {
        let mut sum: f64 = input[0];
        for i in 1..(input.len())  {
            sum += input[i]*x.powf(i as f64);
            println!("{} {} {}", input[i]*x.powf(i as f64), i, input[i])
        };
        return sum;
    })
}

pub fn derivative(f: &dyn Fn(f64) -> f64, h: f64) -> Box<dyn Fn(f64) -> f64 + '_> {
    return Box::new(move |x: f64| -> f64 {(f(x+h)-f(x-h))/(2.0*h)});
}

pub fn cderivative(f: &dyn Fn(Complex<f64>) -> Complex<f64>, h: Complex<f64>) -> Box<dyn Fn(Complex<f64>) -> Complex<f64> + '_> {
    return Box::new(move |x: Complex<f64>| -> Complex<f64> {(f(x+h)-f(x-h))/(2.0*h)});
}

pub fn nroot(n: f64, degree: f64, h: f64) -> f64 {
    let mut x: f64 = 10.0;
    let f = |x: f64| -> f64 {x.powf(degree) - n};
    for _ in 1..20 {
        x = x - f(x)/derivative(&f, h)(x);
    }
    return x;
}

pub fn croot(n: Complex<f64>, degree: f64) -> Complex<f64> {
    let r = (Complex::new(1.0/degree, 0.0)*n.ln()).exp();
    let real = f64::trunc(r.re*10e12)/10e12;
    let imag = f64::trunc(r.im*10e12)/10e12;
    return Complex::new(real, imag);
}


pub fn froot(f: &dyn Fn(f64) -> f64, h: f64) -> Vec<f64> {
    let mut roots: Vec<f64> = Vec::new();
    for i in -10..10 {
        let mut x: f64 = i as f64;
        for _ in 1..32 {
            x = x-f(x)/derivative(f, h)(x);
        }
        if f(x).abs() <= 1e-9 {
            roots.push(f64::trunc(x * 1e12) / 1e12);
        }
    }
    roots.dedup();
    return roots;
}

pub fn cfroot(f: &dyn Fn(Complex<f64>) -> Complex<f64>, h: Complex<f64>) -> Vec<Complex<f64>> {
    let mut roots: Vec<Complex<f64>> = Vec::new();
    for i in -10..10 {
        let mut x: Complex<f64> = Complex::new(i as f64, i as f64);
        for _ in 1..64 {
            x = x-f(x)/cderivative(f, h)(x);
        }
        let real = f64::trunc(x.re*10e12)/10e12;
        let imag = f64::trunc(x.im*10e12)/10e12;
        roots.push(Complex::new(real, imag));
    }
    roots.dedup();
    return roots;
}

pub fn extrema(f: &dyn Fn(f64) -> f64, h: f64) -> Vec<f64> {
    return froot(&derivative(f, h), h);
}