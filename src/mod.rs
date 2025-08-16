use crate::blocks::blocks::Blocks;
use crate::shapes::shapes::Shapes;

#[derive(Debug)]
pub struct Mod<'a> {
    pub blocks_option: Option<&'a mut Blocks>,
    pub shapes_option: Option<&'a mut Shapes>,
}

// impl Default for Mod<'_> {
//     fn default() -> Self {
//         Mod { blocks_option: Some(&mut Blocks::default()), shapes_option: Some(&mut Shapes::default()) }
//     }
// }
