/**
@author 韩峰 - 08 - Team1
@date 2021-08-13
*/

use std::f64::consts::PI;
use std::fmt::Debug;

// HW1：为枚举交通信号灯实现一个trait，trait里包含一个返回时间的方法，不同灯持续时间不同
pub trait Time {
    fn time(&self) -> u8;
}

pub enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl Time for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            Self::Red => 70,
            Self::Green => 60,
            Self::Yellow => 3,
        }
    }
}

//HW2：实现一个函数，为u32类型的整数集合求和，参数类型为&[u32]，返回类型为Option<u32>，溢出时返回None
pub fn sum(arr: &[u32]) -> Option<u32> {
    arr.iter().try_fold(0u32,|acc,&x| acc.checked_add(x))
}

//HW3：实现一个打印图形面积的函数，接收一个可以计算面积的类型作为参数，需要用到泛型和泛型约束
pub fn print_area<T: Area + Debug>(item: T) {
    println!("{:?} area is {}",item, item.area());
}

pub trait Area {
    fn area(&self) -> f64;
}

#[derive(Debug)]
pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Circle {
            radius,
        }
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.radius.powf(2.0)
    }
}

#[derive(Debug)]
pub struct Square {
    width: f64,
    length: f64,
}

impl Square {
    pub fn new(width: f64,length:f64) -> Self {
        Square {
            width,
            length,
        }
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.width*self.length
    }
}

#[derive(Debug)]
pub struct Triangel {
    base: f64,
    height: f64,
}

impl Triangel {
    pub fn new(base: f64,height:f64) -> Self {
        Triangel {
            base,
            height,
        }
    }
}

impl Area for Triangel{
    fn area(&self) -> f64 {
        self.base*self.height*0.5f64
    }
}

//HW1
pub fn test_traffic_light() {
    let light = TrafficLight::Red;
    println!("red time is {}", light.time());
    let light = TrafficLight::Green;
    println!("green time is {}", light.time());
    let light = TrafficLight::Yellow;
    println!("yellow time is {}", light.time());
}

//HW2
pub fn test_sum() {
    let arr1: [u32; 5] = [1,2,3,4,5];
    assert_eq!(Some(15), sum(&arr1));

    let arr2 : [u32; 2] = [1, u32::max_value()];
    assert_eq!(None, sum(&arr2)); 
}

//HW3
pub fn test_print_area() {
    print_area(Circle::new(5.0));
    print_area(Square::new(4.0, 5.0));
    print_area(Triangel::new(4.0, 5.0));
}

fn main() {
    test_traffic_light();
    test_sum();
    test_print_area();
}