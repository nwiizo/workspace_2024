#![allow(unused_imports, non_snake_case)]
pub use __cargo_equip::prelude::*;

use convex_hull::{monotone_chain, Point};
use itertools::{Itertools, *};
use proconio::{marker::*, *};

#[fastout]
fn main() {
    input! {
        N: usize,
        X_H: [(i64, i64); N],
    }
    let mut ans = -1.0;
    for ls in X_H.windows(2) {
        let (x1, y1) = ls[0];
        let (x2, y2) = ls[1];
        let numer = y1 * (x2 - x1) - (y2 - y1) * x1;
        if numer >= 0 {
            let denom = x2 - x1;
            let cur_ans = numer as f64 / denom as f64;
            chmax!(ans, cur_ans);
        }
    }
    if ans == -1.0 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

#[macro_export]
macro_rules! mat {
    ($($e:expr),*) => { vec![$($e),*] };
    ($($e:expr,)*) => { vec![$($e),*] };
    ($e:expr; $d:expr) => { vec![$e; $d] };
    ($e:expr; $d:expr $(; $ds:expr)+) => { vec![mat![$e $(; $ds)*]; $d] };
}

#[macro_export]
macro_rules! chmin {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    };
}

#[macro_export]
macro_rules! chmax {
    ($a:expr, $b:expr) => {
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    };
}

/// https://maguro.dev/debug-macro/
#[macro_export]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

// The following code was expanded by `cargo-equip`.

///  # Bundled libraries
/// 
///  - `convex_hull 0.1.0 (git+https://github.com/CoCo-Japan-pan/procon_lib_rs.git#63e59940df32b985fa783dc40d066f53dc9ba130)`     licensed under `CC0-1.0` as `crate::__cargo_equip::crates::convex_hull`
///  - `geometry_basics 0.1.0 (git+https://github.com/CoCo-Japan-pan/procon_lib_rs.git#63e59940df32b985fa783dc40d066f53dc9ba130)` licensed under `CC0-1.0` as `crate::__cargo_equip::crates::__geometry_basics_0_1_0`
#[cfg_attr(any(), rustfmt::skip)]
#[allow(unused)]
mod __cargo_equip {
    pub(crate) mod crates {
        pub mod convex_hull {use crate::__cargo_equip::preludes::convex_hull::*;pub use geometry_basics::Point;pub fn monotone_chain(points:&[Point],contain_mid_point:bool)->(Vec<Point>,Vec<Point>){for ls in points.windows(2){assert!(ls[0]<=ls[1],"please sort the input for monotone chain!!!");}let lower_hull=calc_hull(points.len(),points.iter(),contain_mid_point);let upper_hull=calc_hull(points.len(),points.iter().rev(),contain_mid_point);(lower_hull,upper_hull)}fn calc_hull<'a,T:Iterator<Item=&'a Point>>(len:usize,points:T,contain_mid_point:bool,)->Vec<Point>{let mut hull=Vec::with_capacity(len);for&p in points{while hull.len()>1{let second=hull[hull.len()-2];let first=hull[hull.len()-1];let from=second-first;let to=p-first;let cross=from.cross(to);if cross>0||(!contain_mid_point&&cross==0){hull.pop();}else{break;}}hull.push(p);}hull.shrink_to(0);hull}pub fn calc_farthest_point_pair(points:&[Point])->i64{let ch={let(mut lower_hull,mut upper_hull)=monotone_chain(points,false);lower_hull.pop();upper_hull.pop();lower_hull.append(&mut upper_hull);lower_hull};let len=ch.len();if len==2{return(ch[0]-ch[1]).square_dist();}let mut i=ch.iter().enumerate().min_by_key(|(_,p)|**p).unwrap().0;let mut j=ch.iter().enumerate().max_by_key(|(_,p)|**p).unwrap().0;let mut dist=0;let si=i;let sj=j;while i!=sj||j!=si{dist=dist.max((ch[i]-ch[j]).square_dist());let i_i1=ch[(i+1)%len]-ch[i];let j_j1=ch[(j+1)%len]-ch[j];if i_i1.cross(j_j1)<0{i=(i+1)%len;}else{j=(j+1)%len;}}dist}}
        pub mod __geometry_basics_0_1_0 {use std::fmt::Display;use std::ops::{Add,Mul,Sub};#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]pub struct Point{pub x:i64,pub y:i64,}impl Display for Point{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"{} {}",self.x,self.y)}}impl From<(i64,i64)>for Point{fn from(value:(i64,i64))->Self{Point::new(value.0,value.1)}}impl Point{pub fn new(x:i64,y:i64)->Self{Point{x,y}}pub fn dot(self,rhs:Self)->i64{self.x*rhs.x+self.y*rhs.y}pub fn cross(self,rhs:Self)->i64{self.x*rhs.y-self.y*rhs.x}pub fn square_dist(self)->i64{self.dot(self)}}impl Add for Point{type Output=Self;fn add(self,rhs:Self)->Self::Output{Point::new(self.x+rhs.x,self.y+rhs.y)}}impl Sub for Point{type Output=Self;fn sub(self,rhs:Self)->Self::Output{Point::new(self.x-rhs.x,self.y-rhs.y)}}impl Mul<i64>for Point{type Output=Self;fn mul(self,rhs:i64)->Self::Output{Point::new(self.x*rhs,self.y*rhs)}}}
    }

    pub(crate) mod macros {
        pub mod convex_hull {}
        pub mod __geometry_basics_0_1_0 {}
    }

    pub(crate) mod prelude {pub use crate::__cargo_equip::crates::*;}

    mod preludes {
        pub mod convex_hull {pub(in crate::__cargo_equip)use crate::__cargo_equip::crates::__geometry_basics_0_1_0 as geometry_basics;}
        pub mod __geometry_basics_0_1_0 {}
    }
}
