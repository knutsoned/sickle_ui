pub mod ui_style {
    use bevy::{
        ecs::system::{EntityCommand, EntityCommands},
        prelude::*, ui::FocusPolicy, utils::HashSet,
    };
    use sickle_macros::StyleCommands;
    use sickle_math::lerp::Lerp;
    use crate::{
        theme::{
            dynamic_style::DynamicStyle,
            dynamic_style_attribute::{DynamicStyleAttribute, DynamicStyleController},
            style_animation::{AnimationProgress, StyleAnimation},
        },
        FluxInteraction,
    };
    pub struct UiStyle<'a> {
        commands: EntityCommands<'a>,
    }
    impl<'a> UiStyle<'a> {
        pub fn id(&self) -> Entity {
            self.commands.id()
        }
        pub fn entity_commands(&mut self) -> EntityCommands {
            self.commands.reborrow()
        }
    }
    pub trait UiStyleExt<'a> {
        fn style(&'a mut self, entity: Entity) -> UiStyle<'a>;
    }
    impl<'a> UiStyleExt<'a> for Commands<'_, '_> {
        fn style(&'a mut self, entity: Entity) -> UiStyle<'a> {
            UiStyle {
                commands: self.entity(entity),
            }
        }
    }
    pub struct UiStyleUnchecked<'a> {
        commands: EntityCommands<'a>,
    }
    impl<'a> UiStyleUnchecked<'a> {
        pub fn id(&self) -> Entity {
            self.commands.id()
        }
        pub fn entity_commands(&mut self) -> EntityCommands {
            self.commands.reborrow()
        }
    }
    pub trait UiStyleUncheckedExt<'a> {
        fn style(&'a mut self, entity: Entity) -> UiStyleUnchecked<'a>;
    }
    impl<'a> UiStyleUncheckedExt<'a> for Commands<'_, '_> {
        fn style(&'a mut self, entity: Entity) -> UiStyleUnchecked<'a> {
            UiStyleUnchecked {
                commands: self.entity(entity),
            }
        }
    }
    /// Derive leaves the original struct, ignore it.
    /// (derive macros have a better style overall)
    enum _StyleAttributes {
        Display { display: Display },
        PositionType { position_type: PositionType },
        Overflow { overflow: Overflow },
        Direction { direction: Direction },
        #[animatable]
        Left { left: Val },
        #[animatable]
        Right { right: Val },
        #[animatable]
        Top { top: Val },
        #[animatable]
        Bottom { bottom: Val },
        #[animatable]
        Width { width: Val },
        #[animatable]
        Height { height: Val },
        #[animatable]
        MinWidth { min_width: Val },
        #[animatable]
        MinHeight { min_height: Val },
        #[animatable]
        MaxWidth { max_width: Val },
        #[animatable]
        MaxHeight { max_height: Val },
        AspectRatio { aspect_ratio: Option<f32> },
        AlignItems { align_items: AlignItems },
        JustifyItems { justify_items: JustifyItems },
        AlignSelf { align_self: AlignSelf },
        JustifySelf { justify_self: JustifySelf },
        AlignContent { align_content: AlignContent },
        JustifyContents { justify_content: JustifyContent },
        #[animatable]
        Margin { margin: UiRect },
        #[animatable]
        Padding { padding: UiRect },
        #[animatable]
        Border { border: UiRect },
        FlexDirection { flex_direction: FlexDirection },
        FlexWrap { flex_wrap: FlexWrap },
        #[animatable]
        FlexGrow { flex_grow: f32 },
        #[animatable]
        FlexShrink { flex_shrink: f32 },
        #[animatable]
        FlexBasis { flex_basis: Val },
        #[animatable]
        RowGap { row_gap: Val },
        #[animatable]
        ColumnGap { column_gap: Val },
        GridAutoFlow { grid_auto_flow: GridAutoFlow },
        GridTemplateRows { grid_template_rows: Vec<RepeatedGridTrack> },
        GridTemplateColumns { grid_template_columns: Vec<RepeatedGridTrack> },
        GridAutoRows { grid_auto_rows: Vec<GridTrack> },
        GridAutoColumns { grid_auto_columns: Vec<GridTrack> },
        GridRow { grid_row: GridPlacement },
        GridColumn { grid_column: GridPlacement },
        #[target_tupl(BackgroundColor)]
        #[animatable]
        BackgroundColor { background_color: Color },
        #[target_tupl(BorderColor)]
        #[animatable]
        BorderColor { border_color: Color },
        #[target_enum]
        FocusPolicy { focus_policy: FocusPolicy },
        #[target_enum]
        Visibility { visibility: Visibility },
        #[skip_enity_command]
        ZIndex { z_index: ZIndex },
        #[skip_ui_style_ext]
        Image { image: String },
        #[skip_enity_command]
        ImageScaleMode { image_scale_mode: Option<ImageScaleMode> },
        #[static_style_only]
        #[skip_ui_style_ext]
        FluxInteraction { flux_interaction_enabled: bool },
        #[skip_lockable_enum]
        #[skip_ui_style_ext]
        AbsolutePosition { absolute_position: Vec2 },
    }
    pub enum StaticStyleAttribute {
        Display(Display),
        PositionType(PositionType),
        Overflow(Overflow),
        Direction(Direction),
        Left(Val),
        Right(Val),
        Top(Val),
        Bottom(Val),
        Width(Val),
        Height(Val),
        MinWidth(Val),
        MinHeight(Val),
        MaxWidth(Val),
        MaxHeight(Val),
        AspectRatio(Option<f32>),
        AlignItems(AlignItems),
        JustifyItems(JustifyItems),
        AlignSelf(AlignSelf),
        JustifySelf(JustifySelf),
        AlignContent(AlignContent),
        JustifyContents(JustifyContent),
        Margin(UiRect),
        Padding(UiRect),
        Border(UiRect),
        FlexDirection(FlexDirection),
        FlexWrap(FlexWrap),
        FlexGrow(f32),
        FlexShrink(f32),
        FlexBasis(Val),
        RowGap(Val),
        ColumnGap(Val),
        GridAutoFlow(GridAutoFlow),
        GridTemplateRows(Vec<RepeatedGridTrack>),
        GridTemplateColumns(Vec<RepeatedGridTrack>),
        GridAutoRows(Vec<GridTrack>),
        GridAutoColumns(Vec<GridTrack>),
        GridRow(GridPlacement),
        GridColumn(GridPlacement),
        BackgroundColor(Color),
        BorderColor(Color),
        FocusPolicy(FocusPolicy),
        Visibility(Visibility),
        ZIndex(ZIndex),
        Image(String),
        ImageScaleMode(Option<ImageScaleMode>),
        FluxInteraction(bool),
        AbsolutePosition(Vec2),
        Custom(fn(Entity, &mut World)),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StaticStyleAttribute {
        #[inline]
        fn clone(&self) -> StaticStyleAttribute {
            match self {
                StaticStyleAttribute::Display(__self_0) => {
                    StaticStyleAttribute::Display(::core::clone::Clone::clone(__self_0))
                }
                StaticStyleAttribute::PositionType(__self_0) => {
                    StaticStyleAttribute::PositionType(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::Overflow(__self_0) => {
                    StaticStyleAttribute::Overflow(::core::clone::Clone::clone(__self_0))
                }
                StaticStyleAttribute::Direction(__self_0) => {
                    StaticStyleAttribute::Direction(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::Left(__self_0) => {
                    StaticStyleAttribute::Left(::core::clone::Clone::clone(__self_0))
                }
                StaticStyleAttribute::Right(__self_0) => {
                    StaticStyleAttribute::Right(::core::clone::Clone::clone(__self_0))
                }
                StaticStyleAttribute::Top(__self_0) => {
                    StaticStyleAttribute::Top(::core::clone::Clone::clone(__self_0))
                }
                StaticStyleAttribute::Bottom(__self_0) => {
                    StaticStyleAttribute::Bottom(::core::clone::Clone::clone(__self_0))
                }
                StaticStyleAttribute::Width(__self_0) => {
                    StaticStyleAttribute::Width(::core::clone::Clone::clone(__self_0))
                }
                StaticStyleAttribute::Height(__self_0) => {
                    StaticStyleAttribute::Height(::core::clone::Clone::clone(__self_0))
                }
                StaticStyleAttribute::MinWidth(__self_0) => {
                    StaticStyleAttribute::MinWidth(::core::clone::Clone::clone(__self_0))
                }
                StaticStyleAttribute::MinHeight(__self_0) => {
                    StaticStyleAttribute::MinHeight(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::MaxWidth(__self_0) => {
                    StaticStyleAttribute::MaxWidth(::core::clone::Clone::clone(__self_0))
                }
                StaticStyleAttribute::MaxHeight(__self_0) => {
                    StaticStyleAttribute::MaxHeight(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::AspectRatio(__self_0) => {
                    StaticStyleAttribute::AspectRatio(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::AlignItems(__self_0) => {
                    StaticStyleAttribute::AlignItems(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::JustifyItems(__self_0) => {
                    StaticStyleAttribute::JustifyItems(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::AlignSelf(__self_0) => {
                    StaticStyleAttribute::AlignSelf(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::JustifySelf(__self_0) => {
                    StaticStyleAttribute::JustifySelf(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::AlignContent(__self_0) => {
                    StaticStyleAttribute::AlignContent(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::JustifyContents(__self_0) => {
                    StaticStyleAttribute::JustifyContents(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::Margin(__self_0) => {
                    StaticStyleAttribute::Margin(::core::clone::Clone::clone(__self_0))
                }
                StaticStyleAttribute::Padding(__self_0) => {
                    StaticStyleAttribute::Padding(::core::clone::Clone::clone(__self_0))
                }
                StaticStyleAttribute::Border(__self_0) => {
                    StaticStyleAttribute::Border(::core::clone::Clone::clone(__self_0))
                }
                StaticStyleAttribute::FlexDirection(__self_0) => {
                    StaticStyleAttribute::FlexDirection(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::FlexWrap(__self_0) => {
                    StaticStyleAttribute::FlexWrap(::core::clone::Clone::clone(__self_0))
                }
                StaticStyleAttribute::FlexGrow(__self_0) => {
                    StaticStyleAttribute::FlexGrow(::core::clone::Clone::clone(__self_0))
                }
                StaticStyleAttribute::FlexShrink(__self_0) => {
                    StaticStyleAttribute::FlexShrink(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::FlexBasis(__self_0) => {
                    StaticStyleAttribute::FlexBasis(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::RowGap(__self_0) => {
                    StaticStyleAttribute::RowGap(::core::clone::Clone::clone(__self_0))
                }
                StaticStyleAttribute::ColumnGap(__self_0) => {
                    StaticStyleAttribute::ColumnGap(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::GridAutoFlow(__self_0) => {
                    StaticStyleAttribute::GridAutoFlow(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::GridTemplateRows(__self_0) => {
                    StaticStyleAttribute::GridTemplateRows(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::GridTemplateColumns(__self_0) => {
                    StaticStyleAttribute::GridTemplateColumns(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::GridAutoRows(__self_0) => {
                    StaticStyleAttribute::GridAutoRows(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::GridAutoColumns(__self_0) => {
                    StaticStyleAttribute::GridAutoColumns(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::GridRow(__self_0) => {
                    StaticStyleAttribute::GridRow(::core::clone::Clone::clone(__self_0))
                }
                StaticStyleAttribute::GridColumn(__self_0) => {
                    StaticStyleAttribute::GridColumn(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::BackgroundColor(__self_0) => {
                    StaticStyleAttribute::BackgroundColor(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::BorderColor(__self_0) => {
                    StaticStyleAttribute::BorderColor(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::FocusPolicy(__self_0) => {
                    StaticStyleAttribute::FocusPolicy(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::Visibility(__self_0) => {
                    StaticStyleAttribute::Visibility(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::ZIndex(__self_0) => {
                    StaticStyleAttribute::ZIndex(::core::clone::Clone::clone(__self_0))
                }
                StaticStyleAttribute::Image(__self_0) => {
                    StaticStyleAttribute::Image(::core::clone::Clone::clone(__self_0))
                }
                StaticStyleAttribute::ImageScaleMode(__self_0) => {
                    StaticStyleAttribute::ImageScaleMode(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::FluxInteraction(__self_0) => {
                    StaticStyleAttribute::FluxInteraction(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::AbsolutePosition(__self_0) => {
                    StaticStyleAttribute::AbsolutePosition(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                StaticStyleAttribute::Custom(__self_0) => {
                    StaticStyleAttribute::Custom(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for StaticStyleAttribute {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                StaticStyleAttribute::Display(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Display",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::PositionType(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "PositionType",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::Overflow(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Overflow",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::Direction(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Direction",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::Left(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Left",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::Right(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Right",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::Top(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Top",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::Bottom(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Bottom",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::Width(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Width",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::Height(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Height",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::MinWidth(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "MinWidth",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::MinHeight(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "MinHeight",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::MaxWidth(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "MaxWidth",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::MaxHeight(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "MaxHeight",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::AspectRatio(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AspectRatio",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::AlignItems(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AlignItems",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::JustifyItems(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "JustifyItems",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::AlignSelf(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AlignSelf",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::JustifySelf(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "JustifySelf",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::AlignContent(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AlignContent",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::JustifyContents(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "JustifyContents",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::Margin(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Margin",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::Padding(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Padding",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::Border(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Border",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::FlexDirection(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FlexDirection",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::FlexWrap(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FlexWrap",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::FlexGrow(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FlexGrow",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::FlexShrink(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FlexShrink",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::FlexBasis(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FlexBasis",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::RowGap(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "RowGap",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::ColumnGap(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ColumnGap",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::GridAutoFlow(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "GridAutoFlow",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::GridTemplateRows(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "GridTemplateRows",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::GridTemplateColumns(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "GridTemplateColumns",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::GridAutoRows(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "GridAutoRows",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::GridAutoColumns(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "GridAutoColumns",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::GridRow(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "GridRow",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::GridColumn(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "GridColumn",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::BackgroundColor(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BackgroundColor",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::BorderColor(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BorderColor",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::FocusPolicy(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FocusPolicy",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::Visibility(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Visibility",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::ZIndex(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZIndex",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::Image(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Image",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::ImageScaleMode(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ImageScaleMode",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::FluxInteraction(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FluxInteraction",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::AbsolutePosition(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AbsolutePosition",
                        &__self_0,
                    )
                }
                StaticStyleAttribute::Custom(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Custom",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl PartialEq for StaticStyleAttribute {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Display(_), Self::Display(_)) => true,
                (Self::PositionType(_), Self::PositionType(_)) => true,
                (Self::Overflow(_), Self::Overflow(_)) => true,
                (Self::Direction(_), Self::Direction(_)) => true,
                (Self::Left(_), Self::Left(_)) => true,
                (Self::Right(_), Self::Right(_)) => true,
                (Self::Top(_), Self::Top(_)) => true,
                (Self::Bottom(_), Self::Bottom(_)) => true,
                (Self::Width(_), Self::Width(_)) => true,
                (Self::Height(_), Self::Height(_)) => true,
                (Self::MinWidth(_), Self::MinWidth(_)) => true,
                (Self::MinHeight(_), Self::MinHeight(_)) => true,
                (Self::MaxWidth(_), Self::MaxWidth(_)) => true,
                (Self::MaxHeight(_), Self::MaxHeight(_)) => true,
                (Self::AspectRatio(_), Self::AspectRatio(_)) => true,
                (Self::AlignItems(_), Self::AlignItems(_)) => true,
                (Self::JustifyItems(_), Self::JustifyItems(_)) => true,
                (Self::AlignSelf(_), Self::AlignSelf(_)) => true,
                (Self::JustifySelf(_), Self::JustifySelf(_)) => true,
                (Self::AlignContent(_), Self::AlignContent(_)) => true,
                (Self::JustifyContents(_), Self::JustifyContents(_)) => true,
                (Self::Margin(_), Self::Margin(_)) => true,
                (Self::Padding(_), Self::Padding(_)) => true,
                (Self::Border(_), Self::Border(_)) => true,
                (Self::FlexDirection(_), Self::FlexDirection(_)) => true,
                (Self::FlexWrap(_), Self::FlexWrap(_)) => true,
                (Self::FlexGrow(_), Self::FlexGrow(_)) => true,
                (Self::FlexShrink(_), Self::FlexShrink(_)) => true,
                (Self::FlexBasis(_), Self::FlexBasis(_)) => true,
                (Self::RowGap(_), Self::RowGap(_)) => true,
                (Self::ColumnGap(_), Self::ColumnGap(_)) => true,
                (Self::GridAutoFlow(_), Self::GridAutoFlow(_)) => true,
                (Self::GridTemplateRows(_), Self::GridTemplateRows(_)) => true,
                (Self::GridTemplateColumns(_), Self::GridTemplateColumns(_)) => true,
                (Self::GridAutoRows(_), Self::GridAutoRows(_)) => true,
                (Self::GridAutoColumns(_), Self::GridAutoColumns(_)) => true,
                (Self::GridRow(_), Self::GridRow(_)) => true,
                (Self::GridColumn(_), Self::GridColumn(_)) => true,
                (Self::BackgroundColor(_), Self::BackgroundColor(_)) => true,
                (Self::BorderColor(_), Self::BorderColor(_)) => true,
                (Self::FocusPolicy(_), Self::FocusPolicy(_)) => true,
                (Self::Visibility(_), Self::Visibility(_)) => true,
                (Self::ZIndex(_), Self::ZIndex(_)) => true,
                (Self::Image(_), Self::Image(_)) => true,
                (Self::ImageScaleMode(_), Self::ImageScaleMode(_)) => true,
                (Self::FluxInteraction(_), Self::FluxInteraction(_)) => true,
                (Self::AbsolutePosition(_), Self::AbsolutePosition(_)) => true,
                (Self::Custom(l0), Self::Custom(r0)) => l0 == r0,
                _ => false,
            }
        }
    }
    impl StaticStyleAttribute {
        pub fn apply<'a>(&self, ui_style: &'a mut UiStyle<'a>) {
            match self {
                Self::Display(value) => {
                    ui_style.display(value.clone());
                }
                Self::PositionType(value) => {
                    ui_style.position_type(value.clone());
                }
                Self::Overflow(value) => {
                    ui_style.overflow(value.clone());
                }
                Self::Direction(value) => {
                    ui_style.direction(value.clone());
                }
                Self::Left(value) => {
                    ui_style.left(value.clone());
                }
                Self::Right(value) => {
                    ui_style.right(value.clone());
                }
                Self::Top(value) => {
                    ui_style.top(value.clone());
                }
                Self::Bottom(value) => {
                    ui_style.bottom(value.clone());
                }
                Self::Width(value) => {
                    ui_style.width(value.clone());
                }
                Self::Height(value) => {
                    ui_style.height(value.clone());
                }
                Self::MinWidth(value) => {
                    ui_style.min_width(value.clone());
                }
                Self::MinHeight(value) => {
                    ui_style.min_height(value.clone());
                }
                Self::MaxWidth(value) => {
                    ui_style.max_width(value.clone());
                }
                Self::MaxHeight(value) => {
                    ui_style.max_height(value.clone());
                }
                Self::AspectRatio(value) => {
                    ui_style.aspect_ratio(value.clone());
                }
                Self::AlignItems(value) => {
                    ui_style.align_items(value.clone());
                }
                Self::JustifyItems(value) => {
                    ui_style.justify_items(value.clone());
                }
                Self::AlignSelf(value) => {
                    ui_style.align_self(value.clone());
                }
                Self::JustifySelf(value) => {
                    ui_style.justify_self(value.clone());
                }
                Self::AlignContent(value) => {
                    ui_style.align_content(value.clone());
                }
                Self::JustifyContents(value) => {
                    ui_style.justify_content(value.clone());
                }
                Self::Margin(value) => {
                    ui_style.margin(value.clone());
                }
                Self::Padding(value) => {
                    ui_style.padding(value.clone());
                }
                Self::Border(value) => {
                    ui_style.border(value.clone());
                }
                Self::FlexDirection(value) => {
                    ui_style.flex_direction(value.clone());
                }
                Self::FlexWrap(value) => {
                    ui_style.flex_wrap(value.clone());
                }
                Self::FlexGrow(value) => {
                    ui_style.flex_grow(value.clone());
                }
                Self::FlexShrink(value) => {
                    ui_style.flex_shrink(value.clone());
                }
                Self::FlexBasis(value) => {
                    ui_style.flex_basis(value.clone());
                }
                Self::RowGap(value) => {
                    ui_style.row_gap(value.clone());
                }
                Self::ColumnGap(value) => {
                    ui_style.column_gap(value.clone());
                }
                Self::GridAutoFlow(value) => {
                    ui_style.grid_auto_flow(value.clone());
                }
                Self::GridTemplateRows(value) => {
                    ui_style.grid_template_rows(value.clone());
                }
                Self::GridTemplateColumns(value) => {
                    ui_style.grid_template_columns(value.clone());
                }
                Self::GridAutoRows(value) => {
                    ui_style.grid_auto_rows(value.clone());
                }
                Self::GridAutoColumns(value) => {
                    ui_style.grid_auto_columns(value.clone());
                }
                Self::GridRow(value) => {
                    ui_style.grid_row(value.clone());
                }
                Self::GridColumn(value) => {
                    ui_style.grid_column(value.clone());
                }
                Self::BackgroundColor(value) => {
                    ui_style.background_color(value.clone());
                }
                Self::BorderColor(value) => {
                    ui_style.border_color(value.clone());
                }
                Self::FocusPolicy(value) => {
                    ui_style.focus_policy(value.clone());
                }
                Self::Visibility(value) => {
                    ui_style.visibility(value.clone());
                }
                Self::ZIndex(value) => {
                    ui_style.z_index(value.clone());
                }
                Self::Image(value) => {
                    ui_style.image(value.clone());
                }
                Self::ImageScaleMode(value) => {
                    ui_style.image_scale_mode(value.clone());
                }
                Self::FluxInteraction(value) => {
                    ui_style.flux_interaction_enabled(value.clone());
                }
                Self::AbsolutePosition(value) => {
                    ui_style.absolute_position(value.clone());
                }
                Self::Custom(callback) => {
                    ui_style.entity_commands().add(*callback);
                }
            }
        }
    }
    impl StyleBuilder {
        pub fn display(&mut self, display: impl Into<Display>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::Display(display.into()),
                ),
            );
            self
        }
        pub fn position_type(
            &mut self,
            position_type: impl Into<PositionType>,
        ) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::PositionType(position_type.into()),
                ),
            );
            self
        }
        pub fn overflow(&mut self, overflow: impl Into<Overflow>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::Overflow(overflow.into()),
                ),
            );
            self
        }
        pub fn direction(&mut self, direction: impl Into<Direction>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::Direction(direction.into()),
                ),
            );
            self
        }
        pub fn left(&mut self, left: impl Into<Val>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(StaticStyleAttribute::Left(left.into())),
            );
            self
        }
        pub fn right(&mut self, right: impl Into<Val>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(StaticStyleAttribute::Right(right.into())),
            );
            self
        }
        pub fn top(&mut self, top: impl Into<Val>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(StaticStyleAttribute::Top(top.into())),
            );
            self
        }
        pub fn bottom(&mut self, bottom: impl Into<Val>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::Bottom(bottom.into()),
                ),
            );
            self
        }
        pub fn width(&mut self, width: impl Into<Val>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(StaticStyleAttribute::Width(width.into())),
            );
            self
        }
        pub fn height(&mut self, height: impl Into<Val>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::Height(height.into()),
                ),
            );
            self
        }
        pub fn min_width(&mut self, min_width: impl Into<Val>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::MinWidth(min_width.into()),
                ),
            );
            self
        }
        pub fn min_height(&mut self, min_height: impl Into<Val>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::MinHeight(min_height.into()),
                ),
            );
            self
        }
        pub fn max_width(&mut self, max_width: impl Into<Val>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::MaxWidth(max_width.into()),
                ),
            );
            self
        }
        pub fn max_height(&mut self, max_height: impl Into<Val>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::MaxHeight(max_height.into()),
                ),
            );
            self
        }
        pub fn aspect_ratio(
            &mut self,
            aspect_ratio: impl Into<Option<f32>>,
        ) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::AspectRatio(aspect_ratio.into()),
                ),
            );
            self
        }
        pub fn align_items(&mut self, align_items: impl Into<AlignItems>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::AlignItems(align_items.into()),
                ),
            );
            self
        }
        pub fn justify_items(
            &mut self,
            justify_items: impl Into<JustifyItems>,
        ) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::JustifyItems(justify_items.into()),
                ),
            );
            self
        }
        pub fn align_self(&mut self, align_self: impl Into<AlignSelf>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::AlignSelf(align_self.into()),
                ),
            );
            self
        }
        pub fn justify_self(
            &mut self,
            justify_self: impl Into<JustifySelf>,
        ) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::JustifySelf(justify_self.into()),
                ),
            );
            self
        }
        pub fn align_content(
            &mut self,
            align_content: impl Into<AlignContent>,
        ) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::AlignContent(align_content.into()),
                ),
            );
            self
        }
        pub fn justify_content(
            &mut self,
            justify_content: impl Into<JustifyContent>,
        ) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::JustifyContents(justify_content.into()),
                ),
            );
            self
        }
        pub fn margin(&mut self, margin: impl Into<UiRect>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::Margin(margin.into()),
                ),
            );
            self
        }
        pub fn padding(&mut self, padding: impl Into<UiRect>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::Padding(padding.into()),
                ),
            );
            self
        }
        pub fn border(&mut self, border: impl Into<UiRect>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::Border(border.into()),
                ),
            );
            self
        }
        pub fn flex_direction(
            &mut self,
            flex_direction: impl Into<FlexDirection>,
        ) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::FlexDirection(flex_direction.into()),
                ),
            );
            self
        }
        pub fn flex_wrap(&mut self, flex_wrap: impl Into<FlexWrap>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::FlexWrap(flex_wrap.into()),
                ),
            );
            self
        }
        pub fn flex_grow(&mut self, flex_grow: impl Into<f32>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::FlexGrow(flex_grow.into()),
                ),
            );
            self
        }
        pub fn flex_shrink(&mut self, flex_shrink: impl Into<f32>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::FlexShrink(flex_shrink.into()),
                ),
            );
            self
        }
        pub fn flex_basis(&mut self, flex_basis: impl Into<Val>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::FlexBasis(flex_basis.into()),
                ),
            );
            self
        }
        pub fn row_gap(&mut self, row_gap: impl Into<Val>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::RowGap(row_gap.into()),
                ),
            );
            self
        }
        pub fn column_gap(&mut self, column_gap: impl Into<Val>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::ColumnGap(column_gap.into()),
                ),
            );
            self
        }
        pub fn grid_auto_flow(
            &mut self,
            grid_auto_flow: impl Into<GridAutoFlow>,
        ) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::GridAutoFlow(grid_auto_flow.into()),
                ),
            );
            self
        }
        pub fn grid_template_rows(
            &mut self,
            grid_template_rows: impl Into<Vec<RepeatedGridTrack>>,
        ) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::GridTemplateRows(grid_template_rows.into()),
                ),
            );
            self
        }
        pub fn grid_template_columns(
            &mut self,
            grid_template_columns: impl Into<Vec<RepeatedGridTrack>>,
        ) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::GridTemplateColumns(
                        grid_template_columns.into(),
                    ),
                ),
            );
            self
        }
        pub fn grid_auto_rows(
            &mut self,
            grid_auto_rows: impl Into<Vec<GridTrack>>,
        ) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::GridAutoRows(grid_auto_rows.into()),
                ),
            );
            self
        }
        pub fn grid_auto_columns(
            &mut self,
            grid_auto_columns: impl Into<Vec<GridTrack>>,
        ) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::GridAutoColumns(grid_auto_columns.into()),
                ),
            );
            self
        }
        pub fn grid_row(&mut self, grid_row: impl Into<GridPlacement>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::GridRow(grid_row.into()),
                ),
            );
            self
        }
        pub fn grid_column(
            &mut self,
            grid_column: impl Into<GridPlacement>,
        ) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::GridColumn(grid_column.into()),
                ),
            );
            self
        }
        pub fn background_color(
            &mut self,
            background_color: impl Into<Color>,
        ) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::BackgroundColor(background_color.into()),
                ),
            );
            self
        }
        pub fn border_color(&mut self, border_color: impl Into<Color>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::BorderColor(border_color.into()),
                ),
            );
            self
        }
        pub fn focus_policy(
            &mut self,
            focus_policy: impl Into<FocusPolicy>,
        ) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::FocusPolicy(focus_policy.into()),
                ),
            );
            self
        }
        pub fn visibility(&mut self, visibility: impl Into<Visibility>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::Visibility(visibility.into()),
                ),
            );
            self
        }
        pub fn z_index(&mut self, z_index: impl Into<ZIndex>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::ZIndex(z_index.into()),
                ),
            );
            self
        }
        pub fn image(&mut self, image: impl Into<String>) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(StaticStyleAttribute::Image(image.into())),
            );
            self
        }
        pub fn image_scale_mode(
            &mut self,
            image_scale_mode: impl Into<Option<ImageScaleMode>>,
        ) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::ImageScaleMode(image_scale_mode.into()),
                ),
            );
            self
        }
        pub fn flux_interaction_enabled(
            &mut self,
            flux_interaction_enabled: impl Into<bool>,
        ) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::FluxInteraction(
                        flux_interaction_enabled.into(),
                    ),
                ),
            );
            self
        }
        pub fn absolute_position(
            &mut self,
            absolute_position: impl Into<Vec2>,
        ) -> &mut Self {
            self.add(
                DynamicStyleAttribute::Static(
                    StaticStyleAttribute::AbsolutePosition(absolute_position.into()),
                ),
            );
            self
        }
    }
    pub enum LockableStyleAttribute {
        Display,
        PositionType,
        Overflow,
        Direction,
        Left,
        Right,
        Top,
        Bottom,
        Width,
        Height,
        MinWidth,
        MinHeight,
        MaxWidth,
        MaxHeight,
        AspectRatio,
        AlignItems,
        JustifyItems,
        AlignSelf,
        JustifySelf,
        AlignContent,
        JustifyContents,
        Margin,
        Padding,
        Border,
        FlexDirection,
        FlexWrap,
        FlexGrow,
        FlexShrink,
        FlexBasis,
        RowGap,
        ColumnGap,
        GridAutoFlow,
        GridTemplateRows,
        GridTemplateColumns,
        GridAutoRows,
        GridAutoColumns,
        GridRow,
        GridColumn,
        BackgroundColor,
        BorderColor,
        FocusPolicy,
        Visibility,
        ZIndex,
        Image,
        ImageScaleMode,
        FluxInteraction,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for LockableStyleAttribute {
        #[inline]
        fn clone(&self) -> LockableStyleAttribute {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for LockableStyleAttribute {}
    #[automatically_derived]
    impl ::core::fmt::Debug for LockableStyleAttribute {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    LockableStyleAttribute::Display => "Display",
                    LockableStyleAttribute::PositionType => "PositionType",
                    LockableStyleAttribute::Overflow => "Overflow",
                    LockableStyleAttribute::Direction => "Direction",
                    LockableStyleAttribute::Left => "Left",
                    LockableStyleAttribute::Right => "Right",
                    LockableStyleAttribute::Top => "Top",
                    LockableStyleAttribute::Bottom => "Bottom",
                    LockableStyleAttribute::Width => "Width",
                    LockableStyleAttribute::Height => "Height",
                    LockableStyleAttribute::MinWidth => "MinWidth",
                    LockableStyleAttribute::MinHeight => "MinHeight",
                    LockableStyleAttribute::MaxWidth => "MaxWidth",
                    LockableStyleAttribute::MaxHeight => "MaxHeight",
                    LockableStyleAttribute::AspectRatio => "AspectRatio",
                    LockableStyleAttribute::AlignItems => "AlignItems",
                    LockableStyleAttribute::JustifyItems => "JustifyItems",
                    LockableStyleAttribute::AlignSelf => "AlignSelf",
                    LockableStyleAttribute::JustifySelf => "JustifySelf",
                    LockableStyleAttribute::AlignContent => "AlignContent",
                    LockableStyleAttribute::JustifyContents => "JustifyContents",
                    LockableStyleAttribute::Margin => "Margin",
                    LockableStyleAttribute::Padding => "Padding",
                    LockableStyleAttribute::Border => "Border",
                    LockableStyleAttribute::FlexDirection => "FlexDirection",
                    LockableStyleAttribute::FlexWrap => "FlexWrap",
                    LockableStyleAttribute::FlexGrow => "FlexGrow",
                    LockableStyleAttribute::FlexShrink => "FlexShrink",
                    LockableStyleAttribute::FlexBasis => "FlexBasis",
                    LockableStyleAttribute::RowGap => "RowGap",
                    LockableStyleAttribute::ColumnGap => "ColumnGap",
                    LockableStyleAttribute::GridAutoFlow => "GridAutoFlow",
                    LockableStyleAttribute::GridTemplateRows => "GridTemplateRows",
                    LockableStyleAttribute::GridTemplateColumns => "GridTemplateColumns",
                    LockableStyleAttribute::GridAutoRows => "GridAutoRows",
                    LockableStyleAttribute::GridAutoColumns => "GridAutoColumns",
                    LockableStyleAttribute::GridRow => "GridRow",
                    LockableStyleAttribute::GridColumn => "GridColumn",
                    LockableStyleAttribute::BackgroundColor => "BackgroundColor",
                    LockableStyleAttribute::BorderColor => "BorderColor",
                    LockableStyleAttribute::FocusPolicy => "FocusPolicy",
                    LockableStyleAttribute::Visibility => "Visibility",
                    LockableStyleAttribute::ZIndex => "ZIndex",
                    LockableStyleAttribute::Image => "Image",
                    LockableStyleAttribute::ImageScaleMode => "ImageScaleMode",
                    LockableStyleAttribute::FluxInteraction => "FluxInteraction",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for LockableStyleAttribute {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for LockableStyleAttribute {
        #[inline]
        fn eq(&self, other: &LockableStyleAttribute) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for LockableStyleAttribute {}
    #[automatically_derived]
    impl ::core::cmp::Eq for LockableStyleAttribute {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for LockableStyleAttribute {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    const _: () = {
        #[allow(unused_mut)]
        impl bevy_reflect::GetTypeRegistration for LockableStyleAttribute
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn get_type_registration() -> bevy_reflect::TypeRegistration {
                let mut registration = bevy_reflect::TypeRegistration::of::<Self>();
                registration
                    .insert::<
                        bevy_reflect::ReflectFromPtr,
                    >(bevy_reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        bevy_reflect::ReflectFromReflect,
                    >(bevy_reflect::FromType::<Self>::from_type());
                registration
            }
        }
        impl bevy_reflect::Typed for LockableStyleAttribute
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn type_info() -> &'static bevy_reflect::TypeInfo {
                static CELL: bevy_reflect::utility::NonGenericTypeInfoCell = bevy_reflect::utility::NonGenericTypeInfoCell::new();
                CELL.get_or_set(|| {
                    let variants = [
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Display"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("PositionType"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Overflow"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Direction"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Left"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Right"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Top"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Bottom"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Width"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Height"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("MinWidth"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("MinHeight"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("MaxWidth"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("MaxHeight"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("AspectRatio"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("AlignItems"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("JustifyItems"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("AlignSelf"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("JustifySelf"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("AlignContent"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("JustifyContents"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Margin"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Padding"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Border"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("FlexDirection"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("FlexWrap"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("FlexGrow"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("FlexShrink"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("FlexBasis"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("RowGap"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("ColumnGap"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("GridAutoFlow"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("GridTemplateRows"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("GridTemplateColumns"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("GridAutoRows"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("GridAutoColumns"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("GridRow"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("GridColumn"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("BackgroundColor"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("BorderColor"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("FocusPolicy"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Visibility"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("ZIndex"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Image"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("ImageScaleMode"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("FluxInteraction"),
                        ),
                    ];
                    let info = bevy_reflect::EnumInfo::new::<Self>(&variants);
                    bevy_reflect::TypeInfo::Enum(info)
                })
            }
        }
        impl bevy_reflect::TypePath for LockableStyleAttribute
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn type_path() -> &'static str {
                "sickle_ui::ui_style::LockableStyleAttribute"
            }
            fn short_type_path() -> &'static str {
                "LockableStyleAttribute"
            }
            fn type_ident() -> Option<&'static str> {
                ::core::option::Option::Some("LockableStyleAttribute")
            }
            fn crate_name() -> Option<&'static str> {
                ::core::option::Option::Some(
                    "sickle_ui::ui_style".split(':').next().unwrap(),
                )
            }
            fn module_path() -> Option<&'static str> {
                ::core::option::Option::Some("sickle_ui::ui_style")
            }
        }
        impl bevy_reflect::Enum for LockableStyleAttribute
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn field(
                &self,
                __name_param: &str,
            ) -> ::core::option::Option<&dyn bevy_reflect::Reflect> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at(
                &self,
                __index_param: usize,
            ) -> ::core::option::Option<&dyn bevy_reflect::Reflect> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn field_mut(
                &mut self,
                __name_param: &str,
            ) -> ::core::option::Option<&mut dyn bevy_reflect::Reflect> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at_mut(
                &mut self,
                __index_param: usize,
            ) -> ::core::option::Option<&mut dyn bevy_reflect::Reflect> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn index_of(&self, __name_param: &str) -> ::core::option::Option<usize> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn name_at(&self, __index_param: usize) -> ::core::option::Option<&str> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn iter_fields(&self) -> bevy_reflect::VariantFieldIter {
                bevy_reflect::VariantFieldIter::new(self)
            }
            #[inline]
            fn field_len(&self) -> usize {
                match self {
                    LockableStyleAttribute::Display { .. } => 0usize,
                    LockableStyleAttribute::PositionType { .. } => 0usize,
                    LockableStyleAttribute::Overflow { .. } => 0usize,
                    LockableStyleAttribute::Direction { .. } => 0usize,
                    LockableStyleAttribute::Left { .. } => 0usize,
                    LockableStyleAttribute::Right { .. } => 0usize,
                    LockableStyleAttribute::Top { .. } => 0usize,
                    LockableStyleAttribute::Bottom { .. } => 0usize,
                    LockableStyleAttribute::Width { .. } => 0usize,
                    LockableStyleAttribute::Height { .. } => 0usize,
                    LockableStyleAttribute::MinWidth { .. } => 0usize,
                    LockableStyleAttribute::MinHeight { .. } => 0usize,
                    LockableStyleAttribute::MaxWidth { .. } => 0usize,
                    LockableStyleAttribute::MaxHeight { .. } => 0usize,
                    LockableStyleAttribute::AspectRatio { .. } => 0usize,
                    LockableStyleAttribute::AlignItems { .. } => 0usize,
                    LockableStyleAttribute::JustifyItems { .. } => 0usize,
                    LockableStyleAttribute::AlignSelf { .. } => 0usize,
                    LockableStyleAttribute::JustifySelf { .. } => 0usize,
                    LockableStyleAttribute::AlignContent { .. } => 0usize,
                    LockableStyleAttribute::JustifyContents { .. } => 0usize,
                    LockableStyleAttribute::Margin { .. } => 0usize,
                    LockableStyleAttribute::Padding { .. } => 0usize,
                    LockableStyleAttribute::Border { .. } => 0usize,
                    LockableStyleAttribute::FlexDirection { .. } => 0usize,
                    LockableStyleAttribute::FlexWrap { .. } => 0usize,
                    LockableStyleAttribute::FlexGrow { .. } => 0usize,
                    LockableStyleAttribute::FlexShrink { .. } => 0usize,
                    LockableStyleAttribute::FlexBasis { .. } => 0usize,
                    LockableStyleAttribute::RowGap { .. } => 0usize,
                    LockableStyleAttribute::ColumnGap { .. } => 0usize,
                    LockableStyleAttribute::GridAutoFlow { .. } => 0usize,
                    LockableStyleAttribute::GridTemplateRows { .. } => 0usize,
                    LockableStyleAttribute::GridTemplateColumns { .. } => 0usize,
                    LockableStyleAttribute::GridAutoRows { .. } => 0usize,
                    LockableStyleAttribute::GridAutoColumns { .. } => 0usize,
                    LockableStyleAttribute::GridRow { .. } => 0usize,
                    LockableStyleAttribute::GridColumn { .. } => 0usize,
                    LockableStyleAttribute::BackgroundColor { .. } => 0usize,
                    LockableStyleAttribute::BorderColor { .. } => 0usize,
                    LockableStyleAttribute::FocusPolicy { .. } => 0usize,
                    LockableStyleAttribute::Visibility { .. } => 0usize,
                    LockableStyleAttribute::ZIndex { .. } => 0usize,
                    LockableStyleAttribute::Image { .. } => 0usize,
                    LockableStyleAttribute::ImageScaleMode { .. } => 0usize,
                    LockableStyleAttribute::FluxInteraction { .. } => 0usize,
                    _ => 0,
                }
            }
            #[inline]
            fn variant_name(&self) -> &str {
                match self {
                    LockableStyleAttribute::Display { .. } => "Display",
                    LockableStyleAttribute::PositionType { .. } => "PositionType",
                    LockableStyleAttribute::Overflow { .. } => "Overflow",
                    LockableStyleAttribute::Direction { .. } => "Direction",
                    LockableStyleAttribute::Left { .. } => "Left",
                    LockableStyleAttribute::Right { .. } => "Right",
                    LockableStyleAttribute::Top { .. } => "Top",
                    LockableStyleAttribute::Bottom { .. } => "Bottom",
                    LockableStyleAttribute::Width { .. } => "Width",
                    LockableStyleAttribute::Height { .. } => "Height",
                    LockableStyleAttribute::MinWidth { .. } => "MinWidth",
                    LockableStyleAttribute::MinHeight { .. } => "MinHeight",
                    LockableStyleAttribute::MaxWidth { .. } => "MaxWidth",
                    LockableStyleAttribute::MaxHeight { .. } => "MaxHeight",
                    LockableStyleAttribute::AspectRatio { .. } => "AspectRatio",
                    LockableStyleAttribute::AlignItems { .. } => "AlignItems",
                    LockableStyleAttribute::JustifyItems { .. } => "JustifyItems",
                    LockableStyleAttribute::AlignSelf { .. } => "AlignSelf",
                    LockableStyleAttribute::JustifySelf { .. } => "JustifySelf",
                    LockableStyleAttribute::AlignContent { .. } => "AlignContent",
                    LockableStyleAttribute::JustifyContents { .. } => "JustifyContents",
                    LockableStyleAttribute::Margin { .. } => "Margin",
                    LockableStyleAttribute::Padding { .. } => "Padding",
                    LockableStyleAttribute::Border { .. } => "Border",
                    LockableStyleAttribute::FlexDirection { .. } => "FlexDirection",
                    LockableStyleAttribute::FlexWrap { .. } => "FlexWrap",
                    LockableStyleAttribute::FlexGrow { .. } => "FlexGrow",
                    LockableStyleAttribute::FlexShrink { .. } => "FlexShrink",
                    LockableStyleAttribute::FlexBasis { .. } => "FlexBasis",
                    LockableStyleAttribute::RowGap { .. } => "RowGap",
                    LockableStyleAttribute::ColumnGap { .. } => "ColumnGap",
                    LockableStyleAttribute::GridAutoFlow { .. } => "GridAutoFlow",
                    LockableStyleAttribute::GridTemplateRows { .. } => "GridTemplateRows",
                    LockableStyleAttribute::GridTemplateColumns { .. } => {
                        "GridTemplateColumns"
                    }
                    LockableStyleAttribute::GridAutoRows { .. } => "GridAutoRows",
                    LockableStyleAttribute::GridAutoColumns { .. } => "GridAutoColumns",
                    LockableStyleAttribute::GridRow { .. } => "GridRow",
                    LockableStyleAttribute::GridColumn { .. } => "GridColumn",
                    LockableStyleAttribute::BackgroundColor { .. } => "BackgroundColor",
                    LockableStyleAttribute::BorderColor { .. } => "BorderColor",
                    LockableStyleAttribute::FocusPolicy { .. } => "FocusPolicy",
                    LockableStyleAttribute::Visibility { .. } => "Visibility",
                    LockableStyleAttribute::ZIndex { .. } => "ZIndex",
                    LockableStyleAttribute::Image { .. } => "Image",
                    LockableStyleAttribute::ImageScaleMode { .. } => "ImageScaleMode",
                    LockableStyleAttribute::FluxInteraction { .. } => "FluxInteraction",
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
            #[inline]
            fn variant_index(&self) -> usize {
                match self {
                    LockableStyleAttribute::Display { .. } => 0usize,
                    LockableStyleAttribute::PositionType { .. } => 1usize,
                    LockableStyleAttribute::Overflow { .. } => 2usize,
                    LockableStyleAttribute::Direction { .. } => 3usize,
                    LockableStyleAttribute::Left { .. } => 4usize,
                    LockableStyleAttribute::Right { .. } => 5usize,
                    LockableStyleAttribute::Top { .. } => 6usize,
                    LockableStyleAttribute::Bottom { .. } => 7usize,
                    LockableStyleAttribute::Width { .. } => 8usize,
                    LockableStyleAttribute::Height { .. } => 9usize,
                    LockableStyleAttribute::MinWidth { .. } => 10usize,
                    LockableStyleAttribute::MinHeight { .. } => 11usize,
                    LockableStyleAttribute::MaxWidth { .. } => 12usize,
                    LockableStyleAttribute::MaxHeight { .. } => 13usize,
                    LockableStyleAttribute::AspectRatio { .. } => 14usize,
                    LockableStyleAttribute::AlignItems { .. } => 15usize,
                    LockableStyleAttribute::JustifyItems { .. } => 16usize,
                    LockableStyleAttribute::AlignSelf { .. } => 17usize,
                    LockableStyleAttribute::JustifySelf { .. } => 18usize,
                    LockableStyleAttribute::AlignContent { .. } => 19usize,
                    LockableStyleAttribute::JustifyContents { .. } => 20usize,
                    LockableStyleAttribute::Margin { .. } => 21usize,
                    LockableStyleAttribute::Padding { .. } => 22usize,
                    LockableStyleAttribute::Border { .. } => 23usize,
                    LockableStyleAttribute::FlexDirection { .. } => 24usize,
                    LockableStyleAttribute::FlexWrap { .. } => 25usize,
                    LockableStyleAttribute::FlexGrow { .. } => 26usize,
                    LockableStyleAttribute::FlexShrink { .. } => 27usize,
                    LockableStyleAttribute::FlexBasis { .. } => 28usize,
                    LockableStyleAttribute::RowGap { .. } => 29usize,
                    LockableStyleAttribute::ColumnGap { .. } => 30usize,
                    LockableStyleAttribute::GridAutoFlow { .. } => 31usize,
                    LockableStyleAttribute::GridTemplateRows { .. } => 32usize,
                    LockableStyleAttribute::GridTemplateColumns { .. } => 33usize,
                    LockableStyleAttribute::GridAutoRows { .. } => 34usize,
                    LockableStyleAttribute::GridAutoColumns { .. } => 35usize,
                    LockableStyleAttribute::GridRow { .. } => 36usize,
                    LockableStyleAttribute::GridColumn { .. } => 37usize,
                    LockableStyleAttribute::BackgroundColor { .. } => 38usize,
                    LockableStyleAttribute::BorderColor { .. } => 39usize,
                    LockableStyleAttribute::FocusPolicy { .. } => 40usize,
                    LockableStyleAttribute::Visibility { .. } => 41usize,
                    LockableStyleAttribute::ZIndex { .. } => 42usize,
                    LockableStyleAttribute::Image { .. } => 43usize,
                    LockableStyleAttribute::ImageScaleMode { .. } => 44usize,
                    LockableStyleAttribute::FluxInteraction { .. } => 45usize,
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
            #[inline]
            fn variant_type(&self) -> bevy_reflect::VariantType {
                match self {
                    LockableStyleAttribute::Display { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::PositionType { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::Overflow { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::Direction { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::Left { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::Right { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::Top { .. } => bevy_reflect::VariantType::Unit,
                    LockableStyleAttribute::Bottom { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::Width { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::Height { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::MinWidth { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::MinHeight { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::MaxWidth { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::MaxHeight { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::AspectRatio { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::AlignItems { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::JustifyItems { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::AlignSelf { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::JustifySelf { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::AlignContent { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::JustifyContents { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::Margin { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::Padding { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::Border { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::FlexDirection { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::FlexWrap { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::FlexGrow { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::FlexShrink { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::FlexBasis { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::RowGap { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::ColumnGap { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::GridAutoFlow { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::GridTemplateRows { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::GridTemplateColumns { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::GridAutoRows { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::GridAutoColumns { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::GridRow { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::GridColumn { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::BackgroundColor { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::BorderColor { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::FocusPolicy { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::Visibility { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::ZIndex { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::Image { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::ImageScaleMode { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    LockableStyleAttribute::FluxInteraction { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
            fn clone_dynamic(&self) -> bevy_reflect::DynamicEnum {
                bevy_reflect::DynamicEnum::from_ref::<Self>(self)
            }
        }
        impl bevy_reflect::Reflect for LockableStyleAttribute
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            #[inline]
            fn get_represented_type_info(
                &self,
            ) -> ::core::option::Option<&'static bevy_reflect::TypeInfo> {
                ::core::option::Option::Some(<Self as bevy_reflect::Typed>::type_info())
            }
            #[inline]
            fn into_any(
                self: ::std::boxed::Box<Self>,
            ) -> ::std::boxed::Box<dyn ::core::any::Any> {
                self
            }
            #[inline]
            fn as_any(&self) -> &dyn ::core::any::Any {
                self
            }
            #[inline]
            fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                self
            }
            #[inline]
            fn into_reflect(
                self: ::std::boxed::Box<Self>,
            ) -> ::std::boxed::Box<dyn bevy_reflect::Reflect> {
                self
            }
            #[inline]
            fn as_reflect(&self) -> &dyn bevy_reflect::Reflect {
                self
            }
            #[inline]
            fn as_reflect_mut(&mut self) -> &mut dyn bevy_reflect::Reflect {
                self
            }
            #[inline]
            fn clone_value(&self) -> ::std::boxed::Box<dyn bevy_reflect::Reflect> {
                ::std::boxed::Box::new(bevy_reflect::Enum::clone_dynamic(self))
            }
            #[inline]
            fn set(
                &mut self,
                __value_param: ::std::boxed::Box<dyn bevy_reflect::Reflect>,
            ) -> ::core::result::Result<
                (),
                ::std::boxed::Box<dyn bevy_reflect::Reflect>,
            > {
                *self = <dyn bevy_reflect::Reflect>::take(__value_param)?;
                ::core::result::Result::Ok(())
            }
            #[inline]
            fn apply(&mut self, __value_param: &dyn bevy_reflect::Reflect) {
                if let bevy_reflect::ReflectRef::Enum(__value_param) = bevy_reflect::Reflect::reflect_ref(
                    __value_param,
                ) {
                    if bevy_reflect::Enum::variant_name(self)
                        == bevy_reflect::Enum::variant_name(__value_param)
                    {
                        match bevy_reflect::Enum::variant_type(__value_param) {
                            bevy_reflect::VariantType::Struct => {
                                for field in bevy_reflect::Enum::iter_fields(
                                    __value_param,
                                ) {
                                    let name = field.name().unwrap();
                                    bevy_reflect::Enum::field_mut(self, name)
                                        .map(|v| v.apply(field.value()));
                                }
                            }
                            bevy_reflect::VariantType::Tuple => {
                                for (index, field) in ::core::iter::Iterator::enumerate(
                                    bevy_reflect::Enum::iter_fields(__value_param),
                                ) {
                                    bevy_reflect::Enum::field_at_mut(self, index)
                                        .map(|v| v.apply(field.value()));
                                }
                            }
                            _ => {}
                        }
                    } else {
                        match bevy_reflect::Enum::variant_name(__value_param) {
                            "Display" => *self = LockableStyleAttribute::Display {},
                            "PositionType" => {
                                *self = LockableStyleAttribute::PositionType {
                                };
                            }
                            "Overflow" => {
                                *self = LockableStyleAttribute::Overflow {
                                };
                            }
                            "Direction" => {
                                *self = LockableStyleAttribute::Direction {
                                };
                            }
                            "Left" => *self = LockableStyleAttribute::Left {},
                            "Right" => *self = LockableStyleAttribute::Right {},
                            "Top" => *self = LockableStyleAttribute::Top {},
                            "Bottom" => *self = LockableStyleAttribute::Bottom {},
                            "Width" => *self = LockableStyleAttribute::Width {},
                            "Height" => *self = LockableStyleAttribute::Height {},
                            "MinWidth" => {
                                *self = LockableStyleAttribute::MinWidth {
                                };
                            }
                            "MinHeight" => {
                                *self = LockableStyleAttribute::MinHeight {
                                };
                            }
                            "MaxWidth" => {
                                *self = LockableStyleAttribute::MaxWidth {
                                };
                            }
                            "MaxHeight" => {
                                *self = LockableStyleAttribute::MaxHeight {
                                };
                            }
                            "AspectRatio" => {
                                *self = LockableStyleAttribute::AspectRatio {
                                };
                            }
                            "AlignItems" => {
                                *self = LockableStyleAttribute::AlignItems {
                                };
                            }
                            "JustifyItems" => {
                                *self = LockableStyleAttribute::JustifyItems {
                                };
                            }
                            "AlignSelf" => {
                                *self = LockableStyleAttribute::AlignSelf {
                                };
                            }
                            "JustifySelf" => {
                                *self = LockableStyleAttribute::JustifySelf {
                                };
                            }
                            "AlignContent" => {
                                *self = LockableStyleAttribute::AlignContent {
                                };
                            }
                            "JustifyContents" => {
                                *self = LockableStyleAttribute::JustifyContents {
                                };
                            }
                            "Margin" => *self = LockableStyleAttribute::Margin {},
                            "Padding" => *self = LockableStyleAttribute::Padding {},
                            "Border" => *self = LockableStyleAttribute::Border {},
                            "FlexDirection" => {
                                *self = LockableStyleAttribute::FlexDirection {
                                };
                            }
                            "FlexWrap" => {
                                *self = LockableStyleAttribute::FlexWrap {
                                };
                            }
                            "FlexGrow" => {
                                *self = LockableStyleAttribute::FlexGrow {
                                };
                            }
                            "FlexShrink" => {
                                *self = LockableStyleAttribute::FlexShrink {
                                };
                            }
                            "FlexBasis" => {
                                *self = LockableStyleAttribute::FlexBasis {
                                };
                            }
                            "RowGap" => *self = LockableStyleAttribute::RowGap {},
                            "ColumnGap" => {
                                *self = LockableStyleAttribute::ColumnGap {
                                };
                            }
                            "GridAutoFlow" => {
                                *self = LockableStyleAttribute::GridAutoFlow {
                                };
                            }
                            "GridTemplateRows" => {
                                *self = LockableStyleAttribute::GridTemplateRows {
                                };
                            }
                            "GridTemplateColumns" => {
                                *self = LockableStyleAttribute::GridTemplateColumns {
                                };
                            }
                            "GridAutoRows" => {
                                *self = LockableStyleAttribute::GridAutoRows {
                                };
                            }
                            "GridAutoColumns" => {
                                *self = LockableStyleAttribute::GridAutoColumns {
                                };
                            }
                            "GridRow" => *self = LockableStyleAttribute::GridRow {},
                            "GridColumn" => {
                                *self = LockableStyleAttribute::GridColumn {
                                };
                            }
                            "BackgroundColor" => {
                                *self = LockableStyleAttribute::BackgroundColor {
                                };
                            }
                            "BorderColor" => {
                                *self = LockableStyleAttribute::BorderColor {
                                };
                            }
                            "FocusPolicy" => {
                                *self = LockableStyleAttribute::FocusPolicy {
                                };
                            }
                            "Visibility" => {
                                *self = LockableStyleAttribute::Visibility {
                                };
                            }
                            "ZIndex" => *self = LockableStyleAttribute::ZIndex {},
                            "Image" => *self = LockableStyleAttribute::Image {},
                            "ImageScaleMode" => {
                                *self = LockableStyleAttribute::ImageScaleMode {
                                };
                            }
                            "FluxInteraction" => {
                                *self = LockableStyleAttribute::FluxInteraction {
                                };
                            }
                            name => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "variant with name `{0}` does not exist on enum `{1}`",
                                        name,
                                        <Self as bevy_reflect::TypePath>::type_path(),
                                    ),
                                );
                            }
                        }
                    }
                } else {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "`{0}` is not an enum",
                                bevy_reflect::DynamicTypePath::reflect_type_path(
                                    __value_param,
                                ),
                            ),
                        );
                    };
                }
            }
            fn reflect_kind(&self) -> bevy_reflect::ReflectKind {
                bevy_reflect::ReflectKind::Enum
            }
            fn reflect_ref(&self) -> bevy_reflect::ReflectRef {
                bevy_reflect::ReflectRef::Enum(self)
            }
            fn reflect_mut(&mut self) -> bevy_reflect::ReflectMut {
                bevy_reflect::ReflectMut::Enum(self)
            }
            fn reflect_owned(
                self: ::std::boxed::Box<Self>,
            ) -> bevy_reflect::ReflectOwned {
                bevy_reflect::ReflectOwned::Enum(self)
            }
            fn reflect_hash(&self) -> ::core::option::Option<u64> {
                bevy_reflect::enum_hash(self)
            }
            fn reflect_partial_eq(
                &self,
                value: &dyn bevy_reflect::Reflect,
            ) -> ::core::option::Option<bool> {
                bevy_reflect::enum_partial_eq(self, value)
            }
        }
        impl bevy_reflect::FromReflect for LockableStyleAttribute
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn from_reflect(
                __param0: &dyn bevy_reflect::Reflect,
            ) -> ::core::option::Option<Self> {
                if let bevy_reflect::ReflectRef::Enum(__param0) = bevy_reflect::Reflect::reflect_ref(
                    __param0,
                ) {
                    match bevy_reflect::Enum::variant_name(__param0) {
                        "Display" => {
                            ::core::option::Option::Some(LockableStyleAttribute::Display {})
                        }
                        "PositionType" => {
                            ::core::option::Option::Some(LockableStyleAttribute::PositionType {})
                        }
                        "Overflow" => {
                            ::core::option::Option::Some(LockableStyleAttribute::Overflow {})
                        }
                        "Direction" => {
                            ::core::option::Option::Some(LockableStyleAttribute::Direction {})
                        }
                        "Left" => {
                            ::core::option::Option::Some(LockableStyleAttribute::Left {})
                        }
                        "Right" => {
                            ::core::option::Option::Some(LockableStyleAttribute::Right {})
                        }
                        "Top" => {
                            ::core::option::Option::Some(LockableStyleAttribute::Top {})
                        }
                        "Bottom" => {
                            ::core::option::Option::Some(LockableStyleAttribute::Bottom {})
                        }
                        "Width" => {
                            ::core::option::Option::Some(LockableStyleAttribute::Width {})
                        }
                        "Height" => {
                            ::core::option::Option::Some(LockableStyleAttribute::Height {})
                        }
                        "MinWidth" => {
                            ::core::option::Option::Some(LockableStyleAttribute::MinWidth {})
                        }
                        "MinHeight" => {
                            ::core::option::Option::Some(LockableStyleAttribute::MinHeight {})
                        }
                        "MaxWidth" => {
                            ::core::option::Option::Some(LockableStyleAttribute::MaxWidth {})
                        }
                        "MaxHeight" => {
                            ::core::option::Option::Some(LockableStyleAttribute::MaxHeight {})
                        }
                        "AspectRatio" => {
                            ::core::option::Option::Some(LockableStyleAttribute::AspectRatio {})
                        }
                        "AlignItems" => {
                            ::core::option::Option::Some(LockableStyleAttribute::AlignItems {})
                        }
                        "JustifyItems" => {
                            ::core::option::Option::Some(LockableStyleAttribute::JustifyItems {})
                        }
                        "AlignSelf" => {
                            ::core::option::Option::Some(LockableStyleAttribute::AlignSelf {})
                        }
                        "JustifySelf" => {
                            ::core::option::Option::Some(LockableStyleAttribute::JustifySelf {})
                        }
                        "AlignContent" => {
                            ::core::option::Option::Some(LockableStyleAttribute::AlignContent {})
                        }
                        "JustifyContents" => {
                            ::core::option::Option::Some(LockableStyleAttribute::JustifyContents {})
                        }
                        "Margin" => {
                            ::core::option::Option::Some(LockableStyleAttribute::Margin {})
                        }
                        "Padding" => {
                            ::core::option::Option::Some(LockableStyleAttribute::Padding {})
                        }
                        "Border" => {
                            ::core::option::Option::Some(LockableStyleAttribute::Border {})
                        }
                        "FlexDirection" => {
                            ::core::option::Option::Some(LockableStyleAttribute::FlexDirection {})
                        }
                        "FlexWrap" => {
                            ::core::option::Option::Some(LockableStyleAttribute::FlexWrap {})
                        }
                        "FlexGrow" => {
                            ::core::option::Option::Some(LockableStyleAttribute::FlexGrow {})
                        }
                        "FlexShrink" => {
                            ::core::option::Option::Some(LockableStyleAttribute::FlexShrink {})
                        }
                        "FlexBasis" => {
                            ::core::option::Option::Some(LockableStyleAttribute::FlexBasis {})
                        }
                        "RowGap" => {
                            ::core::option::Option::Some(LockableStyleAttribute::RowGap {})
                        }
                        "ColumnGap" => {
                            ::core::option::Option::Some(LockableStyleAttribute::ColumnGap {})
                        }
                        "GridAutoFlow" => {
                            ::core::option::Option::Some(LockableStyleAttribute::GridAutoFlow {})
                        }
                        "GridTemplateRows" => {
                            ::core::option::Option::Some(LockableStyleAttribute::GridTemplateRows {})
                        }
                        "GridTemplateColumns" => {
                            ::core::option::Option::Some(LockableStyleAttribute::GridTemplateColumns {})
                        }
                        "GridAutoRows" => {
                            ::core::option::Option::Some(LockableStyleAttribute::GridAutoRows {})
                        }
                        "GridAutoColumns" => {
                            ::core::option::Option::Some(LockableStyleAttribute::GridAutoColumns {})
                        }
                        "GridRow" => {
                            ::core::option::Option::Some(LockableStyleAttribute::GridRow {})
                        }
                        "GridColumn" => {
                            ::core::option::Option::Some(LockableStyleAttribute::GridColumn {})
                        }
                        "BackgroundColor" => {
                            ::core::option::Option::Some(LockableStyleAttribute::BackgroundColor {})
                        }
                        "BorderColor" => {
                            ::core::option::Option::Some(LockableStyleAttribute::BorderColor {})
                        }
                        "FocusPolicy" => {
                            ::core::option::Option::Some(LockableStyleAttribute::FocusPolicy {})
                        }
                        "Visibility" => {
                            ::core::option::Option::Some(LockableStyleAttribute::Visibility {})
                        }
                        "ZIndex" => {
                            ::core::option::Option::Some(LockableStyleAttribute::ZIndex {})
                        }
                        "Image" => {
                            ::core::option::Option::Some(LockableStyleAttribute::Image {})
                        }
                        "ImageScaleMode" => {
                            ::core::option::Option::Some(LockableStyleAttribute::ImageScaleMode {})
                        }
                        "FluxInteraction" => {
                            ::core::option::Option::Some(LockableStyleAttribute::FluxInteraction {})
                        }
                        name => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "variant with name `{0}` does not exist on enum `{1}`",
                                    name,
                                    <Self as bevy_reflect::TypePath>::type_path(),
                                ),
                            );
                        }
                    }
                } else {
                    ::core::option::Option::None
                }
            }
        }
    };
    pub enum InteractiveStyleAttribute {
        Display(StaticBundle<Display>),
        PositionType(StaticBundle<PositionType>),
        Overflow(StaticBundle<Overflow>),
        Direction(StaticBundle<Direction>),
        Left(StaticBundle<Val>),
        Right(StaticBundle<Val>),
        Top(StaticBundle<Val>),
        Bottom(StaticBundle<Val>),
        Width(StaticBundle<Val>),
        Height(StaticBundle<Val>),
        MinWidth(StaticBundle<Val>),
        MinHeight(StaticBundle<Val>),
        MaxWidth(StaticBundle<Val>),
        MaxHeight(StaticBundle<Val>),
        AspectRatio(StaticBundle<Option<f32>>),
        AlignItems(StaticBundle<AlignItems>),
        JustifyItems(StaticBundle<JustifyItems>),
        AlignSelf(StaticBundle<AlignSelf>),
        JustifySelf(StaticBundle<JustifySelf>),
        AlignContent(StaticBundle<AlignContent>),
        JustifyContents(StaticBundle<JustifyContent>),
        Margin(StaticBundle<UiRect>),
        Padding(StaticBundle<UiRect>),
        Border(StaticBundle<UiRect>),
        FlexDirection(StaticBundle<FlexDirection>),
        FlexWrap(StaticBundle<FlexWrap>),
        FlexGrow(StaticBundle<f32>),
        FlexShrink(StaticBundle<f32>),
        FlexBasis(StaticBundle<Val>),
        RowGap(StaticBundle<Val>),
        ColumnGap(StaticBundle<Val>),
        GridAutoFlow(StaticBundle<GridAutoFlow>),
        GridTemplateRows(StaticBundle<Vec<RepeatedGridTrack>>),
        GridTemplateColumns(StaticBundle<Vec<RepeatedGridTrack>>),
        GridAutoRows(StaticBundle<Vec<GridTrack>>),
        GridAutoColumns(StaticBundle<Vec<GridTrack>>),
        GridRow(StaticBundle<GridPlacement>),
        GridColumn(StaticBundle<GridPlacement>),
        BackgroundColor(StaticBundle<Color>),
        BorderColor(StaticBundle<Color>),
        FocusPolicy(StaticBundle<FocusPolicy>),
        Visibility(StaticBundle<Visibility>),
        ZIndex(StaticBundle<ZIndex>),
        Image(StaticBundle<String>),
        ImageScaleMode(StaticBundle<Option<ImageScaleMode>>),
        AbsolutePosition(StaticBundle<Vec2>),
        Custom(fn(Entity, FluxInteraction, &mut World)),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for InteractiveStyleAttribute {
        #[inline]
        fn clone(&self) -> InteractiveStyleAttribute {
            match self {
                InteractiveStyleAttribute::Display(__self_0) => {
                    InteractiveStyleAttribute::Display(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::PositionType(__self_0) => {
                    InteractiveStyleAttribute::PositionType(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::Overflow(__self_0) => {
                    InteractiveStyleAttribute::Overflow(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::Direction(__self_0) => {
                    InteractiveStyleAttribute::Direction(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::Left(__self_0) => {
                    InteractiveStyleAttribute::Left(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::Right(__self_0) => {
                    InteractiveStyleAttribute::Right(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::Top(__self_0) => {
                    InteractiveStyleAttribute::Top(::core::clone::Clone::clone(__self_0))
                }
                InteractiveStyleAttribute::Bottom(__self_0) => {
                    InteractiveStyleAttribute::Bottom(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::Width(__self_0) => {
                    InteractiveStyleAttribute::Width(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::Height(__self_0) => {
                    InteractiveStyleAttribute::Height(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::MinWidth(__self_0) => {
                    InteractiveStyleAttribute::MinWidth(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::MinHeight(__self_0) => {
                    InteractiveStyleAttribute::MinHeight(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::MaxWidth(__self_0) => {
                    InteractiveStyleAttribute::MaxWidth(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::MaxHeight(__self_0) => {
                    InteractiveStyleAttribute::MaxHeight(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::AspectRatio(__self_0) => {
                    InteractiveStyleAttribute::AspectRatio(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::AlignItems(__self_0) => {
                    InteractiveStyleAttribute::AlignItems(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::JustifyItems(__self_0) => {
                    InteractiveStyleAttribute::JustifyItems(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::AlignSelf(__self_0) => {
                    InteractiveStyleAttribute::AlignSelf(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::JustifySelf(__self_0) => {
                    InteractiveStyleAttribute::JustifySelf(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::AlignContent(__self_0) => {
                    InteractiveStyleAttribute::AlignContent(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::JustifyContents(__self_0) => {
                    InteractiveStyleAttribute::JustifyContents(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::Margin(__self_0) => {
                    InteractiveStyleAttribute::Margin(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::Padding(__self_0) => {
                    InteractiveStyleAttribute::Padding(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::Border(__self_0) => {
                    InteractiveStyleAttribute::Border(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::FlexDirection(__self_0) => {
                    InteractiveStyleAttribute::FlexDirection(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::FlexWrap(__self_0) => {
                    InteractiveStyleAttribute::FlexWrap(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::FlexGrow(__self_0) => {
                    InteractiveStyleAttribute::FlexGrow(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::FlexShrink(__self_0) => {
                    InteractiveStyleAttribute::FlexShrink(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::FlexBasis(__self_0) => {
                    InteractiveStyleAttribute::FlexBasis(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::RowGap(__self_0) => {
                    InteractiveStyleAttribute::RowGap(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::ColumnGap(__self_0) => {
                    InteractiveStyleAttribute::ColumnGap(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::GridAutoFlow(__self_0) => {
                    InteractiveStyleAttribute::GridAutoFlow(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::GridTemplateRows(__self_0) => {
                    InteractiveStyleAttribute::GridTemplateRows(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::GridTemplateColumns(__self_0) => {
                    InteractiveStyleAttribute::GridTemplateColumns(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::GridAutoRows(__self_0) => {
                    InteractiveStyleAttribute::GridAutoRows(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::GridAutoColumns(__self_0) => {
                    InteractiveStyleAttribute::GridAutoColumns(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::GridRow(__self_0) => {
                    InteractiveStyleAttribute::GridRow(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::GridColumn(__self_0) => {
                    InteractiveStyleAttribute::GridColumn(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::BackgroundColor(__self_0) => {
                    InteractiveStyleAttribute::BackgroundColor(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::BorderColor(__self_0) => {
                    InteractiveStyleAttribute::BorderColor(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::FocusPolicy(__self_0) => {
                    InteractiveStyleAttribute::FocusPolicy(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::Visibility(__self_0) => {
                    InteractiveStyleAttribute::Visibility(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::ZIndex(__self_0) => {
                    InteractiveStyleAttribute::ZIndex(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::Image(__self_0) => {
                    InteractiveStyleAttribute::Image(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::ImageScaleMode(__self_0) => {
                    InteractiveStyleAttribute::ImageScaleMode(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::AbsolutePosition(__self_0) => {
                    InteractiveStyleAttribute::AbsolutePosition(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InteractiveStyleAttribute::Custom(__self_0) => {
                    InteractiveStyleAttribute::Custom(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for InteractiveStyleAttribute {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                InteractiveStyleAttribute::Display(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Display",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::PositionType(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "PositionType",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::Overflow(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Overflow",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::Direction(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Direction",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::Left(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Left",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::Right(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Right",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::Top(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Top",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::Bottom(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Bottom",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::Width(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Width",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::Height(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Height",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::MinWidth(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "MinWidth",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::MinHeight(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "MinHeight",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::MaxWidth(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "MaxWidth",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::MaxHeight(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "MaxHeight",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::AspectRatio(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AspectRatio",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::AlignItems(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AlignItems",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::JustifyItems(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "JustifyItems",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::AlignSelf(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AlignSelf",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::JustifySelf(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "JustifySelf",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::AlignContent(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AlignContent",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::JustifyContents(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "JustifyContents",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::Margin(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Margin",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::Padding(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Padding",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::Border(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Border",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::FlexDirection(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FlexDirection",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::FlexWrap(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FlexWrap",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::FlexGrow(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FlexGrow",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::FlexShrink(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FlexShrink",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::FlexBasis(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FlexBasis",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::RowGap(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "RowGap",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::ColumnGap(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ColumnGap",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::GridAutoFlow(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "GridAutoFlow",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::GridTemplateRows(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "GridTemplateRows",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::GridTemplateColumns(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "GridTemplateColumns",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::GridAutoRows(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "GridAutoRows",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::GridAutoColumns(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "GridAutoColumns",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::GridRow(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "GridRow",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::GridColumn(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "GridColumn",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::BackgroundColor(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BackgroundColor",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::BorderColor(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BorderColor",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::FocusPolicy(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FocusPolicy",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::Visibility(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Visibility",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::ZIndex(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZIndex",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::Image(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Image",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::ImageScaleMode(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ImageScaleMode",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::AbsolutePosition(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AbsolutePosition",
                        &__self_0,
                    )
                }
                InteractiveStyleAttribute::Custom(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Custom",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl PartialEq for InteractiveStyleAttribute {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Display(_), Self::Display(_)) => true,
                (Self::PositionType(_), Self::PositionType(_)) => true,
                (Self::Overflow(_), Self::Overflow(_)) => true,
                (Self::Direction(_), Self::Direction(_)) => true,
                (Self::Left(_), Self::Left(_)) => true,
                (Self::Right(_), Self::Right(_)) => true,
                (Self::Top(_), Self::Top(_)) => true,
                (Self::Bottom(_), Self::Bottom(_)) => true,
                (Self::Width(_), Self::Width(_)) => true,
                (Self::Height(_), Self::Height(_)) => true,
                (Self::MinWidth(_), Self::MinWidth(_)) => true,
                (Self::MinHeight(_), Self::MinHeight(_)) => true,
                (Self::MaxWidth(_), Self::MaxWidth(_)) => true,
                (Self::MaxHeight(_), Self::MaxHeight(_)) => true,
                (Self::AspectRatio(_), Self::AspectRatio(_)) => true,
                (Self::AlignItems(_), Self::AlignItems(_)) => true,
                (Self::JustifyItems(_), Self::JustifyItems(_)) => true,
                (Self::AlignSelf(_), Self::AlignSelf(_)) => true,
                (Self::JustifySelf(_), Self::JustifySelf(_)) => true,
                (Self::AlignContent(_), Self::AlignContent(_)) => true,
                (Self::JustifyContents(_), Self::JustifyContents(_)) => true,
                (Self::Margin(_), Self::Margin(_)) => true,
                (Self::Padding(_), Self::Padding(_)) => true,
                (Self::Border(_), Self::Border(_)) => true,
                (Self::FlexDirection(_), Self::FlexDirection(_)) => true,
                (Self::FlexWrap(_), Self::FlexWrap(_)) => true,
                (Self::FlexGrow(_), Self::FlexGrow(_)) => true,
                (Self::FlexShrink(_), Self::FlexShrink(_)) => true,
                (Self::FlexBasis(_), Self::FlexBasis(_)) => true,
                (Self::RowGap(_), Self::RowGap(_)) => true,
                (Self::ColumnGap(_), Self::ColumnGap(_)) => true,
                (Self::GridAutoFlow(_), Self::GridAutoFlow(_)) => true,
                (Self::GridTemplateRows(_), Self::GridTemplateRows(_)) => true,
                (Self::GridTemplateColumns(_), Self::GridTemplateColumns(_)) => true,
                (Self::GridAutoRows(_), Self::GridAutoRows(_)) => true,
                (Self::GridAutoColumns(_), Self::GridAutoColumns(_)) => true,
                (Self::GridRow(_), Self::GridRow(_)) => true,
                (Self::GridColumn(_), Self::GridColumn(_)) => true,
                (Self::BackgroundColor(_), Self::BackgroundColor(_)) => true,
                (Self::BorderColor(_), Self::BorderColor(_)) => true,
                (Self::FocusPolicy(_), Self::FocusPolicy(_)) => true,
                (Self::Visibility(_), Self::Visibility(_)) => true,
                (Self::ZIndex(_), Self::ZIndex(_)) => true,
                (Self::Image(_), Self::Image(_)) => true,
                (Self::ImageScaleMode(_), Self::ImageScaleMode(_)) => true,
                (Self::AbsolutePosition(_), Self::AbsolutePosition(_)) => true,
                (Self::Custom(l0), Self::Custom(r0)) => l0 == r0,
                _ => false,
            }
        }
    }
    impl InteractiveStyleAttribute {
        fn to_attribute(
            &self,
            flux_interaction: FluxInteraction,
        ) -> StaticStyleAttribute {
            match self {
                Self::Display(bundle) => {
                    StaticStyleAttribute::Display(bundle.to_value(flux_interaction))
                }
                Self::PositionType(bundle) => {
                    StaticStyleAttribute::PositionType(bundle.to_value(flux_interaction))
                }
                Self::Overflow(bundle) => {
                    StaticStyleAttribute::Overflow(bundle.to_value(flux_interaction))
                }
                Self::Direction(bundle) => {
                    StaticStyleAttribute::Direction(bundle.to_value(flux_interaction))
                }
                Self::Left(bundle) => {
                    StaticStyleAttribute::Left(bundle.to_value(flux_interaction))
                }
                Self::Right(bundle) => {
                    StaticStyleAttribute::Right(bundle.to_value(flux_interaction))
                }
                Self::Top(bundle) => {
                    StaticStyleAttribute::Top(bundle.to_value(flux_interaction))
                }
                Self::Bottom(bundle) => {
                    StaticStyleAttribute::Bottom(bundle.to_value(flux_interaction))
                }
                Self::Width(bundle) => {
                    StaticStyleAttribute::Width(bundle.to_value(flux_interaction))
                }
                Self::Height(bundle) => {
                    StaticStyleAttribute::Height(bundle.to_value(flux_interaction))
                }
                Self::MinWidth(bundle) => {
                    StaticStyleAttribute::MinWidth(bundle.to_value(flux_interaction))
                }
                Self::MinHeight(bundle) => {
                    StaticStyleAttribute::MinHeight(bundle.to_value(flux_interaction))
                }
                Self::MaxWidth(bundle) => {
                    StaticStyleAttribute::MaxWidth(bundle.to_value(flux_interaction))
                }
                Self::MaxHeight(bundle) => {
                    StaticStyleAttribute::MaxHeight(bundle.to_value(flux_interaction))
                }
                Self::AspectRatio(bundle) => {
                    StaticStyleAttribute::AspectRatio(bundle.to_value(flux_interaction))
                }
                Self::AlignItems(bundle) => {
                    StaticStyleAttribute::AlignItems(bundle.to_value(flux_interaction))
                }
                Self::JustifyItems(bundle) => {
                    StaticStyleAttribute::JustifyItems(bundle.to_value(flux_interaction))
                }
                Self::AlignSelf(bundle) => {
                    StaticStyleAttribute::AlignSelf(bundle.to_value(flux_interaction))
                }
                Self::JustifySelf(bundle) => {
                    StaticStyleAttribute::JustifySelf(bundle.to_value(flux_interaction))
                }
                Self::AlignContent(bundle) => {
                    StaticStyleAttribute::AlignContent(bundle.to_value(flux_interaction))
                }
                Self::JustifyContents(bundle) => {
                    StaticStyleAttribute::JustifyContents(
                        bundle.to_value(flux_interaction),
                    )
                }
                Self::Margin(bundle) => {
                    StaticStyleAttribute::Margin(bundle.to_value(flux_interaction))
                }
                Self::Padding(bundle) => {
                    StaticStyleAttribute::Padding(bundle.to_value(flux_interaction))
                }
                Self::Border(bundle) => {
                    StaticStyleAttribute::Border(bundle.to_value(flux_interaction))
                }
                Self::FlexDirection(bundle) => {
                    StaticStyleAttribute::FlexDirection(
                        bundle.to_value(flux_interaction),
                    )
                }
                Self::FlexWrap(bundle) => {
                    StaticStyleAttribute::FlexWrap(bundle.to_value(flux_interaction))
                }
                Self::FlexGrow(bundle) => {
                    StaticStyleAttribute::FlexGrow(bundle.to_value(flux_interaction))
                }
                Self::FlexShrink(bundle) => {
                    StaticStyleAttribute::FlexShrink(bundle.to_value(flux_interaction))
                }
                Self::FlexBasis(bundle) => {
                    StaticStyleAttribute::FlexBasis(bundle.to_value(flux_interaction))
                }
                Self::RowGap(bundle) => {
                    StaticStyleAttribute::RowGap(bundle.to_value(flux_interaction))
                }
                Self::ColumnGap(bundle) => {
                    StaticStyleAttribute::ColumnGap(bundle.to_value(flux_interaction))
                }
                Self::GridAutoFlow(bundle) => {
                    StaticStyleAttribute::GridAutoFlow(bundle.to_value(flux_interaction))
                }
                Self::GridTemplateRows(bundle) => {
                    StaticStyleAttribute::GridTemplateRows(
                        bundle.to_value(flux_interaction),
                    )
                }
                Self::GridTemplateColumns(bundle) => {
                    StaticStyleAttribute::GridTemplateColumns(
                        bundle.to_value(flux_interaction),
                    )
                }
                Self::GridAutoRows(bundle) => {
                    StaticStyleAttribute::GridAutoRows(bundle.to_value(flux_interaction))
                }
                Self::GridAutoColumns(bundle) => {
                    StaticStyleAttribute::GridAutoColumns(
                        bundle.to_value(flux_interaction),
                    )
                }
                Self::GridRow(bundle) => {
                    StaticStyleAttribute::GridRow(bundle.to_value(flux_interaction))
                }
                Self::GridColumn(bundle) => {
                    StaticStyleAttribute::GridColumn(bundle.to_value(flux_interaction))
                }
                Self::BackgroundColor(bundle) => {
                    StaticStyleAttribute::BackgroundColor(
                        bundle.to_value(flux_interaction),
                    )
                }
                Self::BorderColor(bundle) => {
                    StaticStyleAttribute::BorderColor(bundle.to_value(flux_interaction))
                }
                Self::FocusPolicy(bundle) => {
                    StaticStyleAttribute::FocusPolicy(bundle.to_value(flux_interaction))
                }
                Self::Visibility(bundle) => {
                    StaticStyleAttribute::Visibility(bundle.to_value(flux_interaction))
                }
                Self::ZIndex(bundle) => {
                    StaticStyleAttribute::ZIndex(bundle.to_value(flux_interaction))
                }
                Self::Image(bundle) => {
                    StaticStyleAttribute::Image(bundle.to_value(flux_interaction))
                }
                Self::ImageScaleMode(bundle) => {
                    StaticStyleAttribute::ImageScaleMode(
                        bundle.to_value(flux_interaction),
                    )
                }
                Self::AbsolutePosition(bundle) => {
                    StaticStyleAttribute::AbsolutePosition(
                        bundle.to_value(flux_interaction),
                    )
                }
                Self::Custom(_) => {
                    ::core::panicking::panic("internal error: entered unreachable code")
                }
            }
        }
        pub fn apply<'a>(
            &self,
            flux_interaction: FluxInteraction,
            ui_style: &'a mut UiStyle<'a>,
        ) {
            match self {
                Self::Custom(callback) => {
                    ui_style
                        .entity_commands()
                        .add(CustomInteractiveStyleAttribute {
                            callback: *callback,
                            flux_interaction,
                        });
                }
                _ => {
                    self.to_attribute(flux_interaction).apply(ui_style);
                }
            }
        }
    }
    impl<'a> InteractiveStyleBuilder<'a> {
        pub fn display(
            &mut self,
            bundle: impl Into<StaticBundle<Display>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::Display(bundle.into()),
                    ),
                );
            self
        }
        pub fn position_type(
            &mut self,
            bundle: impl Into<StaticBundle<PositionType>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::PositionType(bundle.into()),
                    ),
                );
            self
        }
        pub fn overflow(
            &mut self,
            bundle: impl Into<StaticBundle<Overflow>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::Overflow(bundle.into()),
                    ),
                );
            self
        }
        pub fn direction(
            &mut self,
            bundle: impl Into<StaticBundle<Direction>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::Direction(bundle.into()),
                    ),
                );
            self
        }
        pub fn left(&mut self, bundle: impl Into<StaticBundle<Val>>) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::Left(bundle.into()),
                    ),
                );
            self
        }
        pub fn right(&mut self, bundle: impl Into<StaticBundle<Val>>) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::Right(bundle.into()),
                    ),
                );
            self
        }
        pub fn top(&mut self, bundle: impl Into<StaticBundle<Val>>) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::Top(bundle.into()),
                    ),
                );
            self
        }
        pub fn bottom(&mut self, bundle: impl Into<StaticBundle<Val>>) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::Bottom(bundle.into()),
                    ),
                );
            self
        }
        pub fn width(&mut self, bundle: impl Into<StaticBundle<Val>>) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::Width(bundle.into()),
                    ),
                );
            self
        }
        pub fn height(&mut self, bundle: impl Into<StaticBundle<Val>>) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::Height(bundle.into()),
                    ),
                );
            self
        }
        pub fn min_width(&mut self, bundle: impl Into<StaticBundle<Val>>) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::MinWidth(bundle.into()),
                    ),
                );
            self
        }
        pub fn min_height(&mut self, bundle: impl Into<StaticBundle<Val>>) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::MinHeight(bundle.into()),
                    ),
                );
            self
        }
        pub fn max_width(&mut self, bundle: impl Into<StaticBundle<Val>>) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::MaxWidth(bundle.into()),
                    ),
                );
            self
        }
        pub fn max_height(&mut self, bundle: impl Into<StaticBundle<Val>>) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::MaxHeight(bundle.into()),
                    ),
                );
            self
        }
        pub fn aspect_ratio(
            &mut self,
            bundle: impl Into<StaticBundle<Option<f32>>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::AspectRatio(bundle.into()),
                    ),
                );
            self
        }
        pub fn align_items(
            &mut self,
            bundle: impl Into<StaticBundle<AlignItems>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::AlignItems(bundle.into()),
                    ),
                );
            self
        }
        pub fn justify_items(
            &mut self,
            bundle: impl Into<StaticBundle<JustifyItems>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::JustifyItems(bundle.into()),
                    ),
                );
            self
        }
        pub fn align_self(
            &mut self,
            bundle: impl Into<StaticBundle<AlignSelf>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::AlignSelf(bundle.into()),
                    ),
                );
            self
        }
        pub fn justify_self(
            &mut self,
            bundle: impl Into<StaticBundle<JustifySelf>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::JustifySelf(bundle.into()),
                    ),
                );
            self
        }
        pub fn align_content(
            &mut self,
            bundle: impl Into<StaticBundle<AlignContent>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::AlignContent(bundle.into()),
                    ),
                );
            self
        }
        pub fn justify_content(
            &mut self,
            bundle: impl Into<StaticBundle<JustifyContent>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::JustifyContents(bundle.into()),
                    ),
                );
            self
        }
        pub fn margin(&mut self, bundle: impl Into<StaticBundle<UiRect>>) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::Margin(bundle.into()),
                    ),
                );
            self
        }
        pub fn padding(&mut self, bundle: impl Into<StaticBundle<UiRect>>) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::Padding(bundle.into()),
                    ),
                );
            self
        }
        pub fn border(&mut self, bundle: impl Into<StaticBundle<UiRect>>) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::Border(bundle.into()),
                    ),
                );
            self
        }
        pub fn flex_direction(
            &mut self,
            bundle: impl Into<StaticBundle<FlexDirection>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::FlexDirection(bundle.into()),
                    ),
                );
            self
        }
        pub fn flex_wrap(
            &mut self,
            bundle: impl Into<StaticBundle<FlexWrap>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::FlexWrap(bundle.into()),
                    ),
                );
            self
        }
        pub fn flex_grow(&mut self, bundle: impl Into<StaticBundle<f32>>) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::FlexGrow(bundle.into()),
                    ),
                );
            self
        }
        pub fn flex_shrink(
            &mut self,
            bundle: impl Into<StaticBundle<f32>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::FlexShrink(bundle.into()),
                    ),
                );
            self
        }
        pub fn flex_basis(&mut self, bundle: impl Into<StaticBundle<Val>>) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::FlexBasis(bundle.into()),
                    ),
                );
            self
        }
        pub fn row_gap(&mut self, bundle: impl Into<StaticBundle<Val>>) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::RowGap(bundle.into()),
                    ),
                );
            self
        }
        pub fn column_gap(&mut self, bundle: impl Into<StaticBundle<Val>>) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::ColumnGap(bundle.into()),
                    ),
                );
            self
        }
        pub fn grid_auto_flow(
            &mut self,
            bundle: impl Into<StaticBundle<GridAutoFlow>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::GridAutoFlow(bundle.into()),
                    ),
                );
            self
        }
        pub fn grid_template_rows(
            &mut self,
            bundle: impl Into<StaticBundle<Vec<RepeatedGridTrack>>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::GridTemplateRows(bundle.into()),
                    ),
                );
            self
        }
        pub fn grid_template_columns(
            &mut self,
            bundle: impl Into<StaticBundle<Vec<RepeatedGridTrack>>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::GridTemplateColumns(bundle.into()),
                    ),
                );
            self
        }
        pub fn grid_auto_rows(
            &mut self,
            bundle: impl Into<StaticBundle<Vec<GridTrack>>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::GridAutoRows(bundle.into()),
                    ),
                );
            self
        }
        pub fn grid_auto_columns(
            &mut self,
            bundle: impl Into<StaticBundle<Vec<GridTrack>>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::GridAutoColumns(bundle.into()),
                    ),
                );
            self
        }
        pub fn grid_row(
            &mut self,
            bundle: impl Into<StaticBundle<GridPlacement>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::GridRow(bundle.into()),
                    ),
                );
            self
        }
        pub fn grid_column(
            &mut self,
            bundle: impl Into<StaticBundle<GridPlacement>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::GridColumn(bundle.into()),
                    ),
                );
            self
        }
        pub fn background_color(
            &mut self,
            bundle: impl Into<StaticBundle<Color>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::BackgroundColor(bundle.into()),
                    ),
                );
            self
        }
        pub fn border_color(
            &mut self,
            bundle: impl Into<StaticBundle<Color>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::BorderColor(bundle.into()),
                    ),
                );
            self
        }
        pub fn focus_policy(
            &mut self,
            bundle: impl Into<StaticBundle<FocusPolicy>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::FocusPolicy(bundle.into()),
                    ),
                );
            self
        }
        pub fn visibility(
            &mut self,
            bundle: impl Into<StaticBundle<Visibility>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::Visibility(bundle.into()),
                    ),
                );
            self
        }
        pub fn z_index(&mut self, bundle: impl Into<StaticBundle<ZIndex>>) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::ZIndex(bundle.into()),
                    ),
                );
            self
        }
        pub fn image(&mut self, bundle: impl Into<StaticBundle<String>>) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::Image(bundle.into()),
                    ),
                );
            self
        }
        pub fn image_scale_mode(
            &mut self,
            bundle: impl Into<StaticBundle<Option<ImageScaleMode>>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::ImageScaleMode(bundle.into()),
                    ),
                );
            self
        }
        pub fn absolute_position(
            &mut self,
            bundle: impl Into<StaticBundle<Vec2>>,
        ) -> &mut Self {
            self.style_builder
                .add(
                    DynamicStyleAttribute::Interactive(
                        InteractiveStyleAttribute::AbsolutePosition(bundle.into()),
                    ),
                );
            self
        }
    }
    pub enum AnimatedStyleAttribute {
        Left(AnimatedBundle<Val>),
        Right(AnimatedBundle<Val>),
        Top(AnimatedBundle<Val>),
        Bottom(AnimatedBundle<Val>),
        Width(AnimatedBundle<Val>),
        Height(AnimatedBundle<Val>),
        MinWidth(AnimatedBundle<Val>),
        MinHeight(AnimatedBundle<Val>),
        MaxWidth(AnimatedBundle<Val>),
        MaxHeight(AnimatedBundle<Val>),
        Margin(AnimatedBundle<UiRect>),
        Padding(AnimatedBundle<UiRect>),
        Border(AnimatedBundle<UiRect>),
        FlexGrow(AnimatedBundle<f32>),
        FlexShrink(AnimatedBundle<f32>),
        FlexBasis(AnimatedBundle<Val>),
        RowGap(AnimatedBundle<Val>),
        ColumnGap(AnimatedBundle<Val>),
        BackgroundColor(AnimatedBundle<Color>),
        BorderColor(AnimatedBundle<Color>),
        Custom(
            fn(Entity, InteractionAnimationState, InteractionAnimationState, &mut World),
        ),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AnimatedStyleAttribute {
        #[inline]
        fn clone(&self) -> AnimatedStyleAttribute {
            match self {
                AnimatedStyleAttribute::Left(__self_0) => {
                    AnimatedStyleAttribute::Left(::core::clone::Clone::clone(__self_0))
                }
                AnimatedStyleAttribute::Right(__self_0) => {
                    AnimatedStyleAttribute::Right(::core::clone::Clone::clone(__self_0))
                }
                AnimatedStyleAttribute::Top(__self_0) => {
                    AnimatedStyleAttribute::Top(::core::clone::Clone::clone(__self_0))
                }
                AnimatedStyleAttribute::Bottom(__self_0) => {
                    AnimatedStyleAttribute::Bottom(::core::clone::Clone::clone(__self_0))
                }
                AnimatedStyleAttribute::Width(__self_0) => {
                    AnimatedStyleAttribute::Width(::core::clone::Clone::clone(__self_0))
                }
                AnimatedStyleAttribute::Height(__self_0) => {
                    AnimatedStyleAttribute::Height(::core::clone::Clone::clone(__self_0))
                }
                AnimatedStyleAttribute::MinWidth(__self_0) => {
                    AnimatedStyleAttribute::MinWidth(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                AnimatedStyleAttribute::MinHeight(__self_0) => {
                    AnimatedStyleAttribute::MinHeight(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                AnimatedStyleAttribute::MaxWidth(__self_0) => {
                    AnimatedStyleAttribute::MaxWidth(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                AnimatedStyleAttribute::MaxHeight(__self_0) => {
                    AnimatedStyleAttribute::MaxHeight(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                AnimatedStyleAttribute::Margin(__self_0) => {
                    AnimatedStyleAttribute::Margin(::core::clone::Clone::clone(__self_0))
                }
                AnimatedStyleAttribute::Padding(__self_0) => {
                    AnimatedStyleAttribute::Padding(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                AnimatedStyleAttribute::Border(__self_0) => {
                    AnimatedStyleAttribute::Border(::core::clone::Clone::clone(__self_0))
                }
                AnimatedStyleAttribute::FlexGrow(__self_0) => {
                    AnimatedStyleAttribute::FlexGrow(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                AnimatedStyleAttribute::FlexShrink(__self_0) => {
                    AnimatedStyleAttribute::FlexShrink(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                AnimatedStyleAttribute::FlexBasis(__self_0) => {
                    AnimatedStyleAttribute::FlexBasis(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                AnimatedStyleAttribute::RowGap(__self_0) => {
                    AnimatedStyleAttribute::RowGap(::core::clone::Clone::clone(__self_0))
                }
                AnimatedStyleAttribute::ColumnGap(__self_0) => {
                    AnimatedStyleAttribute::ColumnGap(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                AnimatedStyleAttribute::BackgroundColor(__self_0) => {
                    AnimatedStyleAttribute::BackgroundColor(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                AnimatedStyleAttribute::BorderColor(__self_0) => {
                    AnimatedStyleAttribute::BorderColor(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                AnimatedStyleAttribute::Custom(__self_0) => {
                    AnimatedStyleAttribute::Custom(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AnimatedStyleAttribute {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AnimatedStyleAttribute::Left(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Left",
                        &__self_0,
                    )
                }
                AnimatedStyleAttribute::Right(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Right",
                        &__self_0,
                    )
                }
                AnimatedStyleAttribute::Top(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Top",
                        &__self_0,
                    )
                }
                AnimatedStyleAttribute::Bottom(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Bottom",
                        &__self_0,
                    )
                }
                AnimatedStyleAttribute::Width(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Width",
                        &__self_0,
                    )
                }
                AnimatedStyleAttribute::Height(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Height",
                        &__self_0,
                    )
                }
                AnimatedStyleAttribute::MinWidth(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "MinWidth",
                        &__self_0,
                    )
                }
                AnimatedStyleAttribute::MinHeight(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "MinHeight",
                        &__self_0,
                    )
                }
                AnimatedStyleAttribute::MaxWidth(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "MaxWidth",
                        &__self_0,
                    )
                }
                AnimatedStyleAttribute::MaxHeight(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "MaxHeight",
                        &__self_0,
                    )
                }
                AnimatedStyleAttribute::Margin(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Margin",
                        &__self_0,
                    )
                }
                AnimatedStyleAttribute::Padding(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Padding",
                        &__self_0,
                    )
                }
                AnimatedStyleAttribute::Border(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Border",
                        &__self_0,
                    )
                }
                AnimatedStyleAttribute::FlexGrow(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FlexGrow",
                        &__self_0,
                    )
                }
                AnimatedStyleAttribute::FlexShrink(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FlexShrink",
                        &__self_0,
                    )
                }
                AnimatedStyleAttribute::FlexBasis(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FlexBasis",
                        &__self_0,
                    )
                }
                AnimatedStyleAttribute::RowGap(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "RowGap",
                        &__self_0,
                    )
                }
                AnimatedStyleAttribute::ColumnGap(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ColumnGap",
                        &__self_0,
                    )
                }
                AnimatedStyleAttribute::BackgroundColor(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BackgroundColor",
                        &__self_0,
                    )
                }
                AnimatedStyleAttribute::BorderColor(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BorderColor",
                        &__self_0,
                    )
                }
                AnimatedStyleAttribute::Custom(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Custom",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl PartialEq for AnimatedStyleAttribute {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Left(_), Self::Left(_)) => true,
                (Self::Right(_), Self::Right(_)) => true,
                (Self::Top(_), Self::Top(_)) => true,
                (Self::Bottom(_), Self::Bottom(_)) => true,
                (Self::Width(_), Self::Width(_)) => true,
                (Self::Height(_), Self::Height(_)) => true,
                (Self::MinWidth(_), Self::MinWidth(_)) => true,
                (Self::MinHeight(_), Self::MinHeight(_)) => true,
                (Self::MaxWidth(_), Self::MaxWidth(_)) => true,
                (Self::MaxHeight(_), Self::MaxHeight(_)) => true,
                (Self::Margin(_), Self::Margin(_)) => true,
                (Self::Padding(_), Self::Padding(_)) => true,
                (Self::Border(_), Self::Border(_)) => true,
                (Self::FlexGrow(_), Self::FlexGrow(_)) => true,
                (Self::FlexShrink(_), Self::FlexShrink(_)) => true,
                (Self::FlexBasis(_), Self::FlexBasis(_)) => true,
                (Self::RowGap(_), Self::RowGap(_)) => true,
                (Self::ColumnGap(_), Self::ColumnGap(_)) => true,
                (Self::BackgroundColor(_), Self::BackgroundColor(_)) => true,
                (Self::BorderColor(_), Self::BorderColor(_)) => true,
                (Self::Custom(l0), Self::Custom(r0)) => l0 == r0,
                _ => false,
            }
        }
    }
    impl AnimatedStyleAttribute {
        fn to_attribute(
            &self,
            transition_base: InteractionAnimationState,
            animation_progress: InteractionAnimationState,
        ) -> StaticStyleAttribute {
            match self {
                Self::Left(bundle) => {
                    StaticStyleAttribute::Left(
                        bundle.to_value(transition_base, animation_progress),
                    )
                }
                Self::Right(bundle) => {
                    StaticStyleAttribute::Right(
                        bundle.to_value(transition_base, animation_progress),
                    )
                }
                Self::Top(bundle) => {
                    StaticStyleAttribute::Top(
                        bundle.to_value(transition_base, animation_progress),
                    )
                }
                Self::Bottom(bundle) => {
                    StaticStyleAttribute::Bottom(
                        bundle.to_value(transition_base, animation_progress),
                    )
                }
                Self::Width(bundle) => {
                    StaticStyleAttribute::Width(
                        bundle.to_value(transition_base, animation_progress),
                    )
                }
                Self::Height(bundle) => {
                    StaticStyleAttribute::Height(
                        bundle.to_value(transition_base, animation_progress),
                    )
                }
                Self::MinWidth(bundle) => {
                    StaticStyleAttribute::MinWidth(
                        bundle.to_value(transition_base, animation_progress),
                    )
                }
                Self::MinHeight(bundle) => {
                    StaticStyleAttribute::MinHeight(
                        bundle.to_value(transition_base, animation_progress),
                    )
                }
                Self::MaxWidth(bundle) => {
                    StaticStyleAttribute::MaxWidth(
                        bundle.to_value(transition_base, animation_progress),
                    )
                }
                Self::MaxHeight(bundle) => {
                    StaticStyleAttribute::MaxHeight(
                        bundle.to_value(transition_base, animation_progress),
                    )
                }
                Self::Margin(bundle) => {
                    StaticStyleAttribute::Margin(
                        bundle.to_value(transition_base, animation_progress),
                    )
                }
                Self::Padding(bundle) => {
                    StaticStyleAttribute::Padding(
                        bundle.to_value(transition_base, animation_progress),
                    )
                }
                Self::Border(bundle) => {
                    StaticStyleAttribute::Border(
                        bundle.to_value(transition_base, animation_progress),
                    )
                }
                Self::FlexGrow(bundle) => {
                    StaticStyleAttribute::FlexGrow(
                        bundle.to_value(transition_base, animation_progress),
                    )
                }
                Self::FlexShrink(bundle) => {
                    StaticStyleAttribute::FlexShrink(
                        bundle.to_value(transition_base, animation_progress),
                    )
                }
                Self::FlexBasis(bundle) => {
                    StaticStyleAttribute::FlexBasis(
                        bundle.to_value(transition_base, animation_progress),
                    )
                }
                Self::RowGap(bundle) => {
                    StaticStyleAttribute::RowGap(
                        bundle.to_value(transition_base, animation_progress),
                    )
                }
                Self::ColumnGap(bundle) => {
                    StaticStyleAttribute::ColumnGap(
                        bundle.to_value(transition_base, animation_progress),
                    )
                }
                Self::BackgroundColor(bundle) => {
                    StaticStyleAttribute::BackgroundColor(
                        bundle.to_value(transition_base, animation_progress),
                    )
                }
                Self::BorderColor(bundle) => {
                    StaticStyleAttribute::BorderColor(
                        bundle.to_value(transition_base, animation_progress),
                    )
                }
                Self::Custom(_) => {
                    ::core::panicking::panic("internal error: entered unreachable code")
                }
            }
        }
        pub fn apply<'a>(
            &self,
            transition_base: InteractionAnimationState,
            animation_progress: InteractionAnimationState,
            ui_style: &'a mut UiStyle<'a>,
        ) {
            match self {
                Self::Custom(callback) => {
                    ui_style
                        .entity_commands()
                        .add(CustomAnimatableStyleAttribute {
                            callback: *callback,
                            transition_base,
                            animation_progress,
                        });
                }
                _ => {
                    self.to_attribute(transition_base, animation_progress)
                        .apply(ui_style);
                }
            }
        }
    }
    impl<'a> AnimatedStyleBuilder<'a> {
        pub fn left(
            &'a mut self,
            bundle: impl Into<AnimatedBundle<Val>>,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::Left(bundle.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
        pub fn right(
            &'a mut self,
            bundle: impl Into<AnimatedBundle<Val>>,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::Right(bundle.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
        pub fn top(
            &'a mut self,
            bundle: impl Into<AnimatedBundle<Val>>,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::Top(bundle.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
        pub fn bottom(
            &'a mut self,
            bundle: impl Into<AnimatedBundle<Val>>,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::Bottom(bundle.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
        pub fn width(
            &'a mut self,
            bundle: impl Into<AnimatedBundle<Val>>,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::Width(bundle.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
        pub fn height(
            &'a mut self,
            bundle: impl Into<AnimatedBundle<Val>>,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::Height(bundle.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
        pub fn min_width(
            &'a mut self,
            bundle: impl Into<AnimatedBundle<Val>>,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::MinWidth(bundle.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
        pub fn min_height(
            &'a mut self,
            bundle: impl Into<AnimatedBundle<Val>>,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::MinHeight(bundle.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
        pub fn max_width(
            &'a mut self,
            bundle: impl Into<AnimatedBundle<Val>>,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::MaxWidth(bundle.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
        pub fn max_height(
            &'a mut self,
            bundle: impl Into<AnimatedBundle<Val>>,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::MaxHeight(bundle.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
        pub fn margin(
            &'a mut self,
            bundle: impl Into<AnimatedBundle<UiRect>>,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::Margin(bundle.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
        pub fn padding(
            &'a mut self,
            bundle: impl Into<AnimatedBundle<UiRect>>,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::Padding(bundle.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
        pub fn border(
            &'a mut self,
            bundle: impl Into<AnimatedBundle<UiRect>>,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::Border(bundle.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
        pub fn flex_grow(
            &'a mut self,
            bundle: impl Into<AnimatedBundle<f32>>,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::FlexGrow(bundle.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
        pub fn flex_shrink(
            &'a mut self,
            bundle: impl Into<AnimatedBundle<f32>>,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::FlexShrink(bundle.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
        pub fn flex_basis(
            &'a mut self,
            bundle: impl Into<AnimatedBundle<Val>>,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::FlexBasis(bundle.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
        pub fn row_gap(
            &'a mut self,
            bundle: impl Into<AnimatedBundle<Val>>,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::RowGap(bundle.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
        pub fn column_gap(
            &'a mut self,
            bundle: impl Into<AnimatedBundle<Val>>,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::ColumnGap(bundle.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
        pub fn background_color(
            &'a mut self,
            bundle: impl Into<AnimatedBundle<Color>>,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::BackgroundColor(bundle.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
        pub fn border_color(
            &'a mut self,
            bundle: impl Into<AnimatedBundle<Color>>,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::BorderColor(bundle.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
    }
    impl PartialEq<StaticStyleAttribute> for InteractiveStyleAttribute {
        fn eq(&self, other: &StaticStyleAttribute) -> bool {
            match (self, other) {
                (Self::Display(_), StaticStyleAttribute::Display(_)) => true,
                (Self::PositionType(_), StaticStyleAttribute::PositionType(_)) => true,
                (Self::Overflow(_), StaticStyleAttribute::Overflow(_)) => true,
                (Self::Direction(_), StaticStyleAttribute::Direction(_)) => true,
                (Self::Left(_), StaticStyleAttribute::Left(_)) => true,
                (Self::Right(_), StaticStyleAttribute::Right(_)) => true,
                (Self::Top(_), StaticStyleAttribute::Top(_)) => true,
                (Self::Bottom(_), StaticStyleAttribute::Bottom(_)) => true,
                (Self::Width(_), StaticStyleAttribute::Width(_)) => true,
                (Self::Height(_), StaticStyleAttribute::Height(_)) => true,
                (Self::MinWidth(_), StaticStyleAttribute::MinWidth(_)) => true,
                (Self::MinHeight(_), StaticStyleAttribute::MinHeight(_)) => true,
                (Self::MaxWidth(_), StaticStyleAttribute::MaxWidth(_)) => true,
                (Self::MaxHeight(_), StaticStyleAttribute::MaxHeight(_)) => true,
                (Self::AspectRatio(_), StaticStyleAttribute::AspectRatio(_)) => true,
                (Self::AlignItems(_), StaticStyleAttribute::AlignItems(_)) => true,
                (Self::JustifyItems(_), StaticStyleAttribute::JustifyItems(_)) => true,
                (Self::AlignSelf(_), StaticStyleAttribute::AlignSelf(_)) => true,
                (Self::JustifySelf(_), StaticStyleAttribute::JustifySelf(_)) => true,
                (Self::AlignContent(_), StaticStyleAttribute::AlignContent(_)) => true,
                (Self::JustifyContents(_), StaticStyleAttribute::JustifyContents(_)) => {
                    true
                }
                (Self::Margin(_), StaticStyleAttribute::Margin(_)) => true,
                (Self::Padding(_), StaticStyleAttribute::Padding(_)) => true,
                (Self::Border(_), StaticStyleAttribute::Border(_)) => true,
                (Self::FlexDirection(_), StaticStyleAttribute::FlexDirection(_)) => true,
                (Self::FlexWrap(_), StaticStyleAttribute::FlexWrap(_)) => true,
                (Self::FlexGrow(_), StaticStyleAttribute::FlexGrow(_)) => true,
                (Self::FlexShrink(_), StaticStyleAttribute::FlexShrink(_)) => true,
                (Self::FlexBasis(_), StaticStyleAttribute::FlexBasis(_)) => true,
                (Self::RowGap(_), StaticStyleAttribute::RowGap(_)) => true,
                (Self::ColumnGap(_), StaticStyleAttribute::ColumnGap(_)) => true,
                (Self::GridAutoFlow(_), StaticStyleAttribute::GridAutoFlow(_)) => true,
                (
                    Self::GridTemplateRows(_),
                    StaticStyleAttribute::GridTemplateRows(_),
                ) => true,
                (
                    Self::GridTemplateColumns(_),
                    StaticStyleAttribute::GridTemplateColumns(_),
                ) => true,
                (Self::GridAutoRows(_), StaticStyleAttribute::GridAutoRows(_)) => true,
                (Self::GridAutoColumns(_), StaticStyleAttribute::GridAutoColumns(_)) => {
                    true
                }
                (Self::GridRow(_), StaticStyleAttribute::GridRow(_)) => true,
                (Self::GridColumn(_), StaticStyleAttribute::GridColumn(_)) => true,
                (Self::BackgroundColor(_), StaticStyleAttribute::BackgroundColor(_)) => {
                    true
                }
                (Self::BorderColor(_), StaticStyleAttribute::BorderColor(_)) => true,
                (Self::FocusPolicy(_), StaticStyleAttribute::FocusPolicy(_)) => true,
                (Self::Visibility(_), StaticStyleAttribute::Visibility(_)) => true,
                (Self::ZIndex(_), StaticStyleAttribute::ZIndex(_)) => true,
                (Self::Image(_), StaticStyleAttribute::Image(_)) => true,
                (Self::ImageScaleMode(_), StaticStyleAttribute::ImageScaleMode(_)) => {
                    true
                }
                (
                    Self::AbsolutePosition(_),
                    StaticStyleAttribute::AbsolutePosition(_),
                ) => true,
                _ => false,
            }
        }
    }
    impl PartialEq<InteractiveStyleAttribute> for StaticStyleAttribute {
        fn eq(&self, other: &InteractiveStyleAttribute) -> bool {
            match (self, other) {
                (Self::Display(_), InteractiveStyleAttribute::Display(_)) => true,
                (Self::PositionType(_), InteractiveStyleAttribute::PositionType(_)) => {
                    true
                }
                (Self::Overflow(_), InteractiveStyleAttribute::Overflow(_)) => true,
                (Self::Direction(_), InteractiveStyleAttribute::Direction(_)) => true,
                (Self::Left(_), InteractiveStyleAttribute::Left(_)) => true,
                (Self::Right(_), InteractiveStyleAttribute::Right(_)) => true,
                (Self::Top(_), InteractiveStyleAttribute::Top(_)) => true,
                (Self::Bottom(_), InteractiveStyleAttribute::Bottom(_)) => true,
                (Self::Width(_), InteractiveStyleAttribute::Width(_)) => true,
                (Self::Height(_), InteractiveStyleAttribute::Height(_)) => true,
                (Self::MinWidth(_), InteractiveStyleAttribute::MinWidth(_)) => true,
                (Self::MinHeight(_), InteractiveStyleAttribute::MinHeight(_)) => true,
                (Self::MaxWidth(_), InteractiveStyleAttribute::MaxWidth(_)) => true,
                (Self::MaxHeight(_), InteractiveStyleAttribute::MaxHeight(_)) => true,
                (Self::AspectRatio(_), InteractiveStyleAttribute::AspectRatio(_)) => true,
                (Self::AlignItems(_), InteractiveStyleAttribute::AlignItems(_)) => true,
                (Self::JustifyItems(_), InteractiveStyleAttribute::JustifyItems(_)) => {
                    true
                }
                (Self::AlignSelf(_), InteractiveStyleAttribute::AlignSelf(_)) => true,
                (Self::JustifySelf(_), InteractiveStyleAttribute::JustifySelf(_)) => true,
                (Self::AlignContent(_), InteractiveStyleAttribute::AlignContent(_)) => {
                    true
                }
                (
                    Self::JustifyContents(_),
                    InteractiveStyleAttribute::JustifyContents(_),
                ) => true,
                (Self::Margin(_), InteractiveStyleAttribute::Margin(_)) => true,
                (Self::Padding(_), InteractiveStyleAttribute::Padding(_)) => true,
                (Self::Border(_), InteractiveStyleAttribute::Border(_)) => true,
                (Self::FlexDirection(_), InteractiveStyleAttribute::FlexDirection(_)) => {
                    true
                }
                (Self::FlexWrap(_), InteractiveStyleAttribute::FlexWrap(_)) => true,
                (Self::FlexGrow(_), InteractiveStyleAttribute::FlexGrow(_)) => true,
                (Self::FlexShrink(_), InteractiveStyleAttribute::FlexShrink(_)) => true,
                (Self::FlexBasis(_), InteractiveStyleAttribute::FlexBasis(_)) => true,
                (Self::RowGap(_), InteractiveStyleAttribute::RowGap(_)) => true,
                (Self::ColumnGap(_), InteractiveStyleAttribute::ColumnGap(_)) => true,
                (Self::GridAutoFlow(_), InteractiveStyleAttribute::GridAutoFlow(_)) => {
                    true
                }
                (
                    Self::GridTemplateRows(_),
                    InteractiveStyleAttribute::GridTemplateRows(_),
                ) => true,
                (
                    Self::GridTemplateColumns(_),
                    InteractiveStyleAttribute::GridTemplateColumns(_),
                ) => true,
                (Self::GridAutoRows(_), InteractiveStyleAttribute::GridAutoRows(_)) => {
                    true
                }
                (
                    Self::GridAutoColumns(_),
                    InteractiveStyleAttribute::GridAutoColumns(_),
                ) => true,
                (Self::GridRow(_), InteractiveStyleAttribute::GridRow(_)) => true,
                (Self::GridColumn(_), InteractiveStyleAttribute::GridColumn(_)) => true,
                (
                    Self::BackgroundColor(_),
                    InteractiveStyleAttribute::BackgroundColor(_),
                ) => true,
                (Self::BorderColor(_), InteractiveStyleAttribute::BorderColor(_)) => true,
                (Self::FocusPolicy(_), InteractiveStyleAttribute::FocusPolicy(_)) => true,
                (Self::Visibility(_), InteractiveStyleAttribute::Visibility(_)) => true,
                (Self::ZIndex(_), InteractiveStyleAttribute::ZIndex(_)) => true,
                (Self::Image(_), InteractiveStyleAttribute::Image(_)) => true,
                (
                    Self::ImageScaleMode(_),
                    InteractiveStyleAttribute::ImageScaleMode(_),
                ) => true,
                (
                    Self::AbsolutePosition(_),
                    InteractiveStyleAttribute::AbsolutePosition(_),
                ) => true,
                _ => false,
            }
        }
    }
    impl PartialEq<InteractiveStyleAttribute> for AnimatedStyleAttribute {
        fn eq(&self, other: &InteractiveStyleAttribute) -> bool {
            match (self, other) {
                (Self::Left(_), InteractiveStyleAttribute::Left(_)) => true,
                (Self::Right(_), InteractiveStyleAttribute::Right(_)) => true,
                (Self::Top(_), InteractiveStyleAttribute::Top(_)) => true,
                (Self::Bottom(_), InteractiveStyleAttribute::Bottom(_)) => true,
                (Self::Width(_), InteractiveStyleAttribute::Width(_)) => true,
                (Self::Height(_), InteractiveStyleAttribute::Height(_)) => true,
                (Self::MinWidth(_), InteractiveStyleAttribute::MinWidth(_)) => true,
                (Self::MinHeight(_), InteractiveStyleAttribute::MinHeight(_)) => true,
                (Self::MaxWidth(_), InteractiveStyleAttribute::MaxWidth(_)) => true,
                (Self::MaxHeight(_), InteractiveStyleAttribute::MaxHeight(_)) => true,
                (Self::Margin(_), InteractiveStyleAttribute::Margin(_)) => true,
                (Self::Padding(_), InteractiveStyleAttribute::Padding(_)) => true,
                (Self::Border(_), InteractiveStyleAttribute::Border(_)) => true,
                (Self::FlexGrow(_), InteractiveStyleAttribute::FlexGrow(_)) => true,
                (Self::FlexShrink(_), InteractiveStyleAttribute::FlexShrink(_)) => true,
                (Self::FlexBasis(_), InteractiveStyleAttribute::FlexBasis(_)) => true,
                (Self::RowGap(_), InteractiveStyleAttribute::RowGap(_)) => true,
                (Self::ColumnGap(_), InteractiveStyleAttribute::ColumnGap(_)) => true,
                (
                    Self::BackgroundColor(_),
                    InteractiveStyleAttribute::BackgroundColor(_),
                ) => true,
                (Self::BorderColor(_), InteractiveStyleAttribute::BorderColor(_)) => true,
                _ => false,
            }
        }
    }
    impl PartialEq<AnimatedStyleAttribute> for InteractiveStyleAttribute {
        fn eq(&self, other: &AnimatedStyleAttribute) -> bool {
            match (self, other) {
                (Self::Left(_), AnimatedStyleAttribute::Left(_)) => true,
                (Self::Right(_), AnimatedStyleAttribute::Right(_)) => true,
                (Self::Top(_), AnimatedStyleAttribute::Top(_)) => true,
                (Self::Bottom(_), AnimatedStyleAttribute::Bottom(_)) => true,
                (Self::Width(_), AnimatedStyleAttribute::Width(_)) => true,
                (Self::Height(_), AnimatedStyleAttribute::Height(_)) => true,
                (Self::MinWidth(_), AnimatedStyleAttribute::MinWidth(_)) => true,
                (Self::MinHeight(_), AnimatedStyleAttribute::MinHeight(_)) => true,
                (Self::MaxWidth(_), AnimatedStyleAttribute::MaxWidth(_)) => true,
                (Self::MaxHeight(_), AnimatedStyleAttribute::MaxHeight(_)) => true,
                (Self::Margin(_), AnimatedStyleAttribute::Margin(_)) => true,
                (Self::Padding(_), AnimatedStyleAttribute::Padding(_)) => true,
                (Self::Border(_), AnimatedStyleAttribute::Border(_)) => true,
                (Self::FlexGrow(_), AnimatedStyleAttribute::FlexGrow(_)) => true,
                (Self::FlexShrink(_), AnimatedStyleAttribute::FlexShrink(_)) => true,
                (Self::FlexBasis(_), AnimatedStyleAttribute::FlexBasis(_)) => true,
                (Self::RowGap(_), AnimatedStyleAttribute::RowGap(_)) => true,
                (Self::ColumnGap(_), AnimatedStyleAttribute::ColumnGap(_)) => true,
                (
                    Self::BackgroundColor(_),
                    AnimatedStyleAttribute::BackgroundColor(_),
                ) => true,
                (Self::BorderColor(_), AnimatedStyleAttribute::BorderColor(_)) => true,
                _ => false,
            }
        }
    }
    impl PartialEq<StaticStyleAttribute> for AnimatedStyleAttribute {
        fn eq(&self, other: &StaticStyleAttribute) -> bool {
            match (self, other) {
                (Self::Left(_), StaticStyleAttribute::Left(_)) => true,
                (Self::Right(_), StaticStyleAttribute::Right(_)) => true,
                (Self::Top(_), StaticStyleAttribute::Top(_)) => true,
                (Self::Bottom(_), StaticStyleAttribute::Bottom(_)) => true,
                (Self::Width(_), StaticStyleAttribute::Width(_)) => true,
                (Self::Height(_), StaticStyleAttribute::Height(_)) => true,
                (Self::MinWidth(_), StaticStyleAttribute::MinWidth(_)) => true,
                (Self::MinHeight(_), StaticStyleAttribute::MinHeight(_)) => true,
                (Self::MaxWidth(_), StaticStyleAttribute::MaxWidth(_)) => true,
                (Self::MaxHeight(_), StaticStyleAttribute::MaxHeight(_)) => true,
                (Self::Margin(_), StaticStyleAttribute::Margin(_)) => true,
                (Self::Padding(_), StaticStyleAttribute::Padding(_)) => true,
                (Self::Border(_), StaticStyleAttribute::Border(_)) => true,
                (Self::FlexGrow(_), StaticStyleAttribute::FlexGrow(_)) => true,
                (Self::FlexShrink(_), StaticStyleAttribute::FlexShrink(_)) => true,
                (Self::FlexBasis(_), StaticStyleAttribute::FlexBasis(_)) => true,
                (Self::RowGap(_), StaticStyleAttribute::RowGap(_)) => true,
                (Self::ColumnGap(_), StaticStyleAttribute::ColumnGap(_)) => true,
                (Self::BackgroundColor(_), StaticStyleAttribute::BackgroundColor(_)) => {
                    true
                }
                (Self::BorderColor(_), StaticStyleAttribute::BorderColor(_)) => true,
                _ => false,
            }
        }
    }
    impl PartialEq<AnimatedStyleAttribute> for StaticStyleAttribute {
        fn eq(&self, other: &AnimatedStyleAttribute) -> bool {
            match (self, other) {
                (Self::Left(_), AnimatedStyleAttribute::Left(_)) => true,
                (Self::Right(_), AnimatedStyleAttribute::Right(_)) => true,
                (Self::Top(_), AnimatedStyleAttribute::Top(_)) => true,
                (Self::Bottom(_), AnimatedStyleAttribute::Bottom(_)) => true,
                (Self::Width(_), AnimatedStyleAttribute::Width(_)) => true,
                (Self::Height(_), AnimatedStyleAttribute::Height(_)) => true,
                (Self::MinWidth(_), AnimatedStyleAttribute::MinWidth(_)) => true,
                (Self::MinHeight(_), AnimatedStyleAttribute::MinHeight(_)) => true,
                (Self::MaxWidth(_), AnimatedStyleAttribute::MaxWidth(_)) => true,
                (Self::MaxHeight(_), AnimatedStyleAttribute::MaxHeight(_)) => true,
                (Self::Margin(_), AnimatedStyleAttribute::Margin(_)) => true,
                (Self::Padding(_), AnimatedStyleAttribute::Padding(_)) => true,
                (Self::Border(_), AnimatedStyleAttribute::Border(_)) => true,
                (Self::FlexGrow(_), AnimatedStyleAttribute::FlexGrow(_)) => true,
                (Self::FlexShrink(_), AnimatedStyleAttribute::FlexShrink(_)) => true,
                (Self::FlexBasis(_), AnimatedStyleAttribute::FlexBasis(_)) => true,
                (Self::RowGap(_), AnimatedStyleAttribute::RowGap(_)) => true,
                (Self::ColumnGap(_), AnimatedStyleAttribute::ColumnGap(_)) => true,
                (
                    Self::BackgroundColor(_),
                    AnimatedStyleAttribute::BackgroundColor(_),
                ) => true,
                (Self::BorderColor(_), AnimatedStyleAttribute::BorderColor(_)) => true,
                _ => false,
            }
        }
    }
    pub enum StylableAttribute {
        Display,
        PositionType,
        Overflow,
        Direction,
        Left,
        Right,
        Top,
        Bottom,
        Width,
        Height,
        MinWidth,
        MinHeight,
        MaxWidth,
        MaxHeight,
        AspectRatio,
        AlignItems,
        JustifyItems,
        AlignSelf,
        JustifySelf,
        AlignContent,
        JustifyContents,
        Margin,
        Padding,
        Border,
        FlexDirection,
        FlexWrap,
        FlexGrow,
        FlexShrink,
        FlexBasis,
        RowGap,
        ColumnGap,
        GridAutoFlow,
        GridTemplateRows,
        GridTemplateColumns,
        GridAutoRows,
        GridAutoColumns,
        GridRow,
        GridColumn,
        BackgroundColor,
        BorderColor,
        FocusPolicy,
        Visibility,
        ZIndex,
        Image,
        ImageScaleMode,
        FluxInteraction,
        AbsolutePosition,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StylableAttribute {
        #[inline]
        fn clone(&self) -> StylableAttribute {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for StylableAttribute {}
    #[automatically_derived]
    impl ::core::fmt::Debug for StylableAttribute {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    StylableAttribute::Display => "Display",
                    StylableAttribute::PositionType => "PositionType",
                    StylableAttribute::Overflow => "Overflow",
                    StylableAttribute::Direction => "Direction",
                    StylableAttribute::Left => "Left",
                    StylableAttribute::Right => "Right",
                    StylableAttribute::Top => "Top",
                    StylableAttribute::Bottom => "Bottom",
                    StylableAttribute::Width => "Width",
                    StylableAttribute::Height => "Height",
                    StylableAttribute::MinWidth => "MinWidth",
                    StylableAttribute::MinHeight => "MinHeight",
                    StylableAttribute::MaxWidth => "MaxWidth",
                    StylableAttribute::MaxHeight => "MaxHeight",
                    StylableAttribute::AspectRatio => "AspectRatio",
                    StylableAttribute::AlignItems => "AlignItems",
                    StylableAttribute::JustifyItems => "JustifyItems",
                    StylableAttribute::AlignSelf => "AlignSelf",
                    StylableAttribute::JustifySelf => "JustifySelf",
                    StylableAttribute::AlignContent => "AlignContent",
                    StylableAttribute::JustifyContents => "JustifyContents",
                    StylableAttribute::Margin => "Margin",
                    StylableAttribute::Padding => "Padding",
                    StylableAttribute::Border => "Border",
                    StylableAttribute::FlexDirection => "FlexDirection",
                    StylableAttribute::FlexWrap => "FlexWrap",
                    StylableAttribute::FlexGrow => "FlexGrow",
                    StylableAttribute::FlexShrink => "FlexShrink",
                    StylableAttribute::FlexBasis => "FlexBasis",
                    StylableAttribute::RowGap => "RowGap",
                    StylableAttribute::ColumnGap => "ColumnGap",
                    StylableAttribute::GridAutoFlow => "GridAutoFlow",
                    StylableAttribute::GridTemplateRows => "GridTemplateRows",
                    StylableAttribute::GridTemplateColumns => "GridTemplateColumns",
                    StylableAttribute::GridAutoRows => "GridAutoRows",
                    StylableAttribute::GridAutoColumns => "GridAutoColumns",
                    StylableAttribute::GridRow => "GridRow",
                    StylableAttribute::GridColumn => "GridColumn",
                    StylableAttribute::BackgroundColor => "BackgroundColor",
                    StylableAttribute::BorderColor => "BorderColor",
                    StylableAttribute::FocusPolicy => "FocusPolicy",
                    StylableAttribute::Visibility => "Visibility",
                    StylableAttribute::ZIndex => "ZIndex",
                    StylableAttribute::Image => "Image",
                    StylableAttribute::ImageScaleMode => "ImageScaleMode",
                    StylableAttribute::FluxInteraction => "FluxInteraction",
                    StylableAttribute::AbsolutePosition => "AbsolutePosition",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StylableAttribute {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StylableAttribute {
        #[inline]
        fn eq(&self, other: &StylableAttribute) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for StylableAttribute {}
    #[automatically_derived]
    impl ::core::cmp::Eq for StylableAttribute {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for StylableAttribute {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    const _: () = {
        #[allow(unused_mut)]
        impl bevy_reflect::GetTypeRegistration for StylableAttribute
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn get_type_registration() -> bevy_reflect::TypeRegistration {
                let mut registration = bevy_reflect::TypeRegistration::of::<Self>();
                registration
                    .insert::<
                        bevy_reflect::ReflectFromPtr,
                    >(bevy_reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        bevy_reflect::ReflectFromReflect,
                    >(bevy_reflect::FromType::<Self>::from_type());
                registration
            }
        }
        impl bevy_reflect::Typed for StylableAttribute
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn type_info() -> &'static bevy_reflect::TypeInfo {
                static CELL: bevy_reflect::utility::NonGenericTypeInfoCell = bevy_reflect::utility::NonGenericTypeInfoCell::new();
                CELL.get_or_set(|| {
                    let variants = [
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Display"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("PositionType"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Overflow"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Direction"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Left"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Right"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Top"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Bottom"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Width"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Height"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("MinWidth"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("MinHeight"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("MaxWidth"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("MaxHeight"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("AspectRatio"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("AlignItems"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("JustifyItems"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("AlignSelf"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("JustifySelf"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("AlignContent"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("JustifyContents"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Margin"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Padding"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Border"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("FlexDirection"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("FlexWrap"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("FlexGrow"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("FlexShrink"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("FlexBasis"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("RowGap"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("ColumnGap"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("GridAutoFlow"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("GridTemplateRows"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("GridTemplateColumns"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("GridAutoRows"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("GridAutoColumns"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("GridRow"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("GridColumn"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("BackgroundColor"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("BorderColor"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("FocusPolicy"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Visibility"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("ZIndex"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("Image"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("ImageScaleMode"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("FluxInteraction"),
                        ),
                        bevy_reflect::VariantInfo::Unit(
                            bevy_reflect::UnitVariantInfo::new("AbsolutePosition"),
                        ),
                    ];
                    let info = bevy_reflect::EnumInfo::new::<Self>(&variants);
                    bevy_reflect::TypeInfo::Enum(info)
                })
            }
        }
        impl bevy_reflect::TypePath for StylableAttribute
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn type_path() -> &'static str {
                "sickle_ui::ui_style::StylableAttribute"
            }
            fn short_type_path() -> &'static str {
                "StylableAttribute"
            }
            fn type_ident() -> Option<&'static str> {
                ::core::option::Option::Some("StylableAttribute")
            }
            fn crate_name() -> Option<&'static str> {
                ::core::option::Option::Some(
                    "sickle_ui::ui_style".split(':').next().unwrap(),
                )
            }
            fn module_path() -> Option<&'static str> {
                ::core::option::Option::Some("sickle_ui::ui_style")
            }
        }
        impl bevy_reflect::Enum for StylableAttribute
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn field(
                &self,
                __name_param: &str,
            ) -> ::core::option::Option<&dyn bevy_reflect::Reflect> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at(
                &self,
                __index_param: usize,
            ) -> ::core::option::Option<&dyn bevy_reflect::Reflect> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn field_mut(
                &mut self,
                __name_param: &str,
            ) -> ::core::option::Option<&mut dyn bevy_reflect::Reflect> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at_mut(
                &mut self,
                __index_param: usize,
            ) -> ::core::option::Option<&mut dyn bevy_reflect::Reflect> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn index_of(&self, __name_param: &str) -> ::core::option::Option<usize> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn name_at(&self, __index_param: usize) -> ::core::option::Option<&str> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn iter_fields(&self) -> bevy_reflect::VariantFieldIter {
                bevy_reflect::VariantFieldIter::new(self)
            }
            #[inline]
            fn field_len(&self) -> usize {
                match self {
                    StylableAttribute::Display { .. } => 0usize,
                    StylableAttribute::PositionType { .. } => 0usize,
                    StylableAttribute::Overflow { .. } => 0usize,
                    StylableAttribute::Direction { .. } => 0usize,
                    StylableAttribute::Left { .. } => 0usize,
                    StylableAttribute::Right { .. } => 0usize,
                    StylableAttribute::Top { .. } => 0usize,
                    StylableAttribute::Bottom { .. } => 0usize,
                    StylableAttribute::Width { .. } => 0usize,
                    StylableAttribute::Height { .. } => 0usize,
                    StylableAttribute::MinWidth { .. } => 0usize,
                    StylableAttribute::MinHeight { .. } => 0usize,
                    StylableAttribute::MaxWidth { .. } => 0usize,
                    StylableAttribute::MaxHeight { .. } => 0usize,
                    StylableAttribute::AspectRatio { .. } => 0usize,
                    StylableAttribute::AlignItems { .. } => 0usize,
                    StylableAttribute::JustifyItems { .. } => 0usize,
                    StylableAttribute::AlignSelf { .. } => 0usize,
                    StylableAttribute::JustifySelf { .. } => 0usize,
                    StylableAttribute::AlignContent { .. } => 0usize,
                    StylableAttribute::JustifyContents { .. } => 0usize,
                    StylableAttribute::Margin { .. } => 0usize,
                    StylableAttribute::Padding { .. } => 0usize,
                    StylableAttribute::Border { .. } => 0usize,
                    StylableAttribute::FlexDirection { .. } => 0usize,
                    StylableAttribute::FlexWrap { .. } => 0usize,
                    StylableAttribute::FlexGrow { .. } => 0usize,
                    StylableAttribute::FlexShrink { .. } => 0usize,
                    StylableAttribute::FlexBasis { .. } => 0usize,
                    StylableAttribute::RowGap { .. } => 0usize,
                    StylableAttribute::ColumnGap { .. } => 0usize,
                    StylableAttribute::GridAutoFlow { .. } => 0usize,
                    StylableAttribute::GridTemplateRows { .. } => 0usize,
                    StylableAttribute::GridTemplateColumns { .. } => 0usize,
                    StylableAttribute::GridAutoRows { .. } => 0usize,
                    StylableAttribute::GridAutoColumns { .. } => 0usize,
                    StylableAttribute::GridRow { .. } => 0usize,
                    StylableAttribute::GridColumn { .. } => 0usize,
                    StylableAttribute::BackgroundColor { .. } => 0usize,
                    StylableAttribute::BorderColor { .. } => 0usize,
                    StylableAttribute::FocusPolicy { .. } => 0usize,
                    StylableAttribute::Visibility { .. } => 0usize,
                    StylableAttribute::ZIndex { .. } => 0usize,
                    StylableAttribute::Image { .. } => 0usize,
                    StylableAttribute::ImageScaleMode { .. } => 0usize,
                    StylableAttribute::FluxInteraction { .. } => 0usize,
                    StylableAttribute::AbsolutePosition { .. } => 0usize,
                    _ => 0,
                }
            }
            #[inline]
            fn variant_name(&self) -> &str {
                match self {
                    StylableAttribute::Display { .. } => "Display",
                    StylableAttribute::PositionType { .. } => "PositionType",
                    StylableAttribute::Overflow { .. } => "Overflow",
                    StylableAttribute::Direction { .. } => "Direction",
                    StylableAttribute::Left { .. } => "Left",
                    StylableAttribute::Right { .. } => "Right",
                    StylableAttribute::Top { .. } => "Top",
                    StylableAttribute::Bottom { .. } => "Bottom",
                    StylableAttribute::Width { .. } => "Width",
                    StylableAttribute::Height { .. } => "Height",
                    StylableAttribute::MinWidth { .. } => "MinWidth",
                    StylableAttribute::MinHeight { .. } => "MinHeight",
                    StylableAttribute::MaxWidth { .. } => "MaxWidth",
                    StylableAttribute::MaxHeight { .. } => "MaxHeight",
                    StylableAttribute::AspectRatio { .. } => "AspectRatio",
                    StylableAttribute::AlignItems { .. } => "AlignItems",
                    StylableAttribute::JustifyItems { .. } => "JustifyItems",
                    StylableAttribute::AlignSelf { .. } => "AlignSelf",
                    StylableAttribute::JustifySelf { .. } => "JustifySelf",
                    StylableAttribute::AlignContent { .. } => "AlignContent",
                    StylableAttribute::JustifyContents { .. } => "JustifyContents",
                    StylableAttribute::Margin { .. } => "Margin",
                    StylableAttribute::Padding { .. } => "Padding",
                    StylableAttribute::Border { .. } => "Border",
                    StylableAttribute::FlexDirection { .. } => "FlexDirection",
                    StylableAttribute::FlexWrap { .. } => "FlexWrap",
                    StylableAttribute::FlexGrow { .. } => "FlexGrow",
                    StylableAttribute::FlexShrink { .. } => "FlexShrink",
                    StylableAttribute::FlexBasis { .. } => "FlexBasis",
                    StylableAttribute::RowGap { .. } => "RowGap",
                    StylableAttribute::ColumnGap { .. } => "ColumnGap",
                    StylableAttribute::GridAutoFlow { .. } => "GridAutoFlow",
                    StylableAttribute::GridTemplateRows { .. } => "GridTemplateRows",
                    StylableAttribute::GridTemplateColumns { .. } => {
                        "GridTemplateColumns"
                    }
                    StylableAttribute::GridAutoRows { .. } => "GridAutoRows",
                    StylableAttribute::GridAutoColumns { .. } => "GridAutoColumns",
                    StylableAttribute::GridRow { .. } => "GridRow",
                    StylableAttribute::GridColumn { .. } => "GridColumn",
                    StylableAttribute::BackgroundColor { .. } => "BackgroundColor",
                    StylableAttribute::BorderColor { .. } => "BorderColor",
                    StylableAttribute::FocusPolicy { .. } => "FocusPolicy",
                    StylableAttribute::Visibility { .. } => "Visibility",
                    StylableAttribute::ZIndex { .. } => "ZIndex",
                    StylableAttribute::Image { .. } => "Image",
                    StylableAttribute::ImageScaleMode { .. } => "ImageScaleMode",
                    StylableAttribute::FluxInteraction { .. } => "FluxInteraction",
                    StylableAttribute::AbsolutePosition { .. } => "AbsolutePosition",
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
            #[inline]
            fn variant_index(&self) -> usize {
                match self {
                    StylableAttribute::Display { .. } => 0usize,
                    StylableAttribute::PositionType { .. } => 1usize,
                    StylableAttribute::Overflow { .. } => 2usize,
                    StylableAttribute::Direction { .. } => 3usize,
                    StylableAttribute::Left { .. } => 4usize,
                    StylableAttribute::Right { .. } => 5usize,
                    StylableAttribute::Top { .. } => 6usize,
                    StylableAttribute::Bottom { .. } => 7usize,
                    StylableAttribute::Width { .. } => 8usize,
                    StylableAttribute::Height { .. } => 9usize,
                    StylableAttribute::MinWidth { .. } => 10usize,
                    StylableAttribute::MinHeight { .. } => 11usize,
                    StylableAttribute::MaxWidth { .. } => 12usize,
                    StylableAttribute::MaxHeight { .. } => 13usize,
                    StylableAttribute::AspectRatio { .. } => 14usize,
                    StylableAttribute::AlignItems { .. } => 15usize,
                    StylableAttribute::JustifyItems { .. } => 16usize,
                    StylableAttribute::AlignSelf { .. } => 17usize,
                    StylableAttribute::JustifySelf { .. } => 18usize,
                    StylableAttribute::AlignContent { .. } => 19usize,
                    StylableAttribute::JustifyContents { .. } => 20usize,
                    StylableAttribute::Margin { .. } => 21usize,
                    StylableAttribute::Padding { .. } => 22usize,
                    StylableAttribute::Border { .. } => 23usize,
                    StylableAttribute::FlexDirection { .. } => 24usize,
                    StylableAttribute::FlexWrap { .. } => 25usize,
                    StylableAttribute::FlexGrow { .. } => 26usize,
                    StylableAttribute::FlexShrink { .. } => 27usize,
                    StylableAttribute::FlexBasis { .. } => 28usize,
                    StylableAttribute::RowGap { .. } => 29usize,
                    StylableAttribute::ColumnGap { .. } => 30usize,
                    StylableAttribute::GridAutoFlow { .. } => 31usize,
                    StylableAttribute::GridTemplateRows { .. } => 32usize,
                    StylableAttribute::GridTemplateColumns { .. } => 33usize,
                    StylableAttribute::GridAutoRows { .. } => 34usize,
                    StylableAttribute::GridAutoColumns { .. } => 35usize,
                    StylableAttribute::GridRow { .. } => 36usize,
                    StylableAttribute::GridColumn { .. } => 37usize,
                    StylableAttribute::BackgroundColor { .. } => 38usize,
                    StylableAttribute::BorderColor { .. } => 39usize,
                    StylableAttribute::FocusPolicy { .. } => 40usize,
                    StylableAttribute::Visibility { .. } => 41usize,
                    StylableAttribute::ZIndex { .. } => 42usize,
                    StylableAttribute::Image { .. } => 43usize,
                    StylableAttribute::ImageScaleMode { .. } => 44usize,
                    StylableAttribute::FluxInteraction { .. } => 45usize,
                    StylableAttribute::AbsolutePosition { .. } => 46usize,
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
            #[inline]
            fn variant_type(&self) -> bevy_reflect::VariantType {
                match self {
                    StylableAttribute::Display { .. } => bevy_reflect::VariantType::Unit,
                    StylableAttribute::PositionType { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::Overflow { .. } => bevy_reflect::VariantType::Unit,
                    StylableAttribute::Direction { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::Left { .. } => bevy_reflect::VariantType::Unit,
                    StylableAttribute::Right { .. } => bevy_reflect::VariantType::Unit,
                    StylableAttribute::Top { .. } => bevy_reflect::VariantType::Unit,
                    StylableAttribute::Bottom { .. } => bevy_reflect::VariantType::Unit,
                    StylableAttribute::Width { .. } => bevy_reflect::VariantType::Unit,
                    StylableAttribute::Height { .. } => bevy_reflect::VariantType::Unit,
                    StylableAttribute::MinWidth { .. } => bevy_reflect::VariantType::Unit,
                    StylableAttribute::MinHeight { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::MaxWidth { .. } => bevy_reflect::VariantType::Unit,
                    StylableAttribute::MaxHeight { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::AspectRatio { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::AlignItems { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::JustifyItems { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::AlignSelf { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::JustifySelf { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::AlignContent { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::JustifyContents { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::Margin { .. } => bevy_reflect::VariantType::Unit,
                    StylableAttribute::Padding { .. } => bevy_reflect::VariantType::Unit,
                    StylableAttribute::Border { .. } => bevy_reflect::VariantType::Unit,
                    StylableAttribute::FlexDirection { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::FlexWrap { .. } => bevy_reflect::VariantType::Unit,
                    StylableAttribute::FlexGrow { .. } => bevy_reflect::VariantType::Unit,
                    StylableAttribute::FlexShrink { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::FlexBasis { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::RowGap { .. } => bevy_reflect::VariantType::Unit,
                    StylableAttribute::ColumnGap { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::GridAutoFlow { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::GridTemplateRows { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::GridTemplateColumns { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::GridAutoRows { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::GridAutoColumns { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::GridRow { .. } => bevy_reflect::VariantType::Unit,
                    StylableAttribute::GridColumn { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::BackgroundColor { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::BorderColor { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::FocusPolicy { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::Visibility { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::ZIndex { .. } => bevy_reflect::VariantType::Unit,
                    StylableAttribute::Image { .. } => bevy_reflect::VariantType::Unit,
                    StylableAttribute::ImageScaleMode { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::FluxInteraction { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    StylableAttribute::AbsolutePosition { .. } => {
                        bevy_reflect::VariantType::Unit
                    }
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
            fn clone_dynamic(&self) -> bevy_reflect::DynamicEnum {
                bevy_reflect::DynamicEnum::from_ref::<Self>(self)
            }
        }
        impl bevy_reflect::Reflect for StylableAttribute
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            #[inline]
            fn get_represented_type_info(
                &self,
            ) -> ::core::option::Option<&'static bevy_reflect::TypeInfo> {
                ::core::option::Option::Some(<Self as bevy_reflect::Typed>::type_info())
            }
            #[inline]
            fn into_any(
                self: ::std::boxed::Box<Self>,
            ) -> ::std::boxed::Box<dyn ::core::any::Any> {
                self
            }
            #[inline]
            fn as_any(&self) -> &dyn ::core::any::Any {
                self
            }
            #[inline]
            fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                self
            }
            #[inline]
            fn into_reflect(
                self: ::std::boxed::Box<Self>,
            ) -> ::std::boxed::Box<dyn bevy_reflect::Reflect> {
                self
            }
            #[inline]
            fn as_reflect(&self) -> &dyn bevy_reflect::Reflect {
                self
            }
            #[inline]
            fn as_reflect_mut(&mut self) -> &mut dyn bevy_reflect::Reflect {
                self
            }
            #[inline]
            fn clone_value(&self) -> ::std::boxed::Box<dyn bevy_reflect::Reflect> {
                ::std::boxed::Box::new(bevy_reflect::Enum::clone_dynamic(self))
            }
            #[inline]
            fn set(
                &mut self,
                __value_param: ::std::boxed::Box<dyn bevy_reflect::Reflect>,
            ) -> ::core::result::Result<
                (),
                ::std::boxed::Box<dyn bevy_reflect::Reflect>,
            > {
                *self = <dyn bevy_reflect::Reflect>::take(__value_param)?;
                ::core::result::Result::Ok(())
            }
            #[inline]
            fn apply(&mut self, __value_param: &dyn bevy_reflect::Reflect) {
                if let bevy_reflect::ReflectRef::Enum(__value_param) = bevy_reflect::Reflect::reflect_ref(
                    __value_param,
                ) {
                    if bevy_reflect::Enum::variant_name(self)
                        == bevy_reflect::Enum::variant_name(__value_param)
                    {
                        match bevy_reflect::Enum::variant_type(__value_param) {
                            bevy_reflect::VariantType::Struct => {
                                for field in bevy_reflect::Enum::iter_fields(
                                    __value_param,
                                ) {
                                    let name = field.name().unwrap();
                                    bevy_reflect::Enum::field_mut(self, name)
                                        .map(|v| v.apply(field.value()));
                                }
                            }
                            bevy_reflect::VariantType::Tuple => {
                                for (index, field) in ::core::iter::Iterator::enumerate(
                                    bevy_reflect::Enum::iter_fields(__value_param),
                                ) {
                                    bevy_reflect::Enum::field_at_mut(self, index)
                                        .map(|v| v.apply(field.value()));
                                }
                            }
                            _ => {}
                        }
                    } else {
                        match bevy_reflect::Enum::variant_name(__value_param) {
                            "Display" => *self = StylableAttribute::Display {},
                            "PositionType" => *self = StylableAttribute::PositionType {},
                            "Overflow" => *self = StylableAttribute::Overflow {},
                            "Direction" => *self = StylableAttribute::Direction {},
                            "Left" => *self = StylableAttribute::Left {},
                            "Right" => *self = StylableAttribute::Right {},
                            "Top" => *self = StylableAttribute::Top {},
                            "Bottom" => *self = StylableAttribute::Bottom {},
                            "Width" => *self = StylableAttribute::Width {},
                            "Height" => *self = StylableAttribute::Height {},
                            "MinWidth" => *self = StylableAttribute::MinWidth {},
                            "MinHeight" => *self = StylableAttribute::MinHeight {},
                            "MaxWidth" => *self = StylableAttribute::MaxWidth {},
                            "MaxHeight" => *self = StylableAttribute::MaxHeight {},
                            "AspectRatio" => *self = StylableAttribute::AspectRatio {},
                            "AlignItems" => *self = StylableAttribute::AlignItems {},
                            "JustifyItems" => *self = StylableAttribute::JustifyItems {},
                            "AlignSelf" => *self = StylableAttribute::AlignSelf {},
                            "JustifySelf" => *self = StylableAttribute::JustifySelf {},
                            "AlignContent" => *self = StylableAttribute::AlignContent {},
                            "JustifyContents" => {
                                *self = StylableAttribute::JustifyContents {
                                };
                            }
                            "Margin" => *self = StylableAttribute::Margin {},
                            "Padding" => *self = StylableAttribute::Padding {},
                            "Border" => *self = StylableAttribute::Border {},
                            "FlexDirection" => {
                                *self = StylableAttribute::FlexDirection {
                                };
                            }
                            "FlexWrap" => *self = StylableAttribute::FlexWrap {},
                            "FlexGrow" => *self = StylableAttribute::FlexGrow {},
                            "FlexShrink" => *self = StylableAttribute::FlexShrink {},
                            "FlexBasis" => *self = StylableAttribute::FlexBasis {},
                            "RowGap" => *self = StylableAttribute::RowGap {},
                            "ColumnGap" => *self = StylableAttribute::ColumnGap {},
                            "GridAutoFlow" => *self = StylableAttribute::GridAutoFlow {},
                            "GridTemplateRows" => {
                                *self = StylableAttribute::GridTemplateRows {
                                };
                            }
                            "GridTemplateColumns" => {
                                *self = StylableAttribute::GridTemplateColumns {
                                };
                            }
                            "GridAutoRows" => *self = StylableAttribute::GridAutoRows {},
                            "GridAutoColumns" => {
                                *self = StylableAttribute::GridAutoColumns {
                                };
                            }
                            "GridRow" => *self = StylableAttribute::GridRow {},
                            "GridColumn" => *self = StylableAttribute::GridColumn {},
                            "BackgroundColor" => {
                                *self = StylableAttribute::BackgroundColor {
                                };
                            }
                            "BorderColor" => *self = StylableAttribute::BorderColor {},
                            "FocusPolicy" => *self = StylableAttribute::FocusPolicy {},
                            "Visibility" => *self = StylableAttribute::Visibility {},
                            "ZIndex" => *self = StylableAttribute::ZIndex {},
                            "Image" => *self = StylableAttribute::Image {},
                            "ImageScaleMode" => {
                                *self = StylableAttribute::ImageScaleMode {
                                };
                            }
                            "FluxInteraction" => {
                                *self = StylableAttribute::FluxInteraction {
                                };
                            }
                            "AbsolutePosition" => {
                                *self = StylableAttribute::AbsolutePosition {
                                };
                            }
                            name => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "variant with name `{0}` does not exist on enum `{1}`",
                                        name,
                                        <Self as bevy_reflect::TypePath>::type_path(),
                                    ),
                                );
                            }
                        }
                    }
                } else {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "`{0}` is not an enum",
                                bevy_reflect::DynamicTypePath::reflect_type_path(
                                    __value_param,
                                ),
                            ),
                        );
                    };
                }
            }
            fn reflect_kind(&self) -> bevy_reflect::ReflectKind {
                bevy_reflect::ReflectKind::Enum
            }
            fn reflect_ref(&self) -> bevy_reflect::ReflectRef {
                bevy_reflect::ReflectRef::Enum(self)
            }
            fn reflect_mut(&mut self) -> bevy_reflect::ReflectMut {
                bevy_reflect::ReflectMut::Enum(self)
            }
            fn reflect_owned(
                self: ::std::boxed::Box<Self>,
            ) -> bevy_reflect::ReflectOwned {
                bevy_reflect::ReflectOwned::Enum(self)
            }
            fn reflect_hash(&self) -> ::core::option::Option<u64> {
                bevy_reflect::enum_hash(self)
            }
            fn reflect_partial_eq(
                &self,
                value: &dyn bevy_reflect::Reflect,
            ) -> ::core::option::Option<bool> {
                bevy_reflect::enum_partial_eq(self, value)
            }
        }
        impl bevy_reflect::FromReflect for StylableAttribute
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn from_reflect(
                __param0: &dyn bevy_reflect::Reflect,
            ) -> ::core::option::Option<Self> {
                if let bevy_reflect::ReflectRef::Enum(__param0) = bevy_reflect::Reflect::reflect_ref(
                    __param0,
                ) {
                    match bevy_reflect::Enum::variant_name(__param0) {
                        "Display" => {
                            ::core::option::Option::Some(StylableAttribute::Display {})
                        }
                        "PositionType" => {
                            ::core::option::Option::Some(StylableAttribute::PositionType {})
                        }
                        "Overflow" => {
                            ::core::option::Option::Some(StylableAttribute::Overflow {})
                        }
                        "Direction" => {
                            ::core::option::Option::Some(StylableAttribute::Direction {})
                        }
                        "Left" => {
                            ::core::option::Option::Some(StylableAttribute::Left {})
                        }
                        "Right" => {
                            ::core::option::Option::Some(StylableAttribute::Right {})
                        }
                        "Top" => ::core::option::Option::Some(StylableAttribute::Top {}),
                        "Bottom" => {
                            ::core::option::Option::Some(StylableAttribute::Bottom {})
                        }
                        "Width" => {
                            ::core::option::Option::Some(StylableAttribute::Width {})
                        }
                        "Height" => {
                            ::core::option::Option::Some(StylableAttribute::Height {})
                        }
                        "MinWidth" => {
                            ::core::option::Option::Some(StylableAttribute::MinWidth {})
                        }
                        "MinHeight" => {
                            ::core::option::Option::Some(StylableAttribute::MinHeight {})
                        }
                        "MaxWidth" => {
                            ::core::option::Option::Some(StylableAttribute::MaxWidth {})
                        }
                        "MaxHeight" => {
                            ::core::option::Option::Some(StylableAttribute::MaxHeight {})
                        }
                        "AspectRatio" => {
                            ::core::option::Option::Some(StylableAttribute::AspectRatio {})
                        }
                        "AlignItems" => {
                            ::core::option::Option::Some(StylableAttribute::AlignItems {})
                        }
                        "JustifyItems" => {
                            ::core::option::Option::Some(StylableAttribute::JustifyItems {})
                        }
                        "AlignSelf" => {
                            ::core::option::Option::Some(StylableAttribute::AlignSelf {})
                        }
                        "JustifySelf" => {
                            ::core::option::Option::Some(StylableAttribute::JustifySelf {})
                        }
                        "AlignContent" => {
                            ::core::option::Option::Some(StylableAttribute::AlignContent {})
                        }
                        "JustifyContents" => {
                            ::core::option::Option::Some(StylableAttribute::JustifyContents {})
                        }
                        "Margin" => {
                            ::core::option::Option::Some(StylableAttribute::Margin {})
                        }
                        "Padding" => {
                            ::core::option::Option::Some(StylableAttribute::Padding {})
                        }
                        "Border" => {
                            ::core::option::Option::Some(StylableAttribute::Border {})
                        }
                        "FlexDirection" => {
                            ::core::option::Option::Some(StylableAttribute::FlexDirection {})
                        }
                        "FlexWrap" => {
                            ::core::option::Option::Some(StylableAttribute::FlexWrap {})
                        }
                        "FlexGrow" => {
                            ::core::option::Option::Some(StylableAttribute::FlexGrow {})
                        }
                        "FlexShrink" => {
                            ::core::option::Option::Some(StylableAttribute::FlexShrink {})
                        }
                        "FlexBasis" => {
                            ::core::option::Option::Some(StylableAttribute::FlexBasis {})
                        }
                        "RowGap" => {
                            ::core::option::Option::Some(StylableAttribute::RowGap {})
                        }
                        "ColumnGap" => {
                            ::core::option::Option::Some(StylableAttribute::ColumnGap {})
                        }
                        "GridAutoFlow" => {
                            ::core::option::Option::Some(StylableAttribute::GridAutoFlow {})
                        }
                        "GridTemplateRows" => {
                            ::core::option::Option::Some(StylableAttribute::GridTemplateRows {})
                        }
                        "GridTemplateColumns" => {
                            ::core::option::Option::Some(StylableAttribute::GridTemplateColumns {})
                        }
                        "GridAutoRows" => {
                            ::core::option::Option::Some(StylableAttribute::GridAutoRows {})
                        }
                        "GridAutoColumns" => {
                            ::core::option::Option::Some(StylableAttribute::GridAutoColumns {})
                        }
                        "GridRow" => {
                            ::core::option::Option::Some(StylableAttribute::GridRow {})
                        }
                        "GridColumn" => {
                            ::core::option::Option::Some(StylableAttribute::GridColumn {})
                        }
                        "BackgroundColor" => {
                            ::core::option::Option::Some(StylableAttribute::BackgroundColor {})
                        }
                        "BorderColor" => {
                            ::core::option::Option::Some(StylableAttribute::BorderColor {})
                        }
                        "FocusPolicy" => {
                            ::core::option::Option::Some(StylableAttribute::FocusPolicy {})
                        }
                        "Visibility" => {
                            ::core::option::Option::Some(StylableAttribute::Visibility {})
                        }
                        "ZIndex" => {
                            ::core::option::Option::Some(StylableAttribute::ZIndex {})
                        }
                        "Image" => {
                            ::core::option::Option::Some(StylableAttribute::Image {})
                        }
                        "ImageScaleMode" => {
                            ::core::option::Option::Some(StylableAttribute::ImageScaleMode {})
                        }
                        "FluxInteraction" => {
                            ::core::option::Option::Some(StylableAttribute::FluxInteraction {})
                        }
                        "AbsolutePosition" => {
                            ::core::option::Option::Some(StylableAttribute::AbsolutePosition {})
                        }
                        name => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "variant with name `{0}` does not exist on enum `{1}`",
                                    name,
                                    <Self as bevy_reflect::TypePath>::type_path(),
                                ),
                            );
                        }
                    }
                } else {
                    ::core::option::Option::None
                }
            }
        }
    };
    struct SetDisplay {
        display: Display,
        check_lock: bool,
    }
    pub trait SetDisplayExt<'a> {
        fn display(&'a mut self, display: Display) -> &mut UiStyle<'a>;
    }
    impl<'a> SetDisplayExt<'a> for UiStyle<'a> {
        fn display(&'a mut self, display: Display) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetDisplay {
                    display,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetDisplayUncheckedExt<'a> {
        fn display(&'a mut self, display: Display) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetDisplayUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn display(&'a mut self, display: Display) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetDisplay {
                    display,
                    check_lock: false,
                });
            self
        }
    }
    struct SetPositionType {
        position_type: PositionType,
        check_lock: bool,
    }
    pub trait SetPositionTypeExt<'a> {
        fn position_type(&'a mut self, position_type: PositionType) -> &mut UiStyle<'a>;
    }
    impl<'a> SetPositionTypeExt<'a> for UiStyle<'a> {
        fn position_type(&'a mut self, position_type: PositionType) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetPositionType {
                    position_type,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetPositionTypeUncheckedExt<'a> {
        fn position_type(
            &'a mut self,
            position_type: PositionType,
        ) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetPositionTypeUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn position_type(
            &'a mut self,
            position_type: PositionType,
        ) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetPositionType {
                    position_type,
                    check_lock: false,
                });
            self
        }
    }
    struct SetOverflow {
        overflow: Overflow,
        check_lock: bool,
    }
    pub trait SetOverflowExt<'a> {
        fn overflow(&'a mut self, overflow: Overflow) -> &mut UiStyle<'a>;
    }
    impl<'a> SetOverflowExt<'a> for UiStyle<'a> {
        fn overflow(&'a mut self, overflow: Overflow) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetOverflow {
                    overflow,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetOverflowUncheckedExt<'a> {
        fn overflow(&'a mut self, overflow: Overflow) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetOverflowUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn overflow(&'a mut self, overflow: Overflow) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetOverflow {
                    overflow,
                    check_lock: false,
                });
            self
        }
    }
    struct SetDirection {
        direction: Direction,
        check_lock: bool,
    }
    pub trait SetDirectionExt<'a> {
        fn direction(&'a mut self, direction: Direction) -> &mut UiStyle<'a>;
    }
    impl<'a> SetDirectionExt<'a> for UiStyle<'a> {
        fn direction(&'a mut self, direction: Direction) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetDirection {
                    direction,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetDirectionUncheckedExt<'a> {
        fn direction(&'a mut self, direction: Direction) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetDirectionUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn direction(&'a mut self, direction: Direction) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetDirection {
                    direction,
                    check_lock: false,
                });
            self
        }
    }
    struct SetLeft {
        left: Val,
        check_lock: bool,
    }
    pub trait SetLeftExt<'a> {
        fn left(&'a mut self, left: Val) -> &mut UiStyle<'a>;
    }
    impl<'a> SetLeftExt<'a> for UiStyle<'a> {
        fn left(&'a mut self, left: Val) -> &mut UiStyle<'a> {
            self.entity_commands().add(SetLeft { left, check_lock: true });
            self
        }
    }
    pub trait SetLeftUncheckedExt<'a> {
        fn left(&'a mut self, left: Val) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetLeftUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn left(&'a mut self, left: Val) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands().add(SetLeft { left, check_lock: false });
            self
        }
    }
    struct SetRight {
        right: Val,
        check_lock: bool,
    }
    pub trait SetRightExt<'a> {
        fn right(&'a mut self, right: Val) -> &mut UiStyle<'a>;
    }
    impl<'a> SetRightExt<'a> for UiStyle<'a> {
        fn right(&'a mut self, right: Val) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetRight {
                    right,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetRightUncheckedExt<'a> {
        fn right(&'a mut self, right: Val) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetRightUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn right(&'a mut self, right: Val) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetRight {
                    right,
                    check_lock: false,
                });
            self
        }
    }
    struct SetTop {
        top: Val,
        check_lock: bool,
    }
    pub trait SetTopExt<'a> {
        fn top(&'a mut self, top: Val) -> &mut UiStyle<'a>;
    }
    impl<'a> SetTopExt<'a> for UiStyle<'a> {
        fn top(&'a mut self, top: Val) -> &mut UiStyle<'a> {
            self.entity_commands().add(SetTop { top, check_lock: true });
            self
        }
    }
    pub trait SetTopUncheckedExt<'a> {
        fn top(&'a mut self, top: Val) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetTopUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn top(&'a mut self, top: Val) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands().add(SetTop { top, check_lock: false });
            self
        }
    }
    struct SetBottom {
        bottom: Val,
        check_lock: bool,
    }
    pub trait SetBottomExt<'a> {
        fn bottom(&'a mut self, bottom: Val) -> &mut UiStyle<'a>;
    }
    impl<'a> SetBottomExt<'a> for UiStyle<'a> {
        fn bottom(&'a mut self, bottom: Val) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetBottom {
                    bottom,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetBottomUncheckedExt<'a> {
        fn bottom(&'a mut self, bottom: Val) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetBottomUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn bottom(&'a mut self, bottom: Val) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetBottom {
                    bottom,
                    check_lock: false,
                });
            self
        }
    }
    struct SetWidth {
        width: Val,
        check_lock: bool,
    }
    pub trait SetWidthExt<'a> {
        fn width(&'a mut self, width: Val) -> &mut UiStyle<'a>;
    }
    impl<'a> SetWidthExt<'a> for UiStyle<'a> {
        fn width(&'a mut self, width: Val) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetWidth {
                    width,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetWidthUncheckedExt<'a> {
        fn width(&'a mut self, width: Val) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetWidthUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn width(&'a mut self, width: Val) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetWidth {
                    width,
                    check_lock: false,
                });
            self
        }
    }
    struct SetHeight {
        height: Val,
        check_lock: bool,
    }
    pub trait SetHeightExt<'a> {
        fn height(&'a mut self, height: Val) -> &mut UiStyle<'a>;
    }
    impl<'a> SetHeightExt<'a> for UiStyle<'a> {
        fn height(&'a mut self, height: Val) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetHeight {
                    height,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetHeightUncheckedExt<'a> {
        fn height(&'a mut self, height: Val) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetHeightUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn height(&'a mut self, height: Val) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetHeight {
                    height,
                    check_lock: false,
                });
            self
        }
    }
    struct SetMinWidth {
        min_width: Val,
        check_lock: bool,
    }
    pub trait SetMinWidthExt<'a> {
        fn min_width(&'a mut self, min_width: Val) -> &mut UiStyle<'a>;
    }
    impl<'a> SetMinWidthExt<'a> for UiStyle<'a> {
        fn min_width(&'a mut self, min_width: Val) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetMinWidth {
                    min_width,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetMinWidthUncheckedExt<'a> {
        fn min_width(&'a mut self, min_width: Val) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetMinWidthUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn min_width(&'a mut self, min_width: Val) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetMinWidth {
                    min_width,
                    check_lock: false,
                });
            self
        }
    }
    struct SetMinHeight {
        min_height: Val,
        check_lock: bool,
    }
    pub trait SetMinHeightExt<'a> {
        fn min_height(&'a mut self, min_height: Val) -> &mut UiStyle<'a>;
    }
    impl<'a> SetMinHeightExt<'a> for UiStyle<'a> {
        fn min_height(&'a mut self, min_height: Val) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetMinHeight {
                    min_height,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetMinHeightUncheckedExt<'a> {
        fn min_height(&'a mut self, min_height: Val) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetMinHeightUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn min_height(&'a mut self, min_height: Val) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetMinHeight {
                    min_height,
                    check_lock: false,
                });
            self
        }
    }
    struct SetMaxWidth {
        max_width: Val,
        check_lock: bool,
    }
    pub trait SetMaxWidthExt<'a> {
        fn max_width(&'a mut self, max_width: Val) -> &mut UiStyle<'a>;
    }
    impl<'a> SetMaxWidthExt<'a> for UiStyle<'a> {
        fn max_width(&'a mut self, max_width: Val) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetMaxWidth {
                    max_width,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetMaxWidthUncheckedExt<'a> {
        fn max_width(&'a mut self, max_width: Val) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetMaxWidthUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn max_width(&'a mut self, max_width: Val) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetMaxWidth {
                    max_width,
                    check_lock: false,
                });
            self
        }
    }
    struct SetMaxHeight {
        max_height: Val,
        check_lock: bool,
    }
    pub trait SetMaxHeightExt<'a> {
        fn max_height(&'a mut self, max_height: Val) -> &mut UiStyle<'a>;
    }
    impl<'a> SetMaxHeightExt<'a> for UiStyle<'a> {
        fn max_height(&'a mut self, max_height: Val) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetMaxHeight {
                    max_height,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetMaxHeightUncheckedExt<'a> {
        fn max_height(&'a mut self, max_height: Val) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetMaxHeightUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn max_height(&'a mut self, max_height: Val) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetMaxHeight {
                    max_height,
                    check_lock: false,
                });
            self
        }
    }
    struct SetAspectRatio {
        aspect_ratio: Option<f32>,
        check_lock: bool,
    }
    pub trait SetAspectRatioExt<'a> {
        fn aspect_ratio(&'a mut self, aspect_ratio: Option<f32>) -> &mut UiStyle<'a>;
    }
    impl<'a> SetAspectRatioExt<'a> for UiStyle<'a> {
        fn aspect_ratio(&'a mut self, aspect_ratio: Option<f32>) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetAspectRatio {
                    aspect_ratio,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetAspectRatioUncheckedExt<'a> {
        fn aspect_ratio(
            &'a mut self,
            aspect_ratio: Option<f32>,
        ) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetAspectRatioUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn aspect_ratio(
            &'a mut self,
            aspect_ratio: Option<f32>,
        ) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetAspectRatio {
                    aspect_ratio,
                    check_lock: false,
                });
            self
        }
    }
    struct SetAlignItems {
        align_items: AlignItems,
        check_lock: bool,
    }
    pub trait SetAlignItemsExt<'a> {
        fn align_items(&'a mut self, align_items: AlignItems) -> &mut UiStyle<'a>;
    }
    impl<'a> SetAlignItemsExt<'a> for UiStyle<'a> {
        fn align_items(&'a mut self, align_items: AlignItems) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetAlignItems {
                    align_items,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetAlignItemsUncheckedExt<'a> {
        fn align_items(
            &'a mut self,
            align_items: AlignItems,
        ) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetAlignItemsUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn align_items(
            &'a mut self,
            align_items: AlignItems,
        ) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetAlignItems {
                    align_items,
                    check_lock: false,
                });
            self
        }
    }
    struct SetJustifyItems {
        justify_items: JustifyItems,
        check_lock: bool,
    }
    pub trait SetJustifyItemsExt<'a> {
        fn justify_items(&'a mut self, justify_items: JustifyItems) -> &mut UiStyle<'a>;
    }
    impl<'a> SetJustifyItemsExt<'a> for UiStyle<'a> {
        fn justify_items(&'a mut self, justify_items: JustifyItems) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetJustifyItems {
                    justify_items,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetJustifyItemsUncheckedExt<'a> {
        fn justify_items(
            &'a mut self,
            justify_items: JustifyItems,
        ) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetJustifyItemsUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn justify_items(
            &'a mut self,
            justify_items: JustifyItems,
        ) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetJustifyItems {
                    justify_items,
                    check_lock: false,
                });
            self
        }
    }
    struct SetAlignSelf {
        align_self: AlignSelf,
        check_lock: bool,
    }
    pub trait SetAlignSelfExt<'a> {
        fn align_self(&'a mut self, align_self: AlignSelf) -> &mut UiStyle<'a>;
    }
    impl<'a> SetAlignSelfExt<'a> for UiStyle<'a> {
        fn align_self(&'a mut self, align_self: AlignSelf) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetAlignSelf {
                    align_self,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetAlignSelfUncheckedExt<'a> {
        fn align_self(&'a mut self, align_self: AlignSelf) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetAlignSelfUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn align_self(&'a mut self, align_self: AlignSelf) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetAlignSelf {
                    align_self,
                    check_lock: false,
                });
            self
        }
    }
    struct SetJustifySelf {
        justify_self: JustifySelf,
        check_lock: bool,
    }
    pub trait SetJustifySelfExt<'a> {
        fn justify_self(&'a mut self, justify_self: JustifySelf) -> &mut UiStyle<'a>;
    }
    impl<'a> SetJustifySelfExt<'a> for UiStyle<'a> {
        fn justify_self(&'a mut self, justify_self: JustifySelf) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetJustifySelf {
                    justify_self,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetJustifySelfUncheckedExt<'a> {
        fn justify_self(
            &'a mut self,
            justify_self: JustifySelf,
        ) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetJustifySelfUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn justify_self(
            &'a mut self,
            justify_self: JustifySelf,
        ) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetJustifySelf {
                    justify_self,
                    check_lock: false,
                });
            self
        }
    }
    struct SetAlignContent {
        align_content: AlignContent,
        check_lock: bool,
    }
    pub trait SetAlignContentExt<'a> {
        fn align_content(&'a mut self, align_content: AlignContent) -> &mut UiStyle<'a>;
    }
    impl<'a> SetAlignContentExt<'a> for UiStyle<'a> {
        fn align_content(&'a mut self, align_content: AlignContent) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetAlignContent {
                    align_content,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetAlignContentUncheckedExt<'a> {
        fn align_content(
            &'a mut self,
            align_content: AlignContent,
        ) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetAlignContentUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn align_content(
            &'a mut self,
            align_content: AlignContent,
        ) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetAlignContent {
                    align_content,
                    check_lock: false,
                });
            self
        }
    }
    struct SetJustifyContents {
        justify_content: JustifyContent,
        check_lock: bool,
    }
    pub trait SetJustifyContentsExt<'a> {
        fn justify_content(
            &'a mut self,
            justify_content: JustifyContent,
        ) -> &mut UiStyle<'a>;
    }
    impl<'a> SetJustifyContentsExt<'a> for UiStyle<'a> {
        fn justify_content(
            &'a mut self,
            justify_content: JustifyContent,
        ) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetJustifyContents {
                    justify_content,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetJustifyContentsUncheckedExt<'a> {
        fn justify_content(
            &'a mut self,
            justify_content: JustifyContent,
        ) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetJustifyContentsUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn justify_content(
            &'a mut self,
            justify_content: JustifyContent,
        ) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetJustifyContents {
                    justify_content,
                    check_lock: false,
                });
            self
        }
    }
    struct SetMargin {
        margin: UiRect,
        check_lock: bool,
    }
    pub trait SetMarginExt<'a> {
        fn margin(&'a mut self, margin: UiRect) -> &mut UiStyle<'a>;
    }
    impl<'a> SetMarginExt<'a> for UiStyle<'a> {
        fn margin(&'a mut self, margin: UiRect) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetMargin {
                    margin,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetMarginUncheckedExt<'a> {
        fn margin(&'a mut self, margin: UiRect) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetMarginUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn margin(&'a mut self, margin: UiRect) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetMargin {
                    margin,
                    check_lock: false,
                });
            self
        }
    }
    struct SetPadding {
        padding: UiRect,
        check_lock: bool,
    }
    pub trait SetPaddingExt<'a> {
        fn padding(&'a mut self, padding: UiRect) -> &mut UiStyle<'a>;
    }
    impl<'a> SetPaddingExt<'a> for UiStyle<'a> {
        fn padding(&'a mut self, padding: UiRect) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetPadding {
                    padding,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetPaddingUncheckedExt<'a> {
        fn padding(&'a mut self, padding: UiRect) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetPaddingUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn padding(&'a mut self, padding: UiRect) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetPadding {
                    padding,
                    check_lock: false,
                });
            self
        }
    }
    struct SetBorder {
        border: UiRect,
        check_lock: bool,
    }
    pub trait SetBorderExt<'a> {
        fn border(&'a mut self, border: UiRect) -> &mut UiStyle<'a>;
    }
    impl<'a> SetBorderExt<'a> for UiStyle<'a> {
        fn border(&'a mut self, border: UiRect) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetBorder {
                    border,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetBorderUncheckedExt<'a> {
        fn border(&'a mut self, border: UiRect) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetBorderUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn border(&'a mut self, border: UiRect) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetBorder {
                    border,
                    check_lock: false,
                });
            self
        }
    }
    struct SetFlexDirection {
        flex_direction: FlexDirection,
        check_lock: bool,
    }
    pub trait SetFlexDirectionExt<'a> {
        fn flex_direction(
            &'a mut self,
            flex_direction: FlexDirection,
        ) -> &mut UiStyle<'a>;
    }
    impl<'a> SetFlexDirectionExt<'a> for UiStyle<'a> {
        fn flex_direction(
            &'a mut self,
            flex_direction: FlexDirection,
        ) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetFlexDirection {
                    flex_direction,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetFlexDirectionUncheckedExt<'a> {
        fn flex_direction(
            &'a mut self,
            flex_direction: FlexDirection,
        ) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetFlexDirectionUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn flex_direction(
            &'a mut self,
            flex_direction: FlexDirection,
        ) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetFlexDirection {
                    flex_direction,
                    check_lock: false,
                });
            self
        }
    }
    struct SetFlexWrap {
        flex_wrap: FlexWrap,
        check_lock: bool,
    }
    pub trait SetFlexWrapExt<'a> {
        fn flex_wrap(&'a mut self, flex_wrap: FlexWrap) -> &mut UiStyle<'a>;
    }
    impl<'a> SetFlexWrapExt<'a> for UiStyle<'a> {
        fn flex_wrap(&'a mut self, flex_wrap: FlexWrap) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetFlexWrap {
                    flex_wrap,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetFlexWrapUncheckedExt<'a> {
        fn flex_wrap(&'a mut self, flex_wrap: FlexWrap) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetFlexWrapUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn flex_wrap(&'a mut self, flex_wrap: FlexWrap) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetFlexWrap {
                    flex_wrap,
                    check_lock: false,
                });
            self
        }
    }
    struct SetFlexGrow {
        flex_grow: f32,
        check_lock: bool,
    }
    pub trait SetFlexGrowExt<'a> {
        fn flex_grow(&'a mut self, flex_grow: f32) -> &mut UiStyle<'a>;
    }
    impl<'a> SetFlexGrowExt<'a> for UiStyle<'a> {
        fn flex_grow(&'a mut self, flex_grow: f32) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetFlexGrow {
                    flex_grow,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetFlexGrowUncheckedExt<'a> {
        fn flex_grow(&'a mut self, flex_grow: f32) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetFlexGrowUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn flex_grow(&'a mut self, flex_grow: f32) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetFlexGrow {
                    flex_grow,
                    check_lock: false,
                });
            self
        }
    }
    struct SetFlexShrink {
        flex_shrink: f32,
        check_lock: bool,
    }
    pub trait SetFlexShrinkExt<'a> {
        fn flex_shrink(&'a mut self, flex_shrink: f32) -> &mut UiStyle<'a>;
    }
    impl<'a> SetFlexShrinkExt<'a> for UiStyle<'a> {
        fn flex_shrink(&'a mut self, flex_shrink: f32) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetFlexShrink {
                    flex_shrink,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetFlexShrinkUncheckedExt<'a> {
        fn flex_shrink(&'a mut self, flex_shrink: f32) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetFlexShrinkUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn flex_shrink(&'a mut self, flex_shrink: f32) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetFlexShrink {
                    flex_shrink,
                    check_lock: false,
                });
            self
        }
    }
    struct SetFlexBasis {
        flex_basis: Val,
        check_lock: bool,
    }
    pub trait SetFlexBasisExt<'a> {
        fn flex_basis(&'a mut self, flex_basis: Val) -> &mut UiStyle<'a>;
    }
    impl<'a> SetFlexBasisExt<'a> for UiStyle<'a> {
        fn flex_basis(&'a mut self, flex_basis: Val) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetFlexBasis {
                    flex_basis,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetFlexBasisUncheckedExt<'a> {
        fn flex_basis(&'a mut self, flex_basis: Val) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetFlexBasisUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn flex_basis(&'a mut self, flex_basis: Val) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetFlexBasis {
                    flex_basis,
                    check_lock: false,
                });
            self
        }
    }
    struct SetRowGap {
        row_gap: Val,
        check_lock: bool,
    }
    pub trait SetRowGapExt<'a> {
        fn row_gap(&'a mut self, row_gap: Val) -> &mut UiStyle<'a>;
    }
    impl<'a> SetRowGapExt<'a> for UiStyle<'a> {
        fn row_gap(&'a mut self, row_gap: Val) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetRowGap {
                    row_gap,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetRowGapUncheckedExt<'a> {
        fn row_gap(&'a mut self, row_gap: Val) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetRowGapUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn row_gap(&'a mut self, row_gap: Val) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetRowGap {
                    row_gap,
                    check_lock: false,
                });
            self
        }
    }
    struct SetColumnGap {
        column_gap: Val,
        check_lock: bool,
    }
    pub trait SetColumnGapExt<'a> {
        fn column_gap(&'a mut self, column_gap: Val) -> &mut UiStyle<'a>;
    }
    impl<'a> SetColumnGapExt<'a> for UiStyle<'a> {
        fn column_gap(&'a mut self, column_gap: Val) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetColumnGap {
                    column_gap,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetColumnGapUncheckedExt<'a> {
        fn column_gap(&'a mut self, column_gap: Val) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetColumnGapUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn column_gap(&'a mut self, column_gap: Val) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetColumnGap {
                    column_gap,
                    check_lock: false,
                });
            self
        }
    }
    struct SetGridAutoFlow {
        grid_auto_flow: GridAutoFlow,
        check_lock: bool,
    }
    pub trait SetGridAutoFlowExt<'a> {
        fn grid_auto_flow(
            &'a mut self,
            grid_auto_flow: GridAutoFlow,
        ) -> &mut UiStyle<'a>;
    }
    impl<'a> SetGridAutoFlowExt<'a> for UiStyle<'a> {
        fn grid_auto_flow(
            &'a mut self,
            grid_auto_flow: GridAutoFlow,
        ) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetGridAutoFlow {
                    grid_auto_flow,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetGridAutoFlowUncheckedExt<'a> {
        fn grid_auto_flow(
            &'a mut self,
            grid_auto_flow: GridAutoFlow,
        ) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetGridAutoFlowUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn grid_auto_flow(
            &'a mut self,
            grid_auto_flow: GridAutoFlow,
        ) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetGridAutoFlow {
                    grid_auto_flow,
                    check_lock: false,
                });
            self
        }
    }
    struct SetGridTemplateRows {
        grid_template_rows: Vec<RepeatedGridTrack>,
        check_lock: bool,
    }
    pub trait SetGridTemplateRowsExt<'a> {
        fn grid_template_rows(
            &'a mut self,
            grid_template_rows: Vec<RepeatedGridTrack>,
        ) -> &mut UiStyle<'a>;
    }
    impl<'a> SetGridTemplateRowsExt<'a> for UiStyle<'a> {
        fn grid_template_rows(
            &'a mut self,
            grid_template_rows: Vec<RepeatedGridTrack>,
        ) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetGridTemplateRows {
                    grid_template_rows,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetGridTemplateRowsUncheckedExt<'a> {
        fn grid_template_rows(
            &'a mut self,
            grid_template_rows: Vec<RepeatedGridTrack>,
        ) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetGridTemplateRowsUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn grid_template_rows(
            &'a mut self,
            grid_template_rows: Vec<RepeatedGridTrack>,
        ) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetGridTemplateRows {
                    grid_template_rows,
                    check_lock: false,
                });
            self
        }
    }
    struct SetGridTemplateColumns {
        grid_template_columns: Vec<RepeatedGridTrack>,
        check_lock: bool,
    }
    pub trait SetGridTemplateColumnsExt<'a> {
        fn grid_template_columns(
            &'a mut self,
            grid_template_columns: Vec<RepeatedGridTrack>,
        ) -> &mut UiStyle<'a>;
    }
    impl<'a> SetGridTemplateColumnsExt<'a> for UiStyle<'a> {
        fn grid_template_columns(
            &'a mut self,
            grid_template_columns: Vec<RepeatedGridTrack>,
        ) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetGridTemplateColumns {
                    grid_template_columns,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetGridTemplateColumnsUncheckedExt<'a> {
        fn grid_template_columns(
            &'a mut self,
            grid_template_columns: Vec<RepeatedGridTrack>,
        ) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetGridTemplateColumnsUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn grid_template_columns(
            &'a mut self,
            grid_template_columns: Vec<RepeatedGridTrack>,
        ) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetGridTemplateColumns {
                    grid_template_columns,
                    check_lock: false,
                });
            self
        }
    }
    struct SetGridAutoRows {
        grid_auto_rows: Vec<GridTrack>,
        check_lock: bool,
    }
    pub trait SetGridAutoRowsExt<'a> {
        fn grid_auto_rows(
            &'a mut self,
            grid_auto_rows: Vec<GridTrack>,
        ) -> &mut UiStyle<'a>;
    }
    impl<'a> SetGridAutoRowsExt<'a> for UiStyle<'a> {
        fn grid_auto_rows(
            &'a mut self,
            grid_auto_rows: Vec<GridTrack>,
        ) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetGridAutoRows {
                    grid_auto_rows,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetGridAutoRowsUncheckedExt<'a> {
        fn grid_auto_rows(
            &'a mut self,
            grid_auto_rows: Vec<GridTrack>,
        ) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetGridAutoRowsUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn grid_auto_rows(
            &'a mut self,
            grid_auto_rows: Vec<GridTrack>,
        ) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetGridAutoRows {
                    grid_auto_rows,
                    check_lock: false,
                });
            self
        }
    }
    struct SetGridAutoColumns {
        grid_auto_columns: Vec<GridTrack>,
        check_lock: bool,
    }
    pub trait SetGridAutoColumnsExt<'a> {
        fn grid_auto_columns(
            &'a mut self,
            grid_auto_columns: Vec<GridTrack>,
        ) -> &mut UiStyle<'a>;
    }
    impl<'a> SetGridAutoColumnsExt<'a> for UiStyle<'a> {
        fn grid_auto_columns(
            &'a mut self,
            grid_auto_columns: Vec<GridTrack>,
        ) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetGridAutoColumns {
                    grid_auto_columns,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetGridAutoColumnsUncheckedExt<'a> {
        fn grid_auto_columns(
            &'a mut self,
            grid_auto_columns: Vec<GridTrack>,
        ) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetGridAutoColumnsUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn grid_auto_columns(
            &'a mut self,
            grid_auto_columns: Vec<GridTrack>,
        ) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetGridAutoColumns {
                    grid_auto_columns,
                    check_lock: false,
                });
            self
        }
    }
    struct SetGridRow {
        grid_row: GridPlacement,
        check_lock: bool,
    }
    pub trait SetGridRowExt<'a> {
        fn grid_row(&'a mut self, grid_row: GridPlacement) -> &mut UiStyle<'a>;
    }
    impl<'a> SetGridRowExt<'a> for UiStyle<'a> {
        fn grid_row(&'a mut self, grid_row: GridPlacement) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetGridRow {
                    grid_row,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetGridRowUncheckedExt<'a> {
        fn grid_row(&'a mut self, grid_row: GridPlacement) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetGridRowUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn grid_row(&'a mut self, grid_row: GridPlacement) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetGridRow {
                    grid_row,
                    check_lock: false,
                });
            self
        }
    }
    struct SetGridColumn {
        grid_column: GridPlacement,
        check_lock: bool,
    }
    pub trait SetGridColumnExt<'a> {
        fn grid_column(&'a mut self, grid_column: GridPlacement) -> &mut UiStyle<'a>;
    }
    impl<'a> SetGridColumnExt<'a> for UiStyle<'a> {
        fn grid_column(&'a mut self, grid_column: GridPlacement) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetGridColumn {
                    grid_column,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetGridColumnUncheckedExt<'a> {
        fn grid_column(
            &'a mut self,
            grid_column: GridPlacement,
        ) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetGridColumnUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn grid_column(
            &'a mut self,
            grid_column: GridPlacement,
        ) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetGridColumn {
                    grid_column,
                    check_lock: false,
                });
            self
        }
    }
    struct SetBackgroundColor {
        background_color: Color,
        check_lock: bool,
    }
    pub trait SetBackgroundColorExt<'a> {
        fn background_color(&'a mut self, background_color: Color) -> &mut UiStyle<'a>;
    }
    impl<'a> SetBackgroundColorExt<'a> for UiStyle<'a> {
        fn background_color(&'a mut self, background_color: Color) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetBackgroundColor {
                    background_color,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetBackgroundColorUncheckedExt<'a> {
        fn background_color(
            &'a mut self,
            background_color: Color,
        ) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetBackgroundColorUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn background_color(
            &'a mut self,
            background_color: Color,
        ) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetBackgroundColor {
                    background_color,
                    check_lock: false,
                });
            self
        }
    }
    struct SetBorderColor {
        border_color: Color,
        check_lock: bool,
    }
    pub trait SetBorderColorExt<'a> {
        fn border_color(&'a mut self, border_color: Color) -> &mut UiStyle<'a>;
    }
    impl<'a> SetBorderColorExt<'a> for UiStyle<'a> {
        fn border_color(&'a mut self, border_color: Color) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetBorderColor {
                    border_color,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetBorderColorUncheckedExt<'a> {
        fn border_color(&'a mut self, border_color: Color) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetBorderColorUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn border_color(&'a mut self, border_color: Color) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetBorderColor {
                    border_color,
                    check_lock: false,
                });
            self
        }
    }
    struct SetFocusPolicy {
        focus_policy: FocusPolicy,
        check_lock: bool,
    }
    pub trait SetFocusPolicyExt<'a> {
        fn focus_policy(&'a mut self, focus_policy: FocusPolicy) -> &mut UiStyle<'a>;
    }
    impl<'a> SetFocusPolicyExt<'a> for UiStyle<'a> {
        fn focus_policy(&'a mut self, focus_policy: FocusPolicy) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetFocusPolicy {
                    focus_policy,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetFocusPolicyUncheckedExt<'a> {
        fn focus_policy(
            &'a mut self,
            focus_policy: FocusPolicy,
        ) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetFocusPolicyUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn focus_policy(
            &'a mut self,
            focus_policy: FocusPolicy,
        ) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetFocusPolicy {
                    focus_policy,
                    check_lock: false,
                });
            self
        }
    }
    struct SetVisibility {
        visibility: Visibility,
        check_lock: bool,
    }
    pub trait SetVisibilityExt<'a> {
        fn visibility(&'a mut self, visibility: Visibility) -> &mut UiStyle<'a>;
    }
    impl<'a> SetVisibilityExt<'a> for UiStyle<'a> {
        fn visibility(&'a mut self, visibility: Visibility) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetVisibility {
                    visibility,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetVisibilityUncheckedExt<'a> {
        fn visibility(&'a mut self, visibility: Visibility) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetVisibilityUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn visibility(
            &'a mut self,
            visibility: Visibility,
        ) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetVisibility {
                    visibility,
                    check_lock: false,
                });
            self
        }
    }
    struct SetZIndex {
        z_index: ZIndex,
        check_lock: bool,
    }
    pub trait SetZIndexExt<'a> {
        fn z_index(&'a mut self, z_index: ZIndex) -> &mut UiStyle<'a>;
    }
    impl<'a> SetZIndexExt<'a> for UiStyle<'a> {
        fn z_index(&'a mut self, z_index: ZIndex) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetZIndex {
                    z_index,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetZIndexUncheckedExt<'a> {
        fn z_index(&'a mut self, z_index: ZIndex) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetZIndexUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn z_index(&'a mut self, z_index: ZIndex) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetZIndex {
                    z_index,
                    check_lock: false,
                });
            self
        }
    }
    struct SetImageScaleMode {
        image_scale_mode: Option<ImageScaleMode>,
        check_lock: bool,
    }
    pub trait SetImageScaleModeExt<'a> {
        fn image_scale_mode(
            &'a mut self,
            image_scale_mode: Option<ImageScaleMode>,
        ) -> &mut UiStyle<'a>;
    }
    impl<'a> SetImageScaleModeExt<'a> for UiStyle<'a> {
        fn image_scale_mode(
            &'a mut self,
            image_scale_mode: Option<ImageScaleMode>,
        ) -> &mut UiStyle<'a> {
            self.entity_commands()
                .add(SetImageScaleMode {
                    image_scale_mode,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetImageScaleModeUncheckedExt<'a> {
        fn image_scale_mode(
            &'a mut self,
            image_scale_mode: Option<ImageScaleMode>,
        ) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetImageScaleModeUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn image_scale_mode(
            &'a mut self,
            image_scale_mode: Option<ImageScaleMode>,
        ) -> &mut UiStyleUnchecked<'a> {
            self.entity_commands()
                .add(SetImageScaleMode {
                    image_scale_mode,
                    check_lock: false,
                });
            self
        }
    }
    impl EntityCommand for SetDisplay {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::Display) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "display",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "display",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.display != self.display {
                style.display = self.display;
            }
        }
    }
    impl EntityCommand for SetPositionType {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::PositionType) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "position_type",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "position_type",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.position_type != self.position_type {
                style.position_type = self.position_type;
            }
        }
    }
    impl EntityCommand for SetOverflow {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::Overflow) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "overflow",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "overflow",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.overflow != self.overflow {
                style.overflow = self.overflow;
            }
        }
    }
    impl EntityCommand for SetDirection {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::Direction) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "direction",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "direction",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.direction != self.direction {
                style.direction = self.direction;
            }
        }
    }
    impl EntityCommand for SetLeft {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::Left) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "left",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "left",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.left != self.left {
                style.left = self.left;
            }
        }
    }
    impl EntityCommand for SetRight {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::Right) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "right",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "right",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.right != self.right {
                style.right = self.right;
            }
        }
    }
    impl EntityCommand for SetTop {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::Top) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "top",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "top",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.top != self.top {
                style.top = self.top;
            }
        }
    }
    impl EntityCommand for SetBottom {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::Bottom) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "bottom",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "bottom",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.bottom != self.bottom {
                style.bottom = self.bottom;
            }
        }
    }
    impl EntityCommand for SetWidth {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::Width) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "width",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "width",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.width != self.width {
                style.width = self.width;
            }
        }
    }
    impl EntityCommand for SetHeight {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::Height) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "height",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "height",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.height != self.height {
                style.height = self.height;
            }
        }
    }
    impl EntityCommand for SetMinWidth {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::MinWidth) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "min_width",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "min_width",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.min_width != self.min_width {
                style.min_width = self.min_width;
            }
        }
    }
    impl EntityCommand for SetMinHeight {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::MinHeight) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "min_height",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "min_height",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.min_height != self.min_height {
                style.min_height = self.min_height;
            }
        }
    }
    impl EntityCommand for SetMaxWidth {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::MaxWidth) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "max_width",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "max_width",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.max_width != self.max_width {
                style.max_width = self.max_width;
            }
        }
    }
    impl EntityCommand for SetMaxHeight {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::MaxHeight) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "max_height",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "max_height",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.max_height != self.max_height {
                style.max_height = self.max_height;
            }
        }
    }
    impl EntityCommand for SetAspectRatio {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::AspectRatio) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "aspect_ratio",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "aspect_ratio",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.aspect_ratio != self.aspect_ratio {
                style.aspect_ratio = self.aspect_ratio;
            }
        }
    }
    impl EntityCommand for SetAlignItems {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::AlignItems) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "align_items",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "align_items",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.align_items != self.align_items {
                style.align_items = self.align_items;
            }
        }
    }
    impl EntityCommand for SetJustifyItems {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::JustifyItems) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "justify_items",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "justify_items",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.justify_items != self.justify_items {
                style.justify_items = self.justify_items;
            }
        }
    }
    impl EntityCommand for SetAlignSelf {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::AlignSelf) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "align_self",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "align_self",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.align_self != self.align_self {
                style.align_self = self.align_self;
            }
        }
    }
    impl EntityCommand for SetJustifySelf {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::JustifySelf) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "justify_self",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "justify_self",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.justify_self != self.justify_self {
                style.justify_self = self.justify_self;
            }
        }
    }
    impl EntityCommand for SetAlignContent {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::AlignContent) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "align_content",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "align_content",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.align_content != self.align_content {
                style.align_content = self.align_content;
            }
        }
    }
    impl EntityCommand for SetJustifyContents {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::JustifyContents) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "justify_content",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "justify_content",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.justify_content != self.justify_content {
                style.justify_content = self.justify_content;
            }
        }
    }
    impl EntityCommand for SetMargin {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::Margin) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "margin",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "margin",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.margin != self.margin {
                style.margin = self.margin;
            }
        }
    }
    impl EntityCommand for SetPadding {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::Padding) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "padding",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "padding",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.padding != self.padding {
                style.padding = self.padding;
            }
        }
    }
    impl EntityCommand for SetBorder {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::Border) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "border",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "border",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.border != self.border {
                style.border = self.border;
            }
        }
    }
    impl EntityCommand for SetFlexDirection {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::FlexDirection) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "flex_direction",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "flex_direction",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.flex_direction != self.flex_direction {
                style.flex_direction = self.flex_direction;
            }
        }
    }
    impl EntityCommand for SetFlexWrap {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::FlexWrap) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "flex_wrap",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "flex_wrap",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.flex_wrap != self.flex_wrap {
                style.flex_wrap = self.flex_wrap;
            }
        }
    }
    impl EntityCommand for SetFlexGrow {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::FlexGrow) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "flex_grow",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "flex_grow",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.flex_grow != self.flex_grow {
                style.flex_grow = self.flex_grow;
            }
        }
    }
    impl EntityCommand for SetFlexShrink {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::FlexShrink) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "flex_shrink",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "flex_shrink",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.flex_shrink != self.flex_shrink {
                style.flex_shrink = self.flex_shrink;
            }
        }
    }
    impl EntityCommand for SetFlexBasis {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::FlexBasis) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "flex_basis",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "flex_basis",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.flex_basis != self.flex_basis {
                style.flex_basis = self.flex_basis;
            }
        }
    }
    impl EntityCommand for SetRowGap {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::RowGap) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "row_gap",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "row_gap",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.row_gap != self.row_gap {
                style.row_gap = self.row_gap;
            }
        }
    }
    impl EntityCommand for SetColumnGap {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::ColumnGap) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "column_gap",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "column_gap",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.column_gap != self.column_gap {
                style.column_gap = self.column_gap;
            }
        }
    }
    impl EntityCommand for SetGridAutoFlow {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::GridAutoFlow) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "grid_auto_flow",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "grid_auto_flow",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.grid_auto_flow != self.grid_auto_flow {
                style.grid_auto_flow = self.grid_auto_flow;
            }
        }
    }
    impl EntityCommand for SetGridTemplateRows {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::GridTemplateRows) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "grid_template_rows",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "grid_template_rows",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.grid_template_rows != self.grid_template_rows {
                style.grid_template_rows = self.grid_template_rows;
            }
        }
    }
    impl EntityCommand for SetGridTemplateColumns {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::GridTemplateColumns)
                    {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "grid_template_columns",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "grid_template_columns",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.grid_template_columns != self.grid_template_columns {
                style.grid_template_columns = self.grid_template_columns;
            }
        }
    }
    impl EntityCommand for SetGridAutoRows {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::GridAutoRows) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "grid_auto_rows",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "grid_auto_rows",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.grid_auto_rows != self.grid_auto_rows {
                style.grid_auto_rows = self.grid_auto_rows;
            }
        }
    }
    impl EntityCommand for SetGridAutoColumns {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::GridAutoColumns) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "grid_auto_columns",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "grid_auto_columns",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.grid_auto_columns != self.grid_auto_columns {
                style.grid_auto_columns = self.grid_auto_columns;
            }
        }
    }
    impl EntityCommand for SetGridRow {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::GridRow) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "grid_row",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "grid_row",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.grid_row != self.grid_row {
                style.grid_row = self.grid_row;
            }
        }
    }
    impl EntityCommand for SetGridColumn {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::GridColumn) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "grid_column",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No Style component found!",
                                                    "grid_column",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if style.grid_column != self.grid_column {
                style.grid_column = self.grid_column;
            }
        }
    }
    impl EntityCommand for SetBackgroundColor {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::BackgroundColor) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "background_color",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut background_color) = world.get_mut::<BackgroundColor>(entity)
            else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No {2} component found!",
                                                    "background_color",
                                                    entity,
                                                    "BackgroundColor",
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if background_color.0 != self.background_color {
                background_color.0 = self.background_color;
            }
        }
    }
    impl EntityCommand for SetBorderColor {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::BorderColor) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "border_color",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut border_color) = world.get_mut::<BorderColor>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No {2} component found!",
                                                    "border_color",
                                                    entity,
                                                    "BorderColor",
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if border_color.0 != self.border_color {
                border_color.0 = self.border_color;
            }
        }
    }
    impl EntityCommand for SetFocusPolicy {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::FocusPolicy) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "focus_policy",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut focus_policy) = world.get_mut::<FocusPolicy>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No {2} component found!",
                                                    "focus_policy",
                                                    entity,
                                                    "FocusPolicy",
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if *focus_policy != self.focus_policy {
                *focus_policy = self.focus_policy;
            }
        }
    }
    impl EntityCommand for SetVisibility {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::Visibility) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:73",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(73u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "visibility",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut visibility) = world.get_mut::<Visibility>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:73",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(73u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set {0} property on entity {1:?}: No {2} component found!",
                                                    "visibility",
                                                    entity,
                                                    "Visibility",
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if *visibility != self.visibility {
                *visibility = self.visibility;
            }
        }
    }
    pub struct LockedStyleAttributes(HashSet<LockableStyleAttribute>);
    impl bevy::ecs::component::Component for LockedStyleAttributes
    where
        Self: Send + Sync + 'static,
    {
        type Storage = bevy::ecs::component::TableStorage;
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for LockedStyleAttributes {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "LockedStyleAttributes",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for LockedStyleAttributes {
        #[inline]
        fn default() -> LockedStyleAttributes {
            LockedStyleAttributes(::core::default::Default::default())
        }
    }
    impl LockedStyleAttributes {
        pub fn contains(&self, attr: LockableStyleAttribute) -> bool {
            self.0.contains(&attr)
        }
    }
    pub struct InteractionAnimationState {
        pub iteration: u8,
        pub progress: AnimationProgress,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for InteractionAnimationState {
        #[inline]
        fn clone(&self) -> InteractionAnimationState {
            let _: ::core::clone::AssertParamIsClone<u8>;
            let _: ::core::clone::AssertParamIsClone<AnimationProgress>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for InteractionAnimationState {}
    #[automatically_derived]
    impl ::core::fmt::Debug for InteractionAnimationState {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "InteractionAnimationState",
                "iteration",
                &self.iteration,
                "progress",
                &&self.progress,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for InteractionAnimationState {
        #[inline]
        fn default() -> InteractionAnimationState {
            InteractionAnimationState {
                iteration: ::core::default::Default::default(),
                progress: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for InteractionAnimationState {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for InteractionAnimationState {
        #[inline]
        fn eq(&self, other: &InteractionAnimationState) -> bool {
            self.iteration == other.iteration && self.progress == other.progress
        }
    }
    pub struct StaticBundle<T: Clone + Default> {
        pub base: T,
        pub hover: Option<T>,
        pub press: Option<T>,
        pub cancel: Option<T>,
    }
    #[automatically_derived]
    impl<T: ::core::clone::Clone + Clone + Default> ::core::clone::Clone
    for StaticBundle<T> {
        #[inline]
        fn clone(&self) -> StaticBundle<T> {
            StaticBundle {
                base: ::core::clone::Clone::clone(&self.base),
                hover: ::core::clone::Clone::clone(&self.hover),
                press: ::core::clone::Clone::clone(&self.press),
                cancel: ::core::clone::Clone::clone(&self.cancel),
            }
        }
    }
    #[automatically_derived]
    impl<T: ::core::marker::Copy + Clone + Default> ::core::marker::Copy
    for StaticBundle<T> {}
    #[automatically_derived]
    impl<T: ::core::fmt::Debug + Clone + Default> ::core::fmt::Debug
    for StaticBundle<T> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "StaticBundle",
                "base",
                &self.base,
                "hover",
                &self.hover,
                "press",
                &self.press,
                "cancel",
                &&self.cancel,
            )
        }
    }
    #[automatically_derived]
    impl<T: ::core::default::Default + Clone + Default> ::core::default::Default
    for StaticBundle<T> {
        #[inline]
        fn default() -> StaticBundle<T> {
            StaticBundle {
                base: ::core::default::Default::default(),
                hover: ::core::default::Default::default(),
                press: ::core::default::Default::default(),
                cancel: ::core::default::Default::default(),
            }
        }
    }
    impl<T: Default + Clone> From<T> for StaticBundle<T> {
        fn from(value: T) -> Self {
            StaticBundle::new(value)
        }
    }
    impl<T: Clone + Default> StaticBundle<T> {
        pub fn new(value: T) -> Self {
            StaticBundle {
                base: value,
                ..default()
            }
        }
        pub fn hover(self, value: T) -> Self {
            Self {
                hover: value.into(),
                ..self
            }
        }
        pub fn press(self, value: T) -> Self {
            Self {
                press: value.into(),
                ..self
            }
        }
        pub fn cancel(self, value: T) -> Self {
            Self {
                cancel: value.into(),
                ..self
            }
        }
        fn to_value(&self, flux_interaction: FluxInteraction) -> T {
            match flux_interaction {
                FluxInteraction::None => self.base.clone(),
                FluxInteraction::PointerEnter => {
                    self.hover.clone().unwrap_or(self.base.clone())
                }
                FluxInteraction::PointerLeave => self.base.clone(),
                FluxInteraction::Pressed => {
                    self.press
                        .clone()
                        .unwrap_or(self.hover.clone().unwrap_or(self.base.clone()))
                }
                FluxInteraction::Released => {
                    self.hover.clone().unwrap_or(self.base.clone())
                }
                FluxInteraction::PressCanceled => {
                    self.cancel.clone().unwrap_or(self.base.clone())
                }
                FluxInteraction::Disabled => self.base.clone(),
            }
        }
    }
    pub struct AnimatedBundle<T: Lerp + Default + Clone + Copy + PartialEq> {
        pub base: T,
        pub hover: Option<T>,
        pub press: Option<T>,
        pub cancel: Option<T>,
    }
    #[automatically_derived]
    impl<
        T: ::core::clone::Clone + Lerp + Default + Clone + Copy + PartialEq,
    > ::core::clone::Clone for AnimatedBundle<T> {
        #[inline]
        fn clone(&self) -> AnimatedBundle<T> {
            AnimatedBundle {
                base: ::core::clone::Clone::clone(&self.base),
                hover: ::core::clone::Clone::clone(&self.hover),
                press: ::core::clone::Clone::clone(&self.press),
                cancel: ::core::clone::Clone::clone(&self.cancel),
            }
        }
    }
    #[automatically_derived]
    impl<
        T: ::core::marker::Copy + Lerp + Default + Clone + Copy + PartialEq,
    > ::core::marker::Copy for AnimatedBundle<T> {}
    #[automatically_derived]
    impl<
        T: ::core::fmt::Debug + Lerp + Default + Clone + Copy + PartialEq,
    > ::core::fmt::Debug for AnimatedBundle<T> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "AnimatedBundle",
                "base",
                &self.base,
                "hover",
                &self.hover,
                "press",
                &self.press,
                "cancel",
                &&self.cancel,
            )
        }
    }
    #[automatically_derived]
    impl<
        T: ::core::default::Default + Lerp + Default + Clone + Copy + PartialEq,
    > ::core::default::Default for AnimatedBundle<T> {
        #[inline]
        fn default() -> AnimatedBundle<T> {
            AnimatedBundle {
                base: ::core::default::Default::default(),
                hover: ::core::default::Default::default(),
                press: ::core::default::Default::default(),
                cancel: ::core::default::Default::default(),
            }
        }
    }
    impl<T: Lerp + Default + Clone + Copy + PartialEq> From<T> for AnimatedBundle<T> {
        fn from(value: T) -> Self {
            AnimatedBundle::new(value)
        }
    }
    impl<T: Lerp + Default + Clone + Copy + PartialEq> From<StaticBundle<T>>
    for AnimatedBundle<T> {
        fn from(value: StaticBundle<T>) -> Self {
            Self {
                base: value.base,
                hover: value.hover,
                press: value.press,
                cancel: value.cancel,
                ..default()
            }
        }
    }
    impl<T: Lerp + Default + Clone + Copy + PartialEq> AnimatedBundle<T> {
        pub fn new(value: T) -> Self {
            AnimatedBundle {
                base: value,
                ..default()
            }
        }
        pub fn hover(self, value: T) -> Self {
            Self {
                hover: value.into(),
                ..self
            }
        }
        pub fn press(self, value: T) -> Self {
            Self {
                press: value.into(),
                ..self
            }
        }
        pub fn cancel(self, value: T) -> Self {
            Self {
                cancel: value.into(),
                ..self
            }
        }
        fn phase_value(&self, flux_interaction: FluxInteraction) -> T {
            match flux_interaction {
                FluxInteraction::None => self.base.clone(),
                FluxInteraction::PointerEnter => {
                    self.hover.clone().unwrap_or(self.base.clone())
                }
                FluxInteraction::PointerLeave => self.base.clone(),
                FluxInteraction::Pressed => {
                    self.press
                        .clone()
                        .unwrap_or(self.hover.clone().unwrap_or(self.base.clone()))
                }
                FluxInteraction::Released => {
                    self.hover.clone().unwrap_or(self.base.clone())
                }
                FluxInteraction::PressCanceled => {
                    self.cancel.clone().unwrap_or(self.base.clone())
                }
                FluxInteraction::Disabled => self.base.clone(),
            }
        }
        fn to_value(
            &self,
            transition_base: InteractionAnimationState,
            animation_progress: InteractionAnimationState,
        ) -> T {
            let start_value = match transition_base.progress {
                AnimationProgress::Start(phase) => self.phase_value(phase),
                AnimationProgress::Inbetween(start_phase, end_phase, t) => {
                    self.phase_value(start_phase).lerp(self.phase_value(end_phase), t)
                }
                AnimationProgress::End(phase) => self.phase_value(phase),
            };
            match animation_progress.progress {
                AnimationProgress::Start(_) => start_value,
                AnimationProgress::Inbetween(_, end_phase, t) => {
                    start_value.lerp(self.phase_value(end_phase), t)
                }
                AnimationProgress::End(phase) => self.phase_value(phase),
            }
        }
    }
    pub struct CustomInteractiveStyleAttribute {
        callback: fn(Entity, FluxInteraction, &mut World),
        flux_interaction: FluxInteraction,
    }
    impl EntityCommand for CustomInteractiveStyleAttribute {
        fn apply(self, id: Entity, world: &mut World) {
            (self.callback)(id, self.flux_interaction, world);
        }
    }
    pub struct CustomAnimatableStyleAttribute {
        callback: fn(
            Entity,
            InteractionAnimationState,
            InteractionAnimationState,
            &mut World,
        ),
        transition_base: InteractionAnimationState,
        animation_progress: InteractionAnimationState,
    }
    impl EntityCommand for CustomAnimatableStyleAttribute {
        fn apply(self, id: Entity, world: &mut World) {
            (self.callback)(id, self.transition_base, self.animation_progress, world);
        }
    }
    pub struct InteractiveStyleBuilder<'a> {
        style_builder: &'a mut StyleBuilder,
    }
    pub struct AnimatedStyleBuilder<'a> {
        style_builder: &'a mut StyleBuilder,
    }
    impl<'a> AnimatedStyleBuilder<'a> {
        fn add_and_extract_animation(
            &'a mut self,
            attribute: DynamicStyleAttribute,
        ) -> &'a mut StyleAnimation {
            self.style_builder.add(attribute.clone());
            let index = self
                .style_builder
                .attributes
                .iter()
                .position(|attr| *attr == attribute)
                .unwrap();
            let DynamicStyleAttribute::Animated {
                controller: DynamicStyleController { ref mut animation, .. },
                ..
            } = self.style_builder.attributes[index] else {
                ::core::panicking::panic("internal error: entered unreachable code");
            };
            animation
        }
        pub fn custom(
            &'a mut self,
            callback: impl Into<
                fn(
                    Entity,
                    InteractionAnimationState,
                    InteractionAnimationState,
                    &mut World,
                ),
            >,
        ) -> &'a mut StyleAnimation {
            let attribute = DynamicStyleAttribute::Animated {
                attribute: AnimatedStyleAttribute::Custom(callback.into()),
                controller: DynamicStyleController::default(),
            };
            self.add_and_extract_animation(attribute)
        }
    }
    pub struct StyleBuilder {
        attributes: Vec<DynamicStyleAttribute>,
    }
    impl StyleBuilder {
        pub fn new() -> Self {
            Self {
                attributes: ::alloc::vec::Vec::new(),
            }
        }
        pub fn add(&mut self, attribute: DynamicStyleAttribute) {
            if !self.attributes.contains(&attribute) {
                self.attributes.push(attribute);
            } else {
                let index = self
                    .attributes
                    .iter()
                    .position(|dsa| *dsa == attribute)
                    .unwrap();
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:512",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(512u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Overwriting {0:?} with {1:?}",
                                                    self.attributes[index],
                                                    attribute,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                self.attributes[index] = attribute;
            }
        }
        pub fn interactive<'a>(&'a mut self) -> InteractiveStyleBuilder<'a> {
            InteractiveStyleBuilder {
                style_builder: self,
            }
        }
        pub fn animated<'a>(&'a mut self) -> AnimatedStyleBuilder<'a> {
            AnimatedStyleBuilder {
                style_builder: self,
            }
        }
    }
    impl From<StyleBuilder> for DynamicStyle {
        fn from(value: StyleBuilder) -> Self {
            DynamicStyle::new(value.attributes)
        }
    }
    impl EntityCommand for SetZIndex {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::ZIndex) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:557",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(557u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "z index",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut z_index) = world.get_mut::<ZIndex>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:561",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(561u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set z index on entity {0:?}: No ZIndex component found!",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if let (ZIndex::Local(level), ZIndex::Local(target)) = (
                *z_index,
                self.z_index,
            ) {
                if level != target {
                    *z_index = self.z_index;
                }
            } else if let (ZIndex::Global(level), ZIndex::Global(target)) = (
                *z_index,
                self.z_index,
            ) {
                if level != target {
                    *z_index = self.z_index;
                }
            } else {
                *z_index = self.z_index;
            }
        }
    }
    struct SetImage {
        path: String,
        check_lock: bool,
    }
    impl EntityCommand for SetImage {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::Image) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:591",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(591u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "image",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let handle = if self.path == "" {
                Handle::default()
            } else {
                world.resource::<AssetServer>().load(self.path)
            };
            let Some(mut image) = world.get_mut::<UiImage>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:601",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(601u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set image on entity {0:?}: No UiImage component found!",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if image.texture != handle {
                image.texture = handle;
            }
        }
    }
    pub trait SetImageExt<'a> {
        fn image(&'a mut self, path: impl Into<String>) -> &mut UiStyle<'a>;
    }
    impl<'a> SetImageExt<'a> for UiStyle<'a> {
        fn image(&'a mut self, path: impl Into<String>) -> &mut UiStyle<'a> {
            self.commands
                .add(SetImage {
                    path: path.into(),
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetImageUncheckedExt<'a> {
        fn image(&'a mut self, path: impl Into<String>) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetImageUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn image(&'a mut self, path: impl Into<String>) -> &mut UiStyleUnchecked<'a> {
            self.commands
                .add(SetImage {
                    path: path.into(),
                    check_lock: false,
                });
            self
        }
    }
    impl EntityCommand for SetImageScaleMode {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::ImageScaleMode) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:645",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(645u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "image scale mode",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            if let Some(image_scale_mode) = self.image_scale_mode {
                if let Some(mut scale_mode) = world.get_mut::<ImageScaleMode>(entity) {
                    *scale_mode = image_scale_mode;
                } else {
                    world.entity_mut(entity).insert(image_scale_mode);
                }
            } else if let Some(_) = world.get::<ImageScaleMode>(entity) {
                world.entity_mut(entity).remove::<ImageScaleMode>();
            }
        }
    }
    struct SetFluxInteractionEnabled {
        enabled: bool,
        check_lock: bool,
    }
    impl EntityCommand for SetFluxInteractionEnabled {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::FluxInteraction) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:673",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(673u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "flux interaction",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let Some(mut flux_interaction) = world.get_mut::<FluxInteraction>(entity)
            else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:682",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(682u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set flux interaction on entity {0:?}: No FluxInteraction component found!",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            if self.enabled {
                if *flux_interaction == FluxInteraction::Disabled {
                    *flux_interaction = FluxInteraction::None;
                }
            } else {
                if *flux_interaction != FluxInteraction::Disabled {
                    *flux_interaction = FluxInteraction::Disabled;
                }
            }
        }
    }
    pub trait SetFluxInteractionExt<'a> {
        fn disable_flux_interaction(&'a mut self) -> &mut UiStyle<'a>;
        fn enable_flux_interaction(&'a mut self) -> &mut UiStyle<'a>;
        fn flux_interaction_enabled(&'a mut self, enabled: bool) -> &mut UiStyle<'a>;
    }
    impl<'a> SetFluxInteractionExt<'a> for UiStyle<'a> {
        fn disable_flux_interaction(&'a mut self) -> &mut UiStyle<'a> {
            self.commands
                .add(SetFluxInteractionEnabled {
                    enabled: false,
                    check_lock: true,
                });
            self
        }
        fn enable_flux_interaction(&'a mut self) -> &mut UiStyle<'a> {
            self.commands
                .add(SetFluxInteractionEnabled {
                    enabled: true,
                    check_lock: true,
                });
            self
        }
        fn flux_interaction_enabled(&'a mut self, enabled: bool) -> &mut UiStyle<'a> {
            self.commands
                .add(SetFluxInteractionEnabled {
                    enabled,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetFluxInteractionUncheckedExt<'a> {
        fn disable_flux_interaction(&'a mut self) -> &mut UiStyleUnchecked<'a>;
        fn enable_flux_interaction(&'a mut self) -> &mut UiStyleUnchecked<'a>;
        fn flux_interaction_enabled(
            &'a mut self,
            enabled: bool,
        ) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetFluxInteractionUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn disable_flux_interaction(&'a mut self) -> &mut UiStyleUnchecked<'a> {
            self.commands
                .add(SetFluxInteractionEnabled {
                    enabled: false,
                    check_lock: false,
                });
            self
        }
        fn enable_flux_interaction(&'a mut self) -> &mut UiStyleUnchecked<'a> {
            self.commands
                .add(SetFluxInteractionEnabled {
                    enabled: true,
                    check_lock: false,
                });
            self
        }
        fn flux_interaction_enabled(
            &'a mut self,
            enabled: bool,
        ) -> &mut UiStyleUnchecked<'a> {
            self.commands
                .add(SetFluxInteractionEnabled {
                    enabled,
                    check_lock: false,
                });
            self
        }
    }
    pub trait SetNodeShowHideExt<'a> {
        fn show(&'a mut self) -> &mut UiStyle<'a>;
        fn hide(&'a mut self) -> &mut UiStyle<'a>;
        fn render(&'a mut self, render: bool) -> &mut UiStyle<'a>;
    }
    impl<'a> SetNodeShowHideExt<'a> for UiStyle<'a> {
        fn show(&'a mut self) -> &mut UiStyle<'a> {
            self.commands
                .add(SetVisibility {
                    visibility: Visibility::Inherited,
                    check_lock: true,
                })
                .add(SetDisplay {
                    display: Display::Flex,
                    check_lock: true,
                });
            self
        }
        fn hide(&'a mut self) -> &mut UiStyle<'a> {
            self.commands
                .add(SetVisibility {
                    visibility: Visibility::Hidden,
                    check_lock: true,
                })
                .add(SetDisplay {
                    display: Display::None,
                    check_lock: true,
                });
            self
        }
        fn render(&'a mut self, render: bool) -> &mut UiStyle<'a> {
            if render {
                self.commands
                    .add(SetVisibility {
                        visibility: Visibility::Inherited,
                        check_lock: true,
                    })
                    .add(SetDisplay {
                        display: Display::Flex,
                        check_lock: true,
                    });
            } else {
                self.commands
                    .add(SetVisibility {
                        visibility: Visibility::Hidden,
                        check_lock: true,
                    })
                    .add(SetDisplay {
                        display: Display::None,
                        check_lock: true,
                    });
            }
            self
        }
    }
    pub trait SetNodeShowHideUncheckedExt<'a> {
        fn show(&'a mut self) -> &mut UiStyleUnchecked<'a>;
        fn hide(&'a mut self) -> &mut UiStyleUnchecked<'a>;
        fn render(&'a mut self, render: bool) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetNodeShowHideUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn show(&'a mut self) -> &mut UiStyleUnchecked<'a> {
            self.commands
                .add(SetVisibility {
                    visibility: Visibility::Inherited,
                    check_lock: false,
                })
                .add(SetDisplay {
                    display: Display::Flex,
                    check_lock: false,
                });
            self
        }
        fn hide(&'a mut self) -> &mut UiStyleUnchecked<'a> {
            self.commands
                .add(SetVisibility {
                    visibility: Visibility::Hidden,
                    check_lock: false,
                })
                .add(SetDisplay {
                    display: Display::None,
                    check_lock: false,
                });
            self
        }
        fn render(&'a mut self, render: bool) -> &mut UiStyleUnchecked<'a> {
            if render {
                self.commands
                    .add(SetVisibility {
                        visibility: Visibility::Inherited,
                        check_lock: false,
                    })
                    .add(SetDisplay {
                        display: Display::Flex,
                        check_lock: false,
                    });
            } else {
                self.commands
                    .add(SetVisibility {
                        visibility: Visibility::Hidden,
                        check_lock: false,
                    })
                    .add(SetDisplay {
                        display: Display::None,
                        check_lock: false,
                    });
            }
            self
        }
    }
    struct SetAbsolutePosition {
        absolute_position: Vec2,
        check_lock: bool,
    }
    impl EntityCommand for SetAbsolutePosition {
        fn apply(self, entity: Entity, world: &mut World) {
            if self.check_lock {
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::PositionType) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:894",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(894u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "position_type",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::Top) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:900",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(900u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "position: top",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
                if let Some(locked_attrs) = world.get::<LockedStyleAttributes>(entity) {
                    if locked_attrs.contains(LockableStyleAttribute::Left) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event sickle_ui\\src\\ui_style.rs:901",
                                        "sickle_ui::ui_style",
                                        ::tracing::Level::WARN,
                                        ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                        ::core::option::Option::Some(901u32),
                                        ::core::option::Option::Some("sickle_ui::ui_style"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Failed to style {0} property on entity {1:?}: Attribute locked!",
                                                            "position: left",
                                                            entity,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return;
                    }
                }
            }
            let offset = if let Some(parent) = world.get::<Parent>(entity) {
                let Some(parent_node) = world.get::<Node>(parent.get()) else {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event sickle_ui\\src\\ui_style.rs:911",
                                    "sickle_ui::ui_style",
                                    ::tracing::Level::WARN,
                                    ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                    ::core::option::Option::Some(911u32),
                                    ::core::option::Option::Some("sickle_ui::ui_style"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::WARN
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::WARN
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::core::iter::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::core::option::Option::Some(
                                                    &format_args!(
                                                        "Failed to set position on entity {0:?}: Parent has no Node component!",
                                                        entity,
                                                    ) as &dyn Value,
                                                ),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    return;
                };
                let size = parent_node.size();
                let Some(parent_transform) = world.get::<GlobalTransform>(parent.get())
                else {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event sickle_ui\\src\\ui_style.rs:920",
                                    "sickle_ui::ui_style",
                                    ::tracing::Level::WARN,
                                    ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                    ::core::option::Option::Some(920u32),
                                    ::core::option::Option::Some("sickle_ui::ui_style"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::WARN
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::WARN
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::core::iter::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::core::option::Option::Some(
                                                    &format_args!(
                                                        "Failed to set position on entity {0:?}: Parent has no GlobalTransform component!",
                                                        entity,
                                                    ) as &dyn Value,
                                                ),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    return;
                };
                parent_transform.translation().truncate() - (size / 2.)
            } else {
                Vec2::ZERO
            };
            let Some(mut style) = world.get_mut::<Style>(entity) else {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event sickle_ui\\src\\ui_style.rs:933",
                                "sickle_ui::ui_style",
                                ::tracing::Level::WARN,
                                ::core::option::Option::Some("sickle_ui\\src\\ui_style.rs"),
                                ::core::option::Option::Some(933u32),
                                ::core::option::Option::Some("sickle_ui::ui_style"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::WARN
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &format_args!(
                                                    "Failed to set position on entity {0:?}: No Style component found!",
                                                    entity,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            };
            style.position_type = PositionType::Absolute;
            style.top = Val::Px(self.absolute_position.y - offset.y);
            style.left = Val::Px(self.absolute_position.x - offset.x);
        }
    }
    pub trait SetAbsolutePositionExt<'a> {
        fn absolute_position(&'a mut self, position: Vec2) -> &mut UiStyle<'a>;
    }
    impl<'a> SetAbsolutePositionExt<'a> for UiStyle<'a> {
        fn absolute_position(&'a mut self, position: Vec2) -> &mut UiStyle<'a> {
            self.commands
                .add(SetAbsolutePosition {
                    absolute_position: position,
                    check_lock: true,
                });
            self
        }
    }
    pub trait SetAbsolutePositionUncheckedExt<'a> {
        fn absolute_position(&'a mut self, position: Vec2) -> &mut UiStyleUnchecked<'a>;
    }
    impl<'a> SetAbsolutePositionUncheckedExt<'a> for UiStyleUnchecked<'a> {
        fn absolute_position(&'a mut self, position: Vec2) -> &mut UiStyleUnchecked<'a> {
            self.commands
                .add(SetAbsolutePosition {
                    absolute_position: position,
                    check_lock: false,
                });
            self
        }
    }
}
