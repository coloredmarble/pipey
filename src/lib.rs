macro_rules! inline_all{($($fn:item)*)=>{$(#[inline(always)]$fn)*};}
pub trait Pipey where Self: Sized {inline_all!{fn tap(self,f: impl FnOnce(&Self)) -> Self{f(&self); self} fn tap_mut(mut self,f: impl FnOnce(&mut Self)) -> Self{f(&mut self); self} fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {f(self)} fn pipe_ref<T>(&self,f: impl FnOnce(&Self) -> T) -> T {f(self)} fn pipe_mut<T>(&mut self,f: impl FnOnce(&mut Self) -> T) -> T {f(self)} fn poof(self,f: impl FnOnce(Self)) {f(self)}}}
impl<T> Pipey for T {}
