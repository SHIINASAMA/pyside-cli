use crate::errcode::Errcode;

pub trait Builder {
    fn pre_build(&self) -> Result<(), Errcode>;
    fn build(&self) -> Result<(), Errcode>;
    fn post_build(&self) -> Result<(), Errcode>;
}
