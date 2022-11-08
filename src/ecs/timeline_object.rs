use super::ecs_builder::EcsBuilder;

pub trait TimelineObject: IntoIterator<Item = anyhow::Result<EcsBuilder>> {
}
