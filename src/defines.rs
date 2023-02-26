////////////////////////////////////////////////////////////////////////////////////////////////////

/// Number of widget children that are stored without storage heap allocation.
///
/// In technical term it is [smallvec::SmallVec] number value.
///
/// Example:
/// ```ascii
///  SmallVec<[Rc<RefCell<dyn IWidget>>; STATIC_CHILD_NUM]>
/// ```
pub const STATIC_CHILD_NUM: usize = 3;

/// Number of regions to detect children redraw without storage heap allocation.
///
/// Example:
/// ```ascii
///  SmallVec<[Rect<f32>; STATIC_REGIONS_NUM]>
/// ```
pub const STATIC_REGIONS_NUM: usize = 3;

////////////////////////////////////////////////////////////////////////////////////////////////////
