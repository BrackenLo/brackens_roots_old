//===============================================================

use shipyard::{Component, EntitiesViewMut, EntityId, Get, Remove, ViewMut};

//===============================================================

pub type HierarchyDepth = u8;

//===============================================================

#[derive(Component)]
#[track(All)]
pub struct UseParentTransform;

//===============================================================

#[derive(Component)]
#[track(All)]
pub struct Parent {
    child_count: usize,
    first_child: EntityId,
}

#[derive(Component)]
#[track(All)]
pub struct Child {
    parent: EntityId,
    prev: EntityId,
    next: EntityId,
    depth: HierarchyDepth,
}
impl Child {
    pub fn parent(&self) -> EntityId {
        self.parent
    }
}

#[derive(Component)]
pub struct ParentRoot;

//===============================================================

pub struct ChildrenIter<C> {
    children_view: C,
    cursor: (EntityId, usize),
}
impl<'a, C> Iterator for ChildrenIter<C>
where
    C: Get<Out = &'a Child> + Copy,
{
    type Item = EntityId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.1 > 0 {
            self.cursor.1 -= 1;
            let to_return = self.cursor.0;
            self.cursor.0 = self.children_view.get(self.cursor.0).unwrap().next;
            Some(to_return)
        } else {
            None
        }
    }
}

//--------------------------------------------------

pub struct AncestorIter<C> {
    children_view: C,
    cursor: EntityId,
}
impl<'a, C> Iterator for AncestorIter<C>
where
    C: Get<Out = &'a Child> + Copy,
{
    // Parent Id and depth
    type Item = (EntityId, HierarchyDepth);

    fn next(&mut self) -> Option<Self::Item> {
        self.children_view.get(self.cursor).ok().map(|child| {
            self.cursor = child.parent;
            (child.parent, child.depth)
        })
    }
}

//--------------------------------------------------

pub struct DescendantsResult {
    pub child_id: EntityId,
    pub parent_id: EntityId,
    pub depth: HierarchyDepth,
}

struct DescendantsCursor {
    child_id: EntityId,
    index: usize,
    parent_id: EntityId,
}

pub struct DescendantsIter<P, C> {
    parent_view: P,
    child_view: C,
    start_depth: HierarchyDepth,
    cursors: Vec<DescendantsCursor>,
}
impl<'a, P, C> Iterator for DescendantsIter<P, C>
where
    P: Get<Out = &'a Parent> + Copy,
    C: Get<Out = &'a Child> + Copy,
{
    type Item = DescendantsResult;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cursor) = self.cursors.last_mut() {
            if cursor.index > 0 {
                // Reduce children to process count by 1
                cursor.index -= 1;
                // Store child id to return later
                let to_return = cursor.child_id;
                // Set new child id
                cursor.child_id = self.child_view.get(cursor.child_id).unwrap().next;
                // Store parent id to return later
                let parent_id = cursor.parent_id;
                // Check to see if our child is a parent. If so, add it's children as a cursor to the cursors
                if let Ok(parent) = self.parent_view.get(to_return) {
                    self.cursors.push(DescendantsCursor {
                        child_id: parent.first_child,
                        index: parent.child_count,
                        parent_id: to_return,
                    })
                }

                Some(DescendantsResult {
                    child_id: to_return,
                    parent_id,
                    depth: self.cursors.len() as HierarchyDepth + self.start_depth,
                })
            } else {
                self.cursors.pop();
                self.next()
            }
        } else {
            None
        }
    }
}

//===============================================================

/// Trait for implementing functions to scan parent and child views
pub trait HierarchyIter<'a, P, C> {
    /// Get the parents of an entity
    fn ancestors(&self, id: EntityId) -> AncestorIter<C>;
    /// Get the children of an entity
    fn children(&self, id: EntityId) -> ChildrenIter<C>;
    /// Get all children that branch from this entity
    fn descendants(&self, id: EntityId) -> DescendantsIter<P, C>;
}
impl<'a, P, C> HierarchyIter<'a, P, C> for (P, C)
where
    P: Get<Out = &'a Parent> + Copy,
    C: Get<Out = &'a Child> + Copy,
{
    fn ancestors(&self, id: EntityId) -> AncestorIter<C> {
        AncestorIter {
            children_view: self.1,
            cursor: id,
        }
    }

    fn children(&self, id: EntityId) -> ChildrenIter<C> {
        let (parents, children) = self;

        ChildrenIter {
            children_view: *children,
            cursor: parents
                .get(id)
                .map_or((id, 0), |parent| (parent.first_child, parent.child_count)),
        }
    }

    fn descendants(&self, id: EntityId) -> DescendantsIter<P, C> {
        let (parents, children) = self;

        let start_depth = children.get(id).map_or_else(|_| 0, |child| child.depth);

        DescendantsIter {
            parent_view: *parents,
            child_view: *children,
            start_depth,
            cursors: parents.get(id).map_or_else(
                |_| Vec::new(),
                |parent| {
                    vec![DescendantsCursor {
                        child_id: parent.first_child,
                        index: parent.child_count,
                        parent_id: id,
                    }]
                },
            ),
        }
    }
}

//===============================================================

pub type HierarchyBundle<'a> = (
    ViewMut<'a, Parent>,
    ViewMut<'a, Child>,
    ViewMut<'a, ParentRoot>,
);

pub trait HierarchyBundleTools {
    /// Attaches an entity as a child to a given parent entity
    fn attach(&mut self, entities: &mut EntitiesViewMut, parent_id: EntityId, child_id: EntityId);
    /// Creates a new entitiy and attaches it to the given parent.
    fn attach_new(&mut self, entities: &mut EntitiesViewMut, parent_id: EntityId) -> EntityId;

    /// Removes the child status of an entity
    fn detach(&mut self, entities: &mut EntitiesViewMut, child_id: EntityId);

    /// Remove an entitiy from the hierarchy completely (parent and child)
    fn remove_all(&mut self, entities: &mut EntitiesViewMut, entity_id: EntityId);
    /// Remove an entity and all its children from the hierarchy completely
    fn remove_all_children(&mut self, entities: &mut EntitiesViewMut, entity_id: EntityId);

    fn calculate_depth(&mut self, entity_id: EntityId);
}

impl<'a> HierarchyBundleTools for HierarchyBundle<'a> {
    fn attach(&mut self, entities: &mut EntitiesViewMut, parent_id: EntityId, child_id: EntityId) {
        // Make sure the new child doesn't already have a parent
        self.detach(entities, child_id);

        let (parents, children, roots) = self;

        // If the child was a root before, it will no longer be now
        roots.remove(child_id);

        // Check if the parent is a child and get its (old/new) depth. Also, if the parent is now a
        // child, it will now be a root;
        let parent_depth = match children.get(parent_id) {
            Ok(parent) => parent.depth,
            Err(_) => {
                entities.add_component(parent_id, roots, ParentRoot);
                0
            }
        };

        // Check to see if the parent we're attaching to is already a parent
        match parents.get(parent_id) {
            // The parent is already a parent and therefore has at least one child already
            Ok(mut parent) => {
                parent.child_count += 1;

                // Get the ids of the previous and next siblings of the new child
                let prev = children[parent.first_child].prev;
                let next = parent.first_child;

                // Change the linking
                children[prev].next = child_id;
                children[next].prev = child_id;

                // Add the child component to the new entity
                entities.add_component(
                    child_id,
                    children,
                    Child {
                        parent: parent_id,
                        prev,
                        next,
                        depth: parent_depth + 1,
                    },
                )
            }

            // The parent is not a parent already. We can add the respective components to each
            // entity without changing anything else. The child will link to itself for now.
            Err(_) => entities.add_component(
                child_id,
                children,
                Child {
                    parent: parent_id,
                    prev: child_id,
                    next: child_id,
                    depth: parent_depth + 1,
                },
            ),
        }

        self.calculate_depth(child_id);
    }

    fn attach_new(&mut self, entities: &mut EntitiesViewMut, parent_id: EntityId) -> EntityId {
        let child_id = entities.add_entity((), ());
        self.attach(entities, child_id, parent_id);
        child_id
    }

    fn detach(&mut self, entities: &mut EntitiesViewMut, child_id: EntityId) {
        let (parents, children, roots) = self;
        // Remove the child component if it exists.
        if let Some(child) = children.remove(child_id) {
            // Retrieve and update parent component from ancestor
            let parent = &mut parents[child.parent];
            parent.child_count -= 1;

            // The parent has now children and is now longer a parnet. Remove its parent and
            // potential root component
            if parent.child_count == 0 {
                parents.remove(child.parent);
                roots.remove(child.parent);
            }
            // The parent still has children and those children need their links to be changed to
            // no longer include the removed entity
            else {
                if parent.first_child == child_id {
                    parent.first_child = child.next;
                }

                children[child.prev].next = child.next;
                children[child.next].prev = child.prev;
            }

            // If the removed entity is a parent, it is now a root node
            if parents.contains(child_id) {
                entities.add_component(child_id, roots, ParentRoot);
            }

            self.calculate_depth(child_id);
        }
    }

    fn remove_all(&mut self, entities: &mut EntitiesViewMut, entity_id: EntityId) {
        // Remove the entities child component
        self.detach(entities, entity_id);

        // Get a vector of the entities children and detach each of them from the parent
        let children = (&self.0, &self.1).children(entity_id).collect::<Vec<_>>();
        children
            .into_iter()
            .for_each(|child_id| self.detach(entities, child_id));

        // Remove it's parent component
        self.1.remove(entity_id);
    }

    fn remove_all_children(&mut self, entities: &mut EntitiesViewMut, entity_id: EntityId) {
        let (parents, children, _) = self;

        (&*parents, &*children)
            .children(entity_id)
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|child_id| self.remove_all_children(entities, child_id));

        self.remove_all(entities, entity_id);
    }

    fn calculate_depth(&mut self, entity_id: EntityId) {
        let (parents, children, _) = self;

        let depth = match children.get(entity_id) {
            Ok(child) => child.depth,
            Err(_) => 0,
        };

        (&*parents, &*children)
            .children(entity_id)
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|child_id| {
                calculate_depth_inner(parents, children, child_id, depth);
            });
    }
}

fn calculate_depth_inner<'a>(
    parents: &mut ViewMut<'a, Parent>,
    children: &mut ViewMut<'a, Child>,
    entity_id: EntityId,
    mut depth: HierarchyDepth,
) {
    depth += 1;

    if let Ok(mut child) = children.get(entity_id) {
        child.depth = depth;
    }

    (&*parents, &*children)
        .children(entity_id)
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|child_id| {
            calculate_depth_inner(parents, children, child_id, depth);
        });
}

//===============================================================
