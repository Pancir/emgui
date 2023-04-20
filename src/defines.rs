use std::time::Duration;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Number of widget children that are stored without heap allocation.
///
/// # Note
/// This can be setup during compilation.
///
/// In technical term it is [smallvec::SmallVec] number value.
///
/// Example:
/// ```ascii
///  SmallVec<[WidgetStrongRef; STATIC_CHILD_NUM]>
/// ```
#[const_env::from_env]
pub const STATIC_CHILD_NUM: usize = 3;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const DEFAULT_DOUBLE_CLICK_TIME: Duration = Duration::from_millis(200);
pub const DEFAULT_TOOL_TIP_TIME: Duration = Duration::from_secs(2);

////////////////////////////////////////////////////////////////////////////////////////////////////
