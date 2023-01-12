use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::MeshVertexAttribute;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_resource::VertexFormat;
use bevy::sprite::Mesh2dHandle;
use crate::ColoredMesh2dBundle;

#[derive(SystemParam)]
pub struct ColoredMesh2dBuilder<'w, 's> {
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub commands: Commands<'w, 's>,
}

#[derive(Clone, Default)]
pub struct ColoredMesh2dVertexBuffer {
    pub vertices: Vec<[f32; 3]>,
    pub colors: Vec<u32>,
    pub indices: Vec<u32>,
    pub index: u32,
}

impl ColoredMesh2dVertexBuffer {
    pub fn with_capacity(n: usize) -> Self {
        Self { 
            vertices: Vec::with_capacity(n),
            colors: Vec::with_capacity(n),
            indices: Vec::with_capacity(n),
            index: 0
        }
    }

    pub fn push_indices<const N: usize>(&mut self, mut indices: [u32; N]) {
        for i in 0..N {
            indices[i] += self.index;
        }   
        self.indices.extend_from_slice(&indices);
    }
}

#[derive(Clone, Default)]
pub struct QuadBuffer {
    pub buffer: ColoredMesh2dVertexBuffer,
}

impl QuadBuffer {
    pub fn push_quad(
        &mut self,
        sw: Vec2,
        nw: Vec2,
        ne: Vec2,
        se: Vec2,
        z: f32,
        color: Color,
    ) {
        let buffer = &mut self.buffer;
        buffer.vertices.push([sw.x, sw.y, z]);
        buffer.vertices.push([nw.x, nw.y, z]);
        buffer.vertices.push([ne.x, ne.y, z]);
        buffer.vertices.push([se.x, se.y, z]);
        let color_u32 = color.as_rgba_u32();
        buffer.colors.extend_from_slice(&[color_u32; 4]);
        buffer.push_indices([0, 2, 1, 0, 3, 2]);
        buffer.index += 4;
    }

    pub fn push_rect(
        &mut self,
        min: Vec2,
        max: Vec2,
        z: f32,
        color: Color,
    ) {
        let buffer = &mut self.buffer;
        buffer.vertices.push([min.x, min.y, z]);
        buffer.vertices.push([min.x, max.y, z]);
        buffer.vertices.push([max.x, max.y, z]);
        buffer.vertices.push([max.x, min.y, z]);
        let color_u32 = color.as_rgba_u32();
        buffer.colors.extend_from_slice(&[color_u32; 4]);
        buffer.push_indices([0, 2, 1, 0, 3, 2]);
        buffer.index += 4;
    }
}

impl ColoredMesh2dBuilder<'_, '_> {
    pub fn make_mesh_bundle(
        &mut self,
        vertices: Vec<[f32; 3]>,
        colors: Vec<u32>,
        indices: Vec<u32>,
        topology: PrimitiveTopology,
        transform: Transform,
    ) -> ColoredMesh2dBundle {
        let mut mesh = Mesh::new(topology);
        // mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        // mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(
            MeshVertexAttribute::new("Vertex_Color", 1, VertexFormat::Uint32),
             colors
        );
        mesh.set_indices(Indices::U32(indices).into());
        let mesh_handle = self.meshes.add(mesh);
        ColoredMesh2dBundle {
            mesh_2d_handle: Mesh2dHandle(mesh_handle),
            transform,
            ..Default::default()
        }
    }

    pub fn spawn_mesh(
        &mut self,
        vertices: Vec<[f32; 3]>,
        colors: Vec<u32>,
        indices: Vec<u32>,
        topology: PrimitiveTopology,
        transform: Transform,
    ) -> (Entity, Handle<Mesh>) {
        let mesh_bundle = self.make_mesh_bundle(vertices, colors, indices, topology, transform);
        let mesh_handle = mesh_bundle.mesh_2d_handle.0.clone();
        let id = self.commands.spawn(mesh_bundle).id();
        (id, mesh_handle)
    }

    pub fn spawn_quads(
        &mut self,
        quads: QuadBuffer,
        transform: Transform,
    ) -> (Entity, Handle<Mesh>) {
        let mesh_bundle = self.make_mesh_bundle(quads.buffer.vertices, quads.buffer.colors, quads.buffer.indices, PrimitiveTopology::TriangleList, transform);
        let mesh_handle = mesh_bundle.mesh_2d_handle.0.clone();
        let id = self.commands.spawn(mesh_bundle).id();
        (id, mesh_handle)
    }
}