// Oliver Berzs
// https://github.com/oberzs/duku

use std::collections::HashSet;
use std::slice::Iter;

use super::Mesh;
use crate::math::Mat4;
use crate::pipeline::Descriptor;
use crate::pipeline::Material;
use crate::resources::Handle;

/// Collection of meshes and materials.
///
/// Makes it easier to render complex scenes
pub struct Model {
    /// render-nodes of the model
    pub nodes: Vec<ModelNode>,
}

/// One node of the model.
///
/// Represents an object in a scene or a
/// child object.
#[derive(Clone)]
pub struct ModelNode {
    /// meshes for this node
    pub meshes: Vec<Handle<Mesh>>,
    /// materials for this node
    pub materials: Vec<Handle<Material>>,
    /// transform in matrix form for this node
    pub matrix: Mat4,
    /// child nodes
    pub children: Vec<Self>,
}

struct ChildIter<'a> {
    stack: Vec<Iter<'a, ModelNode>>,
}

impl Model {
    /// fix the color space for materials, if the .gltf file
    /// was exported incorrectly
    pub fn fix_color_space(&mut self) {
        let mut fixed = HashSet::new();
        self.nodes
            .iter_mut()
            .for_each(|n| n.fix_color_space(&mut fixed));
    }

    /// iterate through all meshes in the model
    pub fn meshes(&self) -> impl Iterator<Item = &Handle<Mesh>> {
        self.nodes.iter().map(|node| node.meshes()).flatten()
    }

    /// iterate through all materials in the model
    pub fn materials(&self) -> impl Iterator<Item = &Handle<Material>> {
        self.nodes.iter().map(|node| node.materials()).flatten()
    }
}

impl ModelNode {
    pub(crate) fn orders(&self) -> impl Iterator<Item = (&Handle<Mesh>, &Handle<Material>)> {
        self.meshes.iter().zip(self.materials.iter())
    }

    fn fix_color_space(&mut self, fixed: &mut HashSet<Descriptor>) {
        for mat in &mut self.materials {
            let mut m = mat.write();
            if !fixed.contains(&m.descriptor()) {
                m.a[0] = to_linear(m.a[0]);
                m.a[1] = to_linear(m.a[1]);
                m.a[2] = to_linear(m.a[2]);
                fixed.insert(m.descriptor());
            }
        }

        self.children
            .iter_mut()
            .for_each(|c| c.fix_color_space(fixed));
    }

    fn meshes(&self) -> impl Iterator<Item = &Handle<Mesh>> {
        self.meshes
            .iter()
            .chain(self.child_iter().map(|node| node.meshes.iter()).flatten())
    }

    fn materials(&self) -> impl Iterator<Item = &Handle<Material>> {
        self.materials.iter().chain(
            self.child_iter()
                .map(|node| node.materials.iter())
                .flatten(),
        )
    }

    fn child_iter(&self) -> ChildIter<'_> {
        ChildIter {
            stack: vec![self.children.iter()],
        }
    }
}

impl<'a> Iterator for ChildIter<'a> {
    type Item = &'a ModelNode;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(mut top_iter) = self.stack.pop() {
                if let Some(node) = top_iter.next() {
                    // put iter back on stack
                    self.stack.push(top_iter);

                    // put node's children on stack
                    self.stack.push(node.children.iter());
                    return Some(&node);
                }
            } else {
                // stack emtpy
                return None;
            }
        }
    }
}

fn to_linear(value: f32) -> f32 {
    let s = clamp(value, 0.0, 1.0);
    let cutoff = 0.04045;
    let gamma = 2.2;

    if s <= cutoff {
        s / 12.92
    } else {
        ((s + 0.055) / 1.055).powf(gamma)
    }
}

fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
