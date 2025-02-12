use crate::blocks::block_id::BlockId;
use crate::blocks::block_sort::BlockSort;
use crate::blocks::extend_accounting_feature_list::ExtendAccountingFeatureList;
use crate::blocks::shrouds::Shrouds;
use crate::shapes::shape::Shape;
use crate::shapes::shape_id::ShapeId;
use crate::utility::color::Color;
use crate::utility::component_formatting::{format_component, format_components};
use crate::utility::funky_string::{funky_string, FunkyString};
use crate::utility::option_comparison_prioritising_some::option_comparison_prioritising_some;
use std::fmt::{self, Display};

macro_rules! block {
    ($($component_name:ident: $component_value:expr),* $(,)?) => {
        Block {
            id: Some(crate::blocks::block_id::BlockId::next()),
            $($component_name: Some($component_value),)*
            ..Block::default()
        }
    };
}
pub(crate) use block;

macro_rules! block_variants {
    ($(($($component_name:ident: $component_value:expr),* $(,)?)),* $(,)?) => {
        vec![$(block_variant!($($component_name: $component_value),*)),*]
    }
}
pub(crate) use block_variants;

macro_rules! block_variant {
    ($($component_name:ident: $component_value:expr),* $(,)?) => {
        block_without_id!($($component_name: $component_value),*)
    }
}
pub(crate) use block_variant;

macro_rules! block_without_id {
    ($($component_name:ident: $component_value:expr),* $(,)?) => {
        Block {
            id: None,
            $($component_name: Some($component_value),)*
            ..Block::default()
        }
    };
}
pub(crate) use block_without_id;

#[derive(Clone)]
pub struct Block {
    pub id: Option<BlockId>,
    pub extends: Option<BlockId>,
    pub group: Option<i32>,
    pub sort: Option<BlockSort>,
    pub features: Option<ExtendAccountingFeatureList>,
    pub capacity: Option<f32>,
    pub elasticity: Option<f32>,
    pub binding_id: Option<u8>,
    pub color_1: Option<Color>,
    pub color_2: Option<Color>,
    pub line_color: Option<Color>,
    pub shape: Option<ShapeId>,
    pub scale: Option<u8>,
    pub name: Option<FunkyString>,
    pub points: Option<i32>,
    pub durability: Option<f32>,
    pub armor: Option<f32>,
    pub density: Option<f32>,
    pub blurb: Option<FunkyString>,
    pub shrouds: Option<Shrouds>,
}

macro_rules! add_scale_name_to_block {
    ($block:expr, $shape:expr, $scale_index:expr) => {
        $block.blurb = Some(funky_string!(format!(
            "{}{}",
            $shape.get_scale_name($scale_index),
            if let Some(blurb) = $block.blurb {
                format!("\\n{}", blurb.0)
            } else {
                "".to_string()
            }
        )))
    };
}

impl Block {
    pub fn extend(&self, mut new_block: Block) -> Block {
        new_block.extends = self.id;
        new_block
    }

    pub fn get_next_scale(&self) -> Block {
        block!(
            extends: self.id.unwrap(),
            scale: match self.scale {
                Some(value) => value + 1,
                None => 2,
            }
        )
    }

    pub fn get_scales(&self, scale_count: usize) -> Vec<Block> {
        (1..scale_count).fold(vec![self.clone()], |mut blocks, _| {
            blocks.push(blocks.last().unwrap().get_next_scale());
            blocks
        })
    }

    pub fn to_extended_blocks_from_singular_shape(self, shape: &Shape) -> Vec<Block> {
        static mut BASE_BLOCK_ID: Option<BlockId> = None;
        static mut LAST_VARIANT_BLOCK_ID: Option<BlockId> = None;
        static mut LAST_SHAPE_BLOCK_ID: Option<BlockId> = None;
        (0..shape.get_scale_count())
            .map({
                let original_block = &self;
                move |scale_index| {
                    if scale_index == 0 {
                        unsafe {
                            BASE_BLOCK_ID = Some(self.id.unwrap());
                            LAST_VARIANT_BLOCK_ID = BASE_BLOCK_ID;
                            LAST_SHAPE_BLOCK_ID = BASE_BLOCK_ID;
                        }
                        let mut new_block = original_block.clone();
                        new_block.shape = shape.get_id();
                        new_block.scale = Some(scale_index as u8 + 1);
                        add_scale_name_to_block!(new_block, shape, scale_index);
                        new_block
                    } else {
                        let mut new_block = block!(
                            extends: unsafe { LAST_SHAPE_BLOCK_ID.unwrap() },
                            scale: scale_index as u8 + 1
                        );
                        new_block.blurb = original_block.blurb.clone();
                        add_scale_name_to_block!(new_block, shape, scale_index);
                        new_block
                    }
                }
            })
            .collect()
    }

    pub fn to_extended_blocks_from_singular_shape_and_plural_variants(
        self,
        shape: &Shape,
        extra_variants: Vec<Block>,
    ) -> Vec<Block> {
        assert_eq!(shape.get_scale_count(), 1 + extra_variants.len());
        static mut BASE_BLOCK_ID: Option<BlockId> = None;
        static mut LAST_VARIANT_BLOCK_ID: Option<BlockId> = None;
        static mut LAST_SHAPE_BLOCK_ID: Option<BlockId> = None;
        (0..shape.get_scale_count())
            .map({
                let original_block = &self;
                move |scale_index| {
                    if scale_index == 0 {
                        unsafe {
                            BASE_BLOCK_ID = Some(self.id.unwrap());
                            LAST_VARIANT_BLOCK_ID = BASE_BLOCK_ID;
                            LAST_SHAPE_BLOCK_ID = BASE_BLOCK_ID;
                        }
                        let mut new_block = original_block.clone();
                        new_block.shape = shape.get_id();
                        new_block.scale = Some(scale_index as u8 + 1);
                        add_scale_name_to_block!(new_block, shape, scale_index);
                        new_block
                    } else {
                        let mut new_block = extra_variants.get(scale_index - 1).unwrap().to_owned();
                        new_block.id = Some(BlockId::next());
                        new_block.extends = Some(unsafe { LAST_SHAPE_BLOCK_ID.unwrap() });
                        new_block.scale = Some((scale_index + 1) as u8);
                        new_block.blurb = original_block.blurb.clone();
                        add_scale_name_to_block!(new_block, shape, scale_index);
                        new_block
                    }
                }
            })
            .collect()
    }

    pub fn to_extended_blocks_from_plural_shapes(self, shapes: &Vec<Shape>) -> Vec<Block> {
        static mut BASE_BLOCK_ID: Option<BlockId> = None;
        static mut LAST_VARIANT_BLOCK_ID: Option<BlockId> = None;
        static mut LAST_SHAPE_BLOCK_ID: Option<BlockId> = None;
        shapes
            .iter()
            .enumerate()
            .flat_map(|(shape_index, shape)| {
                (0..shape.get_scale_count()).map({
                    let original_block = &self;
                    move |scale_index| {
                        if shape_index == 0 && scale_index == 0 {
                            unsafe {
                                BASE_BLOCK_ID = Some(self.id.unwrap());
                                LAST_VARIANT_BLOCK_ID = BASE_BLOCK_ID;
                                LAST_SHAPE_BLOCK_ID = BASE_BLOCK_ID;
                            }
                            let mut new_block = original_block.clone();
                            new_block.shape = shape.get_id();
                            new_block.scale = Some(scale_index as u8 + 1);
                            add_scale_name_to_block!(new_block, shape, scale_index);
                            new_block
                        } else if scale_index == 0 {
                            let mut new_block = block!(
                                extends: unsafe { LAST_VARIANT_BLOCK_ID.unwrap() },
                                shape: shape.get_id().unwrap()
                            );
                            unsafe {
                                LAST_SHAPE_BLOCK_ID = new_block.id;
                            }
                            new_block.blurb = original_block.blurb.clone();
                            add_scale_name_to_block!(new_block, shape, scale_index);
                            new_block
                        } else {
                            let mut new_block = block!(
                                extends: unsafe { LAST_SHAPE_BLOCK_ID.unwrap() },
                                scale: scale_index as u8 + 1
                            );
                            new_block.blurb = original_block.blurb.clone();
                            add_scale_name_to_block!(new_block, shape, scale_index);
                            new_block
                        }
                    }
                })
            })
            .collect::<Vec<_>>()
    }

    pub fn to_extended_blocks_from_plural_shapes_and_plural_variants(
        self,
        shapes: &Vec<Shape>,
        extra_block_variants: Vec<Block>,
    ) -> Vec<Block> {
        let block_variants: Vec<_> = std::iter::once(self.clone())
            .chain(extra_block_variants)
            .collect();
        static mut BASE_BLOCK_ID: Option<BlockId> = None;
        static mut LAST_VARIANT_BLOCK_ID: Option<BlockId> = None;
        static mut LAST_SHAPE_BLOCK_ID: Option<BlockId> = None;
        block_variants
            .iter()
            .enumerate()
            .flat_map(move |(block_variant_index, block_variant)| {
                shapes
                    .iter()
                    .enumerate()
                    .flat_map(|(shape_index, shape)| {
                        (0..shape.get_scale_count()).map({
                            let original_block = &self;
                            move |scale_index| {
                                if block_variant_index == 0 && shape_index == 0 && scale_index == 0
                                {
                                    unsafe {
                                        BASE_BLOCK_ID = Some(self.id.unwrap());
                                        LAST_VARIANT_BLOCK_ID = BASE_BLOCK_ID;
                                        LAST_SHAPE_BLOCK_ID = BASE_BLOCK_ID;
                                    }
                                    let mut new_block = original_block.clone();
                                    new_block.shape = shape.get_id();
                                    new_block.scale = Some(scale_index as u8 + 1);
                                    add_scale_name_to_block!(new_block, shape, scale_index);
                                    new_block
                                } else if shape_index == 0 && scale_index == 0 {
                                    let mut new_block = block_variant.clone();
                                    unsafe {
                                        new_block.id = Some(BlockId::next());
                                        new_block.extends = BASE_BLOCK_ID;
                                        LAST_VARIANT_BLOCK_ID = new_block.id;
                                        LAST_SHAPE_BLOCK_ID = new_block.id;
                                    }
                                    option_comparison_prioritising_some!(
                                        new_block.blurb,
                                        original_block.blurb.clone(),
                                    );
                                    add_scale_name_to_block!(new_block, shape, scale_index);
                                    new_block
                                } else if scale_index == 0 {
                                    let mut new_block = block!(
                                        extends: unsafe { LAST_VARIANT_BLOCK_ID.unwrap() },
                                        shape: shape.get_id().unwrap()
                                    );
                                    unsafe {
                                        LAST_SHAPE_BLOCK_ID = new_block.id;
                                    }
                                    option_comparison_prioritising_some!(
                                        new_block.blurb,
                                        block_variant.blurb.clone(),
                                    );
                                    option_comparison_prioritising_some!(
                                        new_block.blurb,
                                        original_block.blurb.clone(),
                                    );
                                    add_scale_name_to_block!(new_block, shape, scale_index);
                                    new_block
                                } else {
                                    let mut new_block = block!(
                                        extends: unsafe { LAST_SHAPE_BLOCK_ID.unwrap() },
                                        scale: scale_index as u8 + 1
                                    );
                                    option_comparison_prioritising_some!(
                                        new_block.blurb,
                                        block_variant.blurb.clone(),
                                    );
                                    option_comparison_prioritising_some!(
                                        new_block.blurb,
                                        original_block.blurb.clone(),
                                    );
                                    add_scale_name_to_block!(new_block, shape, scale_index);
                                    new_block
                                }
                            }
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
}

impl Default for Block {
    fn default() -> Self {
        Block {
            id: Some(BlockId::default()),
            extends: None,
            group: None,
            sort: None,
            features: None,
            capacity: None,
            elasticity: None,
            binding_id: None,
            color_1: None,
            color_2: None,
            line_color: None,
            shape: None,
            scale: None,
            name: None,
            points: None,
            durability: None,
            armor: None,
            density: None,
            blurb: None,
            shrouds: None,
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}{}{}{}}}",
            match self.id {
                Some(value) => value.to_string(),
                None => String::new(),
            },
            format_components!(
                self.extends => "extends",
                self.group => "group",
                self.sort => "sort",
                self.capacity => "capacity",
                self.elasticity => "elasicity",
                self.binding_id => "bindingId",
                &self.color_1 => "fillColor",
                &self.color_2 => "fillColor1",
                &self.line_color => "lineColor",
                self.shape => "shape",
                self.scale => "scale",
                &self.name => "name",
                self.points => "points",
                self.durability => "durability",
                self.armor => "armor",
                self.density => "density",
                &self.blurb => "blurb",
                &self.shrouds => "shroud",
            ),
            match &self.features {
                Some(extend_accounting_feature_list) => {
                    match extend_accounting_feature_list.feature_list_same_as_extends {
                        true => "".to_string(),
                        false => {
                            format_component!(Some(&extend_accounting_feature_list.features) => "features")
                        }
                    }
                }

                None => "".to_string(),
            },
            match &self.features {
                Some(extend_accounting_feature_list) => extend_accounting_feature_list
                    .features
                    .0
                    .iter()
                    .map(|feature| feature.components_to_string())
                    .collect::<Vec<_>>()
                    .join(""),
                None => "".to_string(),
            },
        )
    }
}
