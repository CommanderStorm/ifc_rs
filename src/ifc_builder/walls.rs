use std::collections::HashSet;

use glam::{DVec2, DVec3};

use crate::prelude::*;

pub struct VerticalWallParameter {
    pub height: f64,
    pub length: f64,
    pub placement: DVec3,
}

impl<'a> IfcBuildingBuilder<'a> {
    pub fn vertical_wall(
        &mut self,
        material: TypedId<MaterialLayerSetUsage>,
        wall_type: TypedId<WallType>,
        name: &str,
        wall_information: VerticalWallParameter,
    ) -> TypedId<Wall> {
        let position = Axis3D::new(Point3D::from(wall_information.placement), self.ifc);
        let wall_thickness = self.calculate_material_layer_set_thickness(material);

        let shape_repr = ShapeRepresentation::new(self.sub_context, self.ifc).add_item(
            ExtrudedAreaSolid::new(
                RectangleProfileDef::new(
                    ProfileType::Area,
                    wall_information.length,
                    wall_thickness,
                )
                // center of the rectangle
                .position(
                    Axis2D::new(
                        Point2D::from(DVec2::new(
                            wall_information.length * 0.5,
                            wall_thickness * 0.5,
                        )),
                        self.ifc,
                    ),
                    self.ifc,
                ),
                // vertical wall (z-up)
                Direction3D::from(DVec3::new(0.0, 0.0, 1.0)),
                wall_information.height,
                self.ifc,
            ),
            self.ifc,
        );

        let product_shape = ProductDefinitionShape::new().add_representation(shape_repr, self.ifc);
        let local_placement = LocalPlacement::new(position, self.ifc);

        let wall = Wall::new(name)
            .owner_history(self.owner_history, self.ifc)
            .object_placement(local_placement, self.ifc)
            .representation(product_shape, self.ifc);

        self.wall(material, wall_type, wall)
    }

    pub fn wall_type(
        &mut self,
        material: TypedId<MaterialLayerSet>,
        name: &str,
        wall_type: WallTypeEnum,
    ) -> TypedId<WallType> {
        let wall_type = WallType::new(name, wall_type)
            .owner_history(self.owner_history, self.ifc)
            .name(name);

        let wall_type_id = self.ifc.data.insert_new(wall_type);

        self.wall_type_to_wall.insert(wall_type_id, HashSet::new());
        self.material_to_wall_type
            .get_mut(&material)
            .unwrap()
            .insert(wall_type_id);

        wall_type_id
    }

    fn wall(
        &mut self,
        material: TypedId<MaterialLayerSetUsage>,
        wall_type: TypedId<WallType>,
        wall: Wall,
    ) -> TypedId<Wall> {
        let wall_id = self.ifc.data.insert_new(wall);

        self.walls.insert(wall_id);
        self.wall_type_to_wall
            .get_mut(&wall_type)
            .unwrap()
            .insert(wall_id);
        self.material_to_wall
            .get_mut(&material)
            .unwrap()
            .insert(wall_id);

        wall_id
    }
}

#[cfg(test)]
mod test {
    use glam::DVec3;

    use crate::prelude::*;

    use super::super::test::create_builder;

    #[test]
    fn builder_walls() {
        let mut builder = create_builder();

        {
            let mut building_builder = builder.new_building("test");

            let material_layer = building_builder.material_layer("ExampleMaterial", 0.02, false);
            let material_layer_set = building_builder.material_layer_set([material_layer]);
            let material_layer_set_usage = building_builder.material_layer_set_usage(
                material_layer_set,
                LayerSetDirectionEnum::Axis2,
                DirectionSenseEnum::Positive,
                0.0,
            );

            let wall_type = building_builder.wall_type(
                material_layer_set,
                "ExampleWallType",
                WallTypeEnum::NotDefined,
            );

            building_builder.vertical_wall(
                material_layer_set_usage,
                wall_type,
                "ExampleWallDefault",
                VerticalWallParameter {
                    height: 2.0,
                    length: 4.0,
                    placement: DVec3::new(0.0, 0.0, 0.0),
                },
            );
        }

        let s = builder.build();
        let ifc = IFC::from_str(&s).unwrap();

        assert_eq!(s, ifc.to_string());
    }
}