//! Global Streaming Aggregators

use crate::array2::Array;

use super::aggregation::*;

/// `GlobalStreamingSumAgg` sums data of the same type. It sums values from
/// `StreamingSumAgg`.
pub type GlobalStreamingSumAgg<R> =
    StreamingFoldAgg<R, R, PrimitiveSummable<<R as Array>::OwnedItem>>;

/// `GloabalStreamingFloatSumAgg` sums data of the same float type. It sums
/// values from `StreamingFloatSumAgg`.
pub type GlobalStreamingFloatSumAgg<R> =
    StreamingFoldAgg<R, R, FloatPrimitiveSummable<<R as Array>::OwnedItem>>;

/// `GlobalStreamingCountAgg` counts data of any type. It sums values from
/// `StreamingCountAgg`.
pub type GlobalStreamingCountAgg<R> =
    StreamingFoldAgg<R, R, PrimitiveSummable<<R as Array>::OwnedItem>>;

/// `GlobalAggregationOperator` shares the sample implementation as
/// `AggregationOperator` for now. It collects data from all upstream local
/// aggregators and sum them up.
pub type GlobalAggregationOperator = super::AggregationOperator;
