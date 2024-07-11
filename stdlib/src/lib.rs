use fs_core::*;

#[cfg(test)]
mod tests;

pub struct Start;
impl StationType for Start {}

pub struct Print;
impl StationType for Print {}

pub struct Println;
impl StationType for Println {}

pub struct Exit;
impl StationType for Exit {}
