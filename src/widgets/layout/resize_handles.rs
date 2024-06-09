use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use bevy_reflect::Reflect;

use sickle_math::ease::Ease;
use sickle_ui_scaffold::{
    animated_interaction::{AnimatedInteraction, AnimationConfig},
    interactions::InteractiveBackground,
    prelude::*,
    ui_commands::SetCursorExt,
};

use super::container::UiContainerExt;

const RESIZE_HANDLES_LOCAL_Z_INDEX: i32 = 100;
const HANDLE_NORTH: &'static str = "HandleNorth";
const HANDLE_NORTH_EAST: &'static str = "HandleNorthEast";
const HANDLE_EAST: &'static str = "HandleEast";
const HANDLE_SOUTH_EAST: &'static str = "HandleSouthEast";
const HANDLE_SOUTH: &'static str = "HandleSouth";
const HANDLE_SOUTH_WEST: &'static str = "HandleSouthWest";
const HANDLE_WEST: &'static str = "HandleWest";
const HANDLE_NORTH_WEST: &'static str = "HandleNorthWest";

pub struct ResizeHandlePlugin;

impl Plugin for ResizeHandlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ComponentThemePlugin::<ResizeHandles>::default())
            .add_systems(
                Update,
                update_cursor_on_resize_handles
                    .run_if(should_update_resize_handle_cursor)
                    .after(FluxInteractionUpdate),
            );
    }
}

fn should_update_resize_handle_cursor(
    q_flux: Query<&ResizeHandle, Changed<FluxInteraction>>,
) -> bool {
    q_flux.iter().count() > 0
}

fn update_cursor_on_resize_handles(
    q_flux: Query<(&ResizeHandle, &FluxInteraction)>,
    mut locked: Local<bool>,
    mut commands: Commands,
) {
    let mut new_cursor: Option<CursorIcon> = None;
    let multiple_active = q_flux
        .iter()
        .filter(|(_, flux)| {
            (**flux == FluxInteraction::PointerEnter && !*locked)
                || **flux == FluxInteraction::Pressed
        })
        .count()
        > 1;

    // TODO: use the correct diagonal when the active handles have the same parent
    let omni_cursor = CursorIcon::Move;

    for (handle, flux) in &q_flux {
        match *flux {
            FluxInteraction::PointerEnter => {
                if !*locked {
                    new_cursor = match multiple_active {
                        true => omni_cursor.into(),
                        false => handle.direction.cursor().into(),
                    };
                }
            }
            FluxInteraction::Pressed => {
                new_cursor = match multiple_active {
                    true => omni_cursor.into(),
                    false => handle.direction.cursor().into(),
                };
                *locked = true;
            }
            FluxInteraction::Released => {
                *locked = false;
                if new_cursor.is_none() {
                    new_cursor = CursorIcon::Default.into();
                }
            }
            FluxInteraction::PressCanceled => {
                *locked = false;
                if new_cursor.is_none() {
                    new_cursor = CursorIcon::Default.into();
                }
            }
            FluxInteraction::PointerLeave => {
                if !*locked && new_cursor.is_none() {
                    new_cursor = CursorIcon::Default.into();
                }
            }
            _ => (),
        }
    }

    if let Some(new_cursor) = new_cursor {
        commands.set_cursor(new_cursor);
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Reflect)]
pub enum ResizeDirection {
    #[default]
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl ResizeDirection {
    pub fn cursor(&self) -> CursorIcon {
        match self {
            ResizeDirection::North => CursorIcon::NResize,
            ResizeDirection::NorthEast => CursorIcon::NeResize,
            ResizeDirection::East => CursorIcon::EResize,
            ResizeDirection::SouthEast => CursorIcon::SeResize,
            ResizeDirection::South => CursorIcon::SResize,
            ResizeDirection::SouthWest => CursorIcon::SwResize,
            ResizeDirection::West => CursorIcon::WResize,
            ResizeDirection::NorthWest => CursorIcon::NwResize,
        }
    }

    pub fn to_size_diff(&self, drag_diff: Vec2) -> Vec2 {
        match self {
            ResizeDirection::North => Vec2 {
                x: 0.,
                y: -drag_diff.y,
            },
            ResizeDirection::NorthEast => Vec2 {
                x: drag_diff.x,
                y: -drag_diff.y,
            },
            ResizeDirection::East => Vec2 {
                x: drag_diff.x,
                y: 0.,
            },
            ResizeDirection::SouthEast => drag_diff,
            ResizeDirection::South => Vec2 {
                x: 0.,
                y: drag_diff.y,
            },
            ResizeDirection::SouthWest => Vec2 {
                x: -drag_diff.x,
                y: drag_diff.y,
            },
            ResizeDirection::West => Vec2 {
                x: -drag_diff.x,
                y: 0.,
            },
            ResizeDirection::NorthWest => Vec2 {
                x: -drag_diff.x,
                y: -drag_diff.y,
            },
        }
    }
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct ResizeHandle {
    direction: ResizeDirection,
}

impl ResizeHandle {
    pub fn direction(&self) -> ResizeDirection {
        self.direction
    }

    fn base_tween() -> AnimationConfig {
        AnimationConfig {
            duration: 0.1,
            easing: Ease::OutExpo,
            ..default()
        }
    }

    pub fn resize_zone_size() -> f32 {
        4.
    }

    pub fn resize_zone_pullback() -> f32 {
        2.
    }

    pub fn resize_handle_container(elevation: i32) -> impl Bundle {
        (
            Name::new("Resize Handle Container"),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::SpaceBetween,
                    align_self: AlignSelf::Stretch,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                z_index: ZIndex::Local(elevation),
                ..default()
            },
        )
    }

    pub fn resize_handle(direction: ResizeDirection) -> impl Bundle {
        let zone_size = ResizeHandle::resize_zone_size();

        let (width, height) = match direction {
            ResizeDirection::North => (Val::Percent(100.), Val::Px(zone_size)),
            ResizeDirection::NorthEast => (Val::Px(zone_size), Val::Px(zone_size)),
            ResizeDirection::East => (Val::Px(zone_size), Val::Percent(100.)),
            ResizeDirection::SouthEast => (Val::Px(zone_size), Val::Px(zone_size)),
            ResizeDirection::South => (Val::Percent(100.), Val::Px(zone_size)),
            ResizeDirection::SouthWest => (Val::Px(zone_size), Val::Px(zone_size)),
            ResizeDirection::West => (Val::Px(zone_size), Val::Percent(100.)),
            ResizeDirection::NorthWest => (Val::Px(zone_size), Val::Px(zone_size)),
        };
        let name = match direction {
            ResizeDirection::North => "North",
            ResizeDirection::NorthEast => "NorthEast",
            ResizeDirection::East => "East",
            ResizeDirection::SouthEast => "SouthEast",
            ResizeDirection::South => "South",
            ResizeDirection::SouthWest => "SouthWest",
            ResizeDirection::West => "West",
            ResizeDirection::NorthWest => "NorthWest",
        };

        let pullback = Val::Px(-ResizeHandle::resize_zone_pullback());
        (
            Name::new(format!("Resize Handle: [{}]", name)),
            NodeBundle {
                style: Style {
                    top: pullback,
                    left: pullback,
                    width,
                    height,
                    ..default()
                },
                focus_policy: bevy::ui::FocusPolicy::Pass,
                ..default()
            },
            Interaction::default(),
            TrackedInteraction::default(),
            InteractiveBackground {
                highlight: Color::rgb(0., 0.5, 1.).into(),
                ..default()
            },
            AnimatedInteraction::<InteractiveBackground> {
                tween: ResizeHandle::base_tween(),
                ..default()
            },
            Draggable::default(),
            RelativeCursorPosition::default(),
            ResizeHandle { direction },
        )
    }
}

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct ResizeHandles {
    pub handle_north: Entity,
    handle_north_east: Entity,
    handle_east: Entity,
    handle_south_east: Entity,
    handle_south: Entity,
    handle_south_west: Entity,
    handle_west: Entity,
    handle_north_west: Entity,
}

impl Default for ResizeHandles {
    fn default() -> Self {
        Self {
            handle_north: Entity::PLACEHOLDER,
            handle_north_east: Entity::PLACEHOLDER,
            handle_east: Entity::PLACEHOLDER,
            handle_south_east: Entity::PLACEHOLDER,
            handle_south: Entity::PLACEHOLDER,
            handle_south_west: Entity::PLACEHOLDER,
            handle_west: Entity::PLACEHOLDER,
            handle_north_west: Entity::PLACEHOLDER,
        }
    }
}

impl UiContext for ResizeHandles {
    fn get(&self, target: &str) -> Result<Entity, String> {
        match target {
            HANDLE_NORTH => Ok(self.handle_north),
            HANDLE_NORTH_EAST => Ok(self.handle_north_east),
            HANDLE_EAST => Ok(self.handle_east),
            HANDLE_SOUTH_EAST => Ok(self.handle_south_east),
            HANDLE_SOUTH => Ok(self.handle_south),
            HANDLE_SOUTH_WEST => Ok(self.handle_south_west),
            HANDLE_WEST => Ok(self.handle_west),
            HANDLE_NORTH_WEST => Ok(self.handle_north_west),
            _ => Err(format!(
                "{} doesn't exists for ResizeHandles. Possible contexts: {:?}",
                target,
                self.contexts()
            )),
        }
    }

    fn contexts(&self) -> Vec<&'static str> {
        vec![
            HANDLE_NORTH,
            HANDLE_NORTH_EAST,
            HANDLE_EAST,
            HANDLE_SOUTH_EAST,
            HANDLE_SOUTH,
            HANDLE_SOUTH_WEST,
            HANDLE_WEST,
            HANDLE_NORTH_WEST,
        ]
    }
}

impl DefaultTheme for ResizeHandles {
    fn default_theme() -> Option<Theme<ResizeHandles>> {
        ResizeHandles::theme().into()
    }
}

impl ResizeHandles {
    pub fn theme() -> Theme<ResizeHandles> {
        let base_theme = PseudoTheme::deferred_world(None, ResizeHandles::primary_style);
        let theme_north = PseudoTheme::deferred(
            vec![PseudoState::Resizable(CardinalDirection::North)],
            ResizeHandles::resizable_north,
        );
        let theme_north_north_east = PseudoTheme::deferred(
            vec![
                PseudoState::Resizable(CardinalDirection::North),
                PseudoState::Resizable(CardinalDirection::NorthEast),
            ],
            ResizeHandles::resizable_north_north_east,
        );
        let theme_north_north_west = PseudoTheme::deferred(
            vec![
                PseudoState::Resizable(CardinalDirection::North),
                PseudoState::Resizable(CardinalDirection::NorthWest),
            ],
            ResizeHandles::resizable_north_north_west,
        );

        let theme_north_east = PseudoTheme::deferred(
            vec![PseudoState::Resizable(CardinalDirection::NorthEast)],
            ResizeHandles::resizable_north_east,
        );

        let theme_east = PseudoTheme::deferred(
            vec![PseudoState::Resizable(CardinalDirection::East)],
            ResizeHandles::resizable_east,
        );
        let theme_east_north_east = PseudoTheme::deferred(
            vec![
                PseudoState::Resizable(CardinalDirection::East),
                PseudoState::Resizable(CardinalDirection::NorthEast),
            ],
            ResizeHandles::resizable_east_north_east,
        );
        let theme_east_south_east = PseudoTheme::deferred(
            vec![
                PseudoState::Resizable(CardinalDirection::East),
                PseudoState::Resizable(CardinalDirection::SouthEast),
            ],
            ResizeHandles::resizable_east_south_east,
        );

        let theme_south_east = PseudoTheme::deferred(
            vec![PseudoState::Resizable(CardinalDirection::SouthEast)],
            ResizeHandles::resizable_south_east,
        );

        let theme_south = PseudoTheme::deferred(
            vec![PseudoState::Resizable(CardinalDirection::South)],
            ResizeHandles::resizable_south,
        );
        let theme_south_south_east = PseudoTheme::deferred(
            vec![
                PseudoState::Resizable(CardinalDirection::South),
                PseudoState::Resizable(CardinalDirection::SouthEast),
            ],
            ResizeHandles::resizable_south_south_east,
        );
        let theme_south_south_west = PseudoTheme::deferred(
            vec![
                PseudoState::Resizable(CardinalDirection::South),
                PseudoState::Resizable(CardinalDirection::SouthWest),
            ],
            ResizeHandles::resizable_south_south_west,
        );

        let theme_south_west = PseudoTheme::deferred(
            vec![PseudoState::Resizable(CardinalDirection::SouthWest)],
            ResizeHandles::resizable_south_west,
        );

        let theme_west = PseudoTheme::deferred(
            vec![PseudoState::Resizable(CardinalDirection::West)],
            ResizeHandles::resizable_west,
        );
        let theme_west_south_west = PseudoTheme::deferred(
            vec![
                PseudoState::Resizable(CardinalDirection::West),
                PseudoState::Resizable(CardinalDirection::SouthWest),
            ],
            ResizeHandles::resizable_west_south_west,
        );
        let theme_west_north_west = PseudoTheme::deferred(
            vec![
                PseudoState::Resizable(CardinalDirection::West),
                PseudoState::Resizable(CardinalDirection::NorthWest),
            ],
            ResizeHandles::resizable_west_north_west,
        );

        let theme_north_west = PseudoTheme::deferred(
            vec![PseudoState::Resizable(CardinalDirection::NorthWest)],
            ResizeHandles::resizable_north_west,
        );

        Theme::new(vec![
            base_theme,
            theme_north,
            theme_north_north_west,
            theme_north_north_east,
            theme_north_east,
            theme_east,
            theme_east_north_east,
            theme_east_south_east,
            theme_south_east,
            theme_south,
            theme_south_south_east,
            theme_south_south_west,
            theme_south_west,
            theme_west,
            theme_west_south_west,
            theme_west_north_west,
            theme_north_west,
        ])
    }

    fn primary_style(
        style_builder: &mut StyleBuilder,
        entity: Entity,
        _: &ResizeHandles,
        world: &mut World,
    ) {
        let theme_data = world.resource::<ThemeData>();
        let resize_spacing = theme_data.spacing.resize_zone;
        let interaction_animation = theme_data.delayed_interaction_animation;
        let colors = theme_data.colors();

        let parent_id: Option<Entity> = match world.get::<Parent>(entity) {
            Some(parent) => Some(parent.get()),
            None => None,
        };

        let parent_border_px = match parent_id {
            Some(parent) => UiUtils::border_as_px(parent, world),
            None => Vec4::ZERO,
        };

        // TODO: bevy 0.14: account for parent border radius
        let pullback = match parent_id {
            Some(parent) => {
                if let Some(parent_style) = world.get::<Style>(parent) {
                    Vec2::new(
                        match parent_style.overflow.x {
                            OverflowAxis::Visible => resize_spacing.pullback,
                            OverflowAxis::Clip => 0.,
                        },
                        match parent_style.overflow.y {
                            OverflowAxis::Visible => resize_spacing.pullback,
                            OverflowAxis::Clip => 0.,
                        },
                    )
                } else {
                    Vec2::splat(resize_spacing.pullback)
                }
            }
            None => Vec2::splat(resize_spacing.pullback),
        };

        let handle_color = AnimatedVals {
            idle: Color::NONE,
            hover: colors.accent(Accent::Outline).into(),
            ..default()
        };

        style_builder
            .position_type(PositionType::Absolute)
            .flex_shrink(0.)
            .top(Val::Px(-parent_border_px.x - pullback.y))
            .right(Val::Px(parent_border_px.w - pullback.x))
            .bottom(Val::Px(parent_border_px.x - pullback.y))
            .left(Val::Px(-parent_border_px.w - pullback.x));

        style_builder
            .switch_placement(HANDLE_NORTH)
            .height(Val::Px(resize_spacing.width))
            .visibility(Visibility::Hidden)
            .animated()
            .background_color(handle_color.clone())
            .copy_from(interaction_animation);

        style_builder
            .switch_placement(HANDLE_NORTH_EAST)
            .height(Val::Px(resize_spacing.width))
            .width(Val::Px(resize_spacing.width))
            .top(Val::Px(0.))
            .right(Val::Px(0.))
            .visibility(Visibility::Hidden)
            .animated()
            .background_color(handle_color.clone())
            .copy_from(interaction_animation);

        style_builder
            .switch_placement(HANDLE_EAST)
            .width(Val::Px(resize_spacing.width))
            .right(Val::Px(0.))
            .visibility(Visibility::Hidden)
            .animated()
            .background_color(handle_color.clone())
            .copy_from(interaction_animation);

        style_builder
            .switch_placement(HANDLE_SOUTH_EAST)
            .width(Val::Px(resize_spacing.width))
            .height(Val::Px(resize_spacing.width))
            .right(Val::Px(0.))
            .bottom(Val::Px(0.))
            .visibility(Visibility::Hidden)
            .animated()
            .background_color(handle_color.clone())
            .copy_from(interaction_animation);

        style_builder
            .switch_placement(HANDLE_SOUTH)
            .height(Val::Px(resize_spacing.width))
            .bottom(Val::Px(0.))
            .visibility(Visibility::Hidden)
            .animated()
            .background_color(handle_color.clone())
            .copy_from(interaction_animation);

        style_builder
            .switch_placement(HANDLE_SOUTH_WEST)
            .width(Val::Px(resize_spacing.width))
            .height(Val::Px(resize_spacing.width))
            .bottom(Val::Px(0.))
            .left(Val::Px(0.))
            .visibility(Visibility::Hidden)
            .animated()
            .background_color(handle_color.clone())
            .copy_from(interaction_animation);

        style_builder
            .switch_placement(HANDLE_WEST)
            .width(Val::Px(resize_spacing.width))
            .left(Val::Px(0.))
            .visibility(Visibility::Hidden)
            .animated()
            .background_color(handle_color.clone())
            .copy_from(interaction_animation);

        style_builder
            .switch_placement(HANDLE_NORTH_WEST)
            .width(Val::Px(resize_spacing.width))
            .height(Val::Px(resize_spacing.width))
            .top(Val::Px(0.))
            .left(Val::Px(0.))
            .visibility(Visibility::Hidden)
            .animated()
            .background_color(handle_color.clone())
            .copy_from(interaction_animation);
    }

    // North handle
    fn resizable_north(style_builder: &mut StyleBuilder, _theme_data: &ThemeData) {
        style_builder
            .switch_target(HANDLE_NORTH)
            .right(Val::Px(0.))
            .left(Val::Px(0.))
            .visibility(Visibility::Inherited);
    }
    fn resizable_north_north_west(style_builder: &mut StyleBuilder, theme_data: &ThemeData) {
        let resize_spacing = theme_data.spacing.resize_zone;

        style_builder
            .switch_target(HANDLE_NORTH)
            .left(Val::Px(resize_spacing.width + resize_spacing.handle_gap));
    }
    fn resizable_north_north_east(style_builder: &mut StyleBuilder, theme_data: &ThemeData) {
        let resize_spacing = theme_data.spacing.resize_zone;

        style_builder
            .switch_target(HANDLE_NORTH)
            .right(Val::Px(resize_spacing.width + resize_spacing.handle_gap));
    }

    // North-east corner
    fn resizable_north_east(style_builder: &mut StyleBuilder, _theme_data: &ThemeData) {
        style_builder
            .switch_target(HANDLE_NORTH_EAST)
            .visibility(Visibility::Inherited);
    }

    // East handle
    fn resizable_east(style_builder: &mut StyleBuilder, _theme_data: &ThemeData) {
        style_builder
            .switch_target(HANDLE_EAST)
            .top(Val::Px(0.))
            .bottom(Val::Px(0.))
            .visibility(Visibility::Inherited);
    }
    fn resizable_east_north_east(style_builder: &mut StyleBuilder, theme_data: &ThemeData) {
        let resize_spacing = theme_data.spacing.resize_zone;

        style_builder
            .switch_target(HANDLE_EAST)
            .top(Val::Px(resize_spacing.width + resize_spacing.handle_gap));
    }
    fn resizable_east_south_east(style_builder: &mut StyleBuilder, theme_data: &ThemeData) {
        let resize_spacing = theme_data.spacing.resize_zone;

        style_builder
            .switch_target(HANDLE_EAST)
            .bottom(Val::Px(resize_spacing.width + resize_spacing.handle_gap));
    }

    // South-east corner
    fn resizable_south_east(style_builder: &mut StyleBuilder, _theme_data: &ThemeData) {
        style_builder
            .switch_target(HANDLE_SOUTH_EAST)
            .visibility(Visibility::Inherited);
    }

    // South handle
    fn resizable_south(style_builder: &mut StyleBuilder, _theme_data: &ThemeData) {
        style_builder
            .switch_target(HANDLE_SOUTH)
            .right(Val::Px(0.))
            .left(Val::Px(0.))
            .visibility(Visibility::Inherited);
    }
    fn resizable_south_south_east(style_builder: &mut StyleBuilder, theme_data: &ThemeData) {
        let resize_spacing = theme_data.spacing.resize_zone;

        style_builder
            .switch_target(HANDLE_SOUTH)
            .right(Val::Px(resize_spacing.width + resize_spacing.handle_gap));
    }
    fn resizable_south_south_west(style_builder: &mut StyleBuilder, theme_data: &ThemeData) {
        let resize_spacing = theme_data.spacing.resize_zone;

        style_builder
            .switch_target(HANDLE_SOUTH)
            .left(Val::Px(resize_spacing.width + resize_spacing.handle_gap));
    }

    // South-west corner
    fn resizable_south_west(style_builder: &mut StyleBuilder, _theme_data: &ThemeData) {
        style_builder
            .switch_target(HANDLE_SOUTH_WEST)
            .visibility(Visibility::Inherited);
    }

    // West handle
    fn resizable_west(style_builder: &mut StyleBuilder, _theme_data: &ThemeData) {
        style_builder
            .switch_target(HANDLE_WEST)
            .top(Val::Px(0.))
            .bottom(Val::Px(0.))
            .visibility(Visibility::Inherited);
    }
    fn resizable_west_south_west(style_builder: &mut StyleBuilder, theme_data: &ThemeData) {
        let resize_spacing = theme_data.spacing.resize_zone;

        style_builder
            .switch_target(HANDLE_WEST)
            .bottom(Val::Px(resize_spacing.width + resize_spacing.handle_gap));
    }
    fn resizable_west_north_west(style_builder: &mut StyleBuilder, theme_data: &ThemeData) {
        let resize_spacing = theme_data.spacing.resize_zone;

        style_builder
            .switch_target(HANDLE_WEST)
            .top(Val::Px(resize_spacing.width + resize_spacing.handle_gap));
    }

    // North-west corner
    fn resizable_north_west(style_builder: &mut StyleBuilder, _theme_data: &ThemeData) {
        style_builder
            .switch_target(HANDLE_NORTH_WEST)
            .visibility(Visibility::Inherited);
    }

    fn container() -> impl Bundle {
        (
            Name::new("Resize Handles"),
            NodeBundle {
                style: Style {
                    overflow: Overflow::visible(),
                    ..default()
                },
                z_index: ZIndex::Local(RESIZE_HANDLES_LOCAL_Z_INDEX),
                focus_policy: bevy::ui::FocusPolicy::Pass,
                ..default()
            },
            LockedStyleAttributes::from_vec(vec![
                LockableStyleAttribute::FocusPolicy,
                LockableStyleAttribute::Overflow,
            ]),
        )
    }

    pub fn handle(&self, direction: ResizeDirection) -> Entity {
        match direction {
            ResizeDirection::North => self.handle_north,
            ResizeDirection::NorthEast => self.handle_north_east,
            ResizeDirection::East => self.handle_east,
            ResizeDirection::SouthEast => self.handle_south_east,
            ResizeDirection::South => self.handle_south,
            ResizeDirection::SouthWest => self.handle_south_west,
            ResizeDirection::West => self.handle_west,
            ResizeDirection::NorthWest => self.handle_north_west,
        }
    }

    fn resize_handle(direction: ResizeDirection) -> impl Bundle {
        let name = match direction {
            ResizeDirection::North => "North",
            ResizeDirection::NorthEast => "NorthEast",
            ResizeDirection::East => "East",
            ResizeDirection::SouthEast => "SouthEast",
            ResizeDirection::South => "South",
            ResizeDirection::SouthWest => "SouthWest",
            ResizeDirection::West => "West",
            ResizeDirection::NorthWest => "NorthWest",
        };

        (
            Name::new(format!("Resize Handle: [{}]", name)),
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                focus_policy: bevy::ui::FocusPolicy::Pass,
                ..default()
            },
            TrackedInteraction::default(),
            Draggable::default(),
            RelativeCursorPosition::default(),
            ResizeHandle { direction },
            LockedStyleAttributes::from_vec(vec![
                LockableStyleAttribute::FocusPolicy,
                LockableStyleAttribute::PositionType,
            ]),
        )
    }
}

pub trait UiResizeHandlesExt<'w, 's> {
    fn resize_handles<'a>(
        &'a mut self,
        marker: impl Bundle + Clone,
    ) -> UiBuilder<'w, 's, 'a, Entity>;
}

impl<'w, 's> UiResizeHandlesExt<'w, 's> for UiBuilder<'w, 's, '_, Entity> {
    fn resize_handles<'a>(
        &'a mut self,
        marker: impl Bundle + Clone,
    ) -> UiBuilder<'w, 's, 'a, Entity> {
        let mut resize_handles = ResizeHandles::default();
        let container = self
            .container(ResizeHandles::container(), |resize_container| {
                resize_handles.handle_north = resize_container
                    .spawn((
                        ResizeHandles::resize_handle(ResizeDirection::North),
                        marker.clone(),
                    ))
                    .id();
                resize_handles.handle_north_east = resize_container
                    .spawn((
                        ResizeHandles::resize_handle(ResizeDirection::NorthEast),
                        marker.clone(),
                    ))
                    .id();
                resize_handles.handle_east = resize_container
                    .spawn((
                        ResizeHandles::resize_handle(ResizeDirection::East),
                        marker.clone(),
                    ))
                    .id();
                resize_handles.handle_south_east = resize_container
                    .spawn((
                        ResizeHandles::resize_handle(ResizeDirection::SouthEast),
                        marker.clone(),
                    ))
                    .id();
                resize_handles.handle_south = resize_container
                    .spawn((
                        ResizeHandles::resize_handle(ResizeDirection::South),
                        marker.clone(),
                    ))
                    .id();
                resize_handles.handle_south_west = resize_container
                    .spawn((
                        ResizeHandles::resize_handle(ResizeDirection::SouthWest),
                        marker.clone(),
                    ))
                    .id();
                resize_handles.handle_west = resize_container
                    .spawn((
                        ResizeHandles::resize_handle(ResizeDirection::West),
                        marker.clone(),
                    ))
                    .id();
                resize_handles.handle_north_west = resize_container
                    .spawn((
                        ResizeHandles::resize_handle(ResizeDirection::NorthWest),
                        marker.clone(),
                    ))
                    .id();
            })
            .insert(resize_handles.clone())
            .id();

        self.commands().ui_builder(container)
    }
}
