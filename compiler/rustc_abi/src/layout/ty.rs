use std::fmt;
use std::ops::{Deref, Range};

use rustc_data_structures::intern::Interned;
use rustc_macros::StableHash;

use crate::layout::{FieldIdx, VariantIdx};
use crate::{
    AbiAlign, Align, BackendRepr, FieldsShape, Float, HasDataLayout, LayoutData, Niche,
    PointeeInfo, Primitive, Size, Variants,
};

// Explicitly import `Float` to avoid ambiguity with `Primitive::Float`.

#[derive(Copy, Clone, PartialEq, Eq, Hash, StableHash)]
#[rustc_pass_by_value]
pub struct Layout<'a>(pub Interned<'a, LayoutData<FieldIdx, VariantIdx>>);

impl<'a> fmt::Debug for Layout<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // See comment on `<LayoutData as Debug>::fmt` above.
        self.0.0.fmt(f)
    }
}

impl<'a> Deref for Layout<'a> {
    type Target = &'a LayoutData<FieldIdx, VariantIdx>;
    fn deref(&self) -> &&'a LayoutData<FieldIdx, VariantIdx> {
        &self.0.0
    }
}

impl<'a> Layout<'a> {
    pub fn fields(self) -> &'a FieldsShape<FieldIdx> {
        &self.0.0.fields
    }

    pub fn variants(self) -> &'a Variants<FieldIdx, VariantIdx> {
        &self.0.0.variants
    }

    pub fn backend_repr(self) -> BackendRepr {
        self.0.0.backend_repr
    }

    pub fn largest_niche(self) -> Option<Niche> {
        self.0.0.largest_niche
    }

    pub fn align(self) -> AbiAlign {
        self.0.0.align
    }

    pub fn size(self) -> Size {
        self.0.0.size
    }

    pub fn max_repr_align(self) -> Option<Align> {
        self.0.0.max_repr_align
    }

    pub fn unadjusted_abi_align(self) -> Align {
        self.0.0.unadjusted_abi_align
    }
}

/// The layout of a type, alongside the type itself.
/// Provides various type traversal APIs (e.g., recursing into fields).
///
/// Note that the layout is NOT guaranteed to always be identical
/// to that obtained from `layout_of(ty)`, as we need to produce
/// layouts for which Rust types do not exist, such as enum variants
/// or synthetic fields of enums (i.e., discriminants) and wide pointers.
#[derive(Copy, Clone, PartialEq, Eq, Hash, StableHash)]
pub struct TyAndLayout<'a, Ty> {
    pub ty: Ty,
    pub layout: Layout<'a>,
}

impl<'a, Ty: fmt::Display> fmt::Debug for TyAndLayout<'a, Ty> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Print the type in a readable way, not its debug representation.
        f.debug_struct("TyAndLayout")
            .field("ty", &format_args!("{}", self.ty))
            .field("layout", &self.layout)
            .finish()
    }
}

impl<'a, Ty> Deref for TyAndLayout<'a, Ty> {
    type Target = &'a LayoutData<FieldIdx, VariantIdx>;
    fn deref(&self) -> &&'a LayoutData<FieldIdx, VariantIdx> {
        &self.layout.0.0
    }
}

impl<'a, Ty> AsRef<LayoutData<FieldIdx, VariantIdx>> for TyAndLayout<'a, Ty> {
    fn as_ref(&self) -> &LayoutData<FieldIdx, VariantIdx> {
        &*self.layout.0.0
    }
}

/// Trait that needs to be implemented by the higher-level type representation
/// (e.g. `rustc_middle::ty::Ty`), to provide `rustc_target::abi` functionality.
pub trait TyAbiInterface<'a, C>: Sized + std::fmt::Debug + std::fmt::Display {
    fn ty_and_layout_for_variant(
        this: TyAndLayout<'a, Self>,
        cx: &C,
        variant_index: VariantIdx,
    ) -> TyAndLayout<'a, Self>;
    fn ty_and_layout_field(this: TyAndLayout<'a, Self>, cx: &C, i: usize) -> TyAndLayout<'a, Self>;
    fn ty_and_layout_pointee_info_at(
        this: TyAndLayout<'a, Self>,
        cx: &C,
        offset: Size,
    ) -> Option<PointeeInfo>;
    fn is_adt(this: TyAndLayout<'a, Self>) -> bool;
    fn is_never(this: TyAndLayout<'a, Self>) -> bool;
    fn is_tuple(this: TyAndLayout<'a, Self>) -> bool;
    fn is_unit(this: TyAndLayout<'a, Self>) -> bool;
    fn is_transparent(this: TyAndLayout<'a, Self>) -> bool;
    fn is_scalable_vector(this: TyAndLayout<'a, Self>) -> bool;
    /// See [`TyAndLayout::pass_indirectly_in_non_rustic_abis`] for details.
    fn is_pass_indirectly_in_non_rustic_abis_flag_set(this: TyAndLayout<'a, Self>) -> bool;
}

impl<'a, Ty> TyAndLayout<'a, Ty> {
    pub fn for_variant<C>(self, cx: &C, variant_index: VariantIdx) -> Self
    where
        Ty: TyAbiInterface<'a, C>,
    {
        Ty::ty_and_layout_for_variant(self, cx, variant_index)
    }

    pub fn field<C>(self, cx: &C, i: usize) -> Self
    where
        Ty: TyAbiInterface<'a, C>,
    {
        Ty::ty_and_layout_field(self, cx, i)
    }

    pub fn pointee_info_at<C>(self, cx: &C, offset: Size) -> Option<PointeeInfo>
    where
        Ty: TyAbiInterface<'a, C>,
    {
        Ty::ty_and_layout_pointee_info_at(self, cx, offset)
    }

    pub fn is_single_fp_element<C>(self, cx: &C) -> bool
    where
        Ty: TyAbiInterface<'a, C>,
        C: HasDataLayout,
    {
        match self.backend_repr {
            BackendRepr::Scalar(scalar) => {
                matches!(scalar.primitive(), Primitive::Float(Float::F32 | Float::F64))
            }
            BackendRepr::Memory { .. } => {
                if self.fields.count() == 1 && self.fields.offset(0).bytes() == 0 {
                    self.field(cx, 0).is_single_fp_element(cx)
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn is_single_vector_element<C>(self, cx: &C, expected_size: Size) -> bool
    where
        Ty: TyAbiInterface<'a, C>,
        C: HasDataLayout,
    {
        match self.backend_repr {
            BackendRepr::SimdVector { .. } => self.size == expected_size,
            BackendRepr::Memory { .. } => {
                if self.fields.count() == 1 && self.fields.offset(0).bytes() == 0 {
                    self.field(cx, 0).is_single_vector_element(cx, expected_size)
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn is_adt<C>(self) -> bool
    where
        Ty: TyAbiInterface<'a, C>,
    {
        Ty::is_adt(self)
    }

    pub fn is_never<C>(self) -> bool
    where
        Ty: TyAbiInterface<'a, C>,
    {
        Ty::is_never(self)
    }

    pub fn is_tuple<C>(self) -> bool
    where
        Ty: TyAbiInterface<'a, C>,
    {
        Ty::is_tuple(self)
    }

    pub fn is_unit<C>(self) -> bool
    where
        Ty: TyAbiInterface<'a, C>,
    {
        Ty::is_unit(self)
    }

    pub fn is_transparent<C>(self) -> bool
    where
        Ty: TyAbiInterface<'a, C>,
    {
        Ty::is_transparent(self)
    }

    pub fn is_scalable_vector<C>(self) -> bool
    where
        Ty: TyAbiInterface<'a, C>,
    {
        Ty::is_scalable_vector(self)
    }

    /// If this method returns `true`, then this type should always have a `PassMode` of
    /// `Indirect { on_stack: false, .. }` when being used as the argument type of a function with a
    /// non-Rustic ABI (this is true for structs annotated with the
    /// `#[rustc_pass_indirectly_in_non_rustic_abis]` attribute).
    ///
    /// This is used to replicate some of the behaviour of C array-to-pointer decay; however unlike
    /// C any changes the caller makes to the passed value will not be reflected in the callee, so
    /// the attribute is only useful for types where observing the value in the caller after the
    /// function call isn't allowed (a.k.a. `va_list`).
    ///
    /// This function handles transparent types automatically.
    pub fn pass_indirectly_in_non_rustic_abis<C>(self, cx: &C) -> bool
    where
        Ty: TyAbiInterface<'a, C> + Copy,
    {
        let base = self.peel_transparent_wrappers(cx);
        Ty::is_pass_indirectly_in_non_rustic_abis_flag_set(base)
    }

    /// Recursively peel away transparent wrappers, returning the inner value.
    ///
    /// The return value is not `repr(transparent)` and/or does
    /// not have a non-1zst field.
    pub fn peel_transparent_wrappers<C>(mut self, cx: &C) -> Self
    where
        Ty: TyAbiInterface<'a, C> + Copy,
    {
        while self.is_transparent()
            && let Some((_, field)) = self.non_1zst_field(cx)
        {
            self = field;
        }

        self
    }

    /// Finds the one field that is not a 1-ZST.
    /// Returns `None` if there are multiple non-1-ZST fields or only 1-ZST-fields.
    pub fn non_1zst_field<C>(&self, cx: &C) -> Option<(FieldIdx, Self)>
    where
        Ty: TyAbiInterface<'a, C> + Copy,
    {
        let mut found = None;
        for field_idx in 0..self.fields.count() {
            let field = self.field(cx, field_idx);
            if field.is_1zst() {
                continue;
            }
            if found.is_some() {
                // More than one non-1-ZST field.
                return None;
            }
            found = Some((FieldIdx::from_usize(field_idx), field));
        }
        found
    }

    /// Computes the byte ranges of this type that are *statically known* to be
    /// uninitialized, i.e. padding between fields and trailing/leading gaps in
    /// unions. The returned ranges are relative to the start of this type and
    /// are sorted and non-overlapping.
    ///
    /// Only offsets that may be uninitialized regardless of the runtime value
    /// are reported. In particular, for an enum (or any multi-variant type), an
    /// offset that is initialized in one variant but not in another is *not*
    /// reported, as it cannot statically be known to be uninitialized. The enum
    /// tag/discriminant is likewise always considered initialized.
    pub fn uninit_ranges<C>(self, cx: &C) -> Vec<Range<Size>>
    where
        Ty: TyAbiInterface<'a, C> + Copy,
    {
        // First collect all the byte ranges that are inside some field in *some*
        // variant; the uninitialized ranges are exactly the complement of these.
        let mut data = RangeSet(Vec::new());
        self.add_data_ranges(cx, Size::ZERO, &mut data);

        // Walk the (sorted, merged) data ranges and emit the gaps between them
        // within `0..self.size`.
        let mut uninit = Vec::new();
        let mut covered_until = Size::ZERO;
        for &(offset, size) in data.0.iter() {
            if offset > covered_until {
                uninit.push(covered_until..offset);
            }
            covered_until = covered_until.max(offset + size);
        }
        if self.size > covered_until {
            uninit.push(covered_until..self.size);
        }
        uninit
    }

    /// Helper for [`Self::uninit_ranges`]: recursively add the byte ranges that
    /// are inside some field (i.e. not padding) to `out`. Mirrors the data-range
    /// computation in `rustc_const_eval`'s `union_data_range`, but generalized to
    /// any type rather than just unions.
    fn add_data_ranges<C>(self, cx: &C, base_offset: Size, out: &mut RangeSet)
    where
        Ty: TyAbiInterface<'a, C> + Copy,
    {
        // If this is a ZST, we don't contain any data. In particular, this helps
        // us to quickly skip over huge arrays of ZST.
        if self.is_zst() {
            return;
        }
        // Recursively add all the fields of everything to the output.
        match &self.fields {
            FieldsShape::Primitive => {
                out.add_range(base_offset, self.size);
            }
            &FieldsShape::Union(fields) => {
                // Currently, all fields start at offset 0 (relative to `base_offset`).
                for field in 0..fields.get() {
                    let field = self.field(cx, field);
                    field.add_data_ranges(cx, base_offset, out);
                }
            }
            &FieldsShape::Array { stride, count } => {
                let elem = self.field(cx, 0);

                // Fast-path for large arrays of simple types that do not contain any padding.
                if elem.backend_repr.is_scalar() {
                    out.add_range(base_offset, elem.size * count);
                } else {
                    for idx in 0..count {
                        elem.add_data_ranges(cx, base_offset + idx * stride, out);
                    }
                }
            }
            FieldsShape::Arbitrary { offsets, .. } => {
                for (field, &offset) in offsets.iter_enumerated() {
                    let field = self.field(cx, field.as_usize());
                    field.add_data_ranges(cx, base_offset + offset, out);
                }
            }
        }
        // Don't forget potential other variants.
        match &self.variants {
            Variants::Single { .. } | Variants::Empty => {
                // Fully handled above.
            }
            Variants::Multiple { variants, .. } => {
                for variant in variants.indices() {
                    let variant = self.for_variant(cx, variant);
                    variant.add_data_ranges(cx, base_offset, out);
                }
            }
        }
    }
}

/// A sorted, non-overlapping set of byte ranges, used to accumulate the ranges
/// of a type that are inside some field. Copied from `rustc_const_eval`'s
/// validity checker.
struct RangeSet(Vec<(Size, Size)>);

impl RangeSet {
    fn add_range(&mut self, offset: Size, size: Size) {
        if size.bytes() == 0 {
            // No need to track empty ranges.
            return;
        }
        let v = &mut self.0;
        // We scan for a partition point where the left partition is all the elements that end
        // strictly before we start. Those are elements that are too "low" to merge with us.
        let idx =
            v.partition_point(|&(other_offset, other_size)| other_offset + other_size < offset);
        // Now we want to either merge with the first element of the second partition, or insert ourselves before that.
        if let Some(&(other_offset, other_size)) = v.get(idx)
            && offset + size >= other_offset
        {
            // Their end is >= our start (otherwise it would not be in the 2nd partition) and
            // our end is >= their start. This means we can merge the ranges.
            let new_start = other_offset.min(offset);
            let mut new_end = (other_offset + other_size).max(offset + size);
            // We grew to the right, so merge with overlapping/adjacent elements.
            // (We also may have grown to the left, but that can never make us adjacent with
            // anything there since we selected the first such candidate via `partition_point`.)
            let mut scan_right = 1;
            while let Some(&(next_offset, next_size)) = v.get(idx + scan_right)
                && new_end >= next_offset
            {
                // Increase our size to absorb the next element.
                new_end = new_end.max(next_offset + next_size);
                // Look at the next element.
                scan_right += 1;
            }
            // Update the element we grew.
            v[idx] = (new_start, new_end - new_start);
            // Remove the elements we absorbed (if any).
            if scan_right > 1 {
                drop(v.drain((idx + 1)..(idx + scan_right)));
            }
        } else {
            // Insert new element.
            v.insert(idx, (offset, size));
        }
    }
}
