macro_rules! option_comparison_prioritising_some {
    ($original_option:expr, $possible_option:expr $(,)?) => {
        if let Some(_) = $original_option {
        } else if let Some(_) = $possible_option {
            $original_option = $possible_option;
        } else {
            $original_option = None;
        }
    }; // ($original_option:expr, $possible_option:expr) => {
       //     if let Some(_) = $original_option {
       //         $original_option
       //     } else if let Some(_) = $possible_option {
       //         $possible_option
       //     } else {
       //         None
       //     }

       // };
}
pub(crate) use option_comparison_prioritising_some;
