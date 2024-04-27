#![no_std]macro_rules! inline_all{($($fn:item)*)=>{$(#[inline(always)]$fn)*};}
pub trait Pipey<T> where Self: Sized {inline_all!{fn tap(self,f: impl FnOnce(&Self)) -> Self{f(&self); self} fn tap_mut(mut self,f: impl FnOnce(&mut Self)) -> Self{f(&mut self); self} fn pipe(self, f: impl FnOnce(Self) -> T) -> T {f(self)} fn pipe_ref(&self,f: impl FnOnce(&Self) -> T) -> T {f(self)} fn pipe_mut(&mut self,f: impl FnOnce(&mut Self) -> T) -> T {f(self)} fn poof(self,f: impl FnOnce(Self) -> T) {f(self);} fn btw(self, f: impl FnOnce()) -> Self {f();self}}}
impl<T> Pipey<T> for T {}
