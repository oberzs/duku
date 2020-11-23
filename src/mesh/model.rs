// Oliver Berzs
// https://github.com/oberzs/duku

use std::slice::Iter;

use super::Mesh;
use crate::math::Matrix4;
use crate::pipeline::Material;
use crate::resources::Handle;

pub struct Model {
    pub nodes: Vec<ModelNode>,
}

pub struct ModelNode {
    pub meshes: Vec<Handle<Mesh>>,
    pub materials: Vec<Handle<Material>>,
    pub matrix: Matrix4,
    pub children: Vec<Self>,
}

struct ChildIter<'a> {
    stack: Vec<Iter<'a, ModelNode>>,
}

impl Model {
    pub fn meshes(&self) -> impl Iterator<Item = &Handle<Mesh>> {
        self.nodes.iter().map(|node| node.meshes()).flatten()
    }

    pub fn materials(&self) -> impl Iterator<Item = &Handle<Material>> {
        self.nodes.iter().map(|node| node.materials()).flatten()
    }
}

impl ModelNode {
    pub(crate) fn orders(&self) -> impl Iterator<Item = (&Handle<Mesh>, &Handle<Material>)> {
        self.meshes.iter().zip(self.materials.iter())
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
