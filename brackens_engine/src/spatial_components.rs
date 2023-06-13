//===============================================================

use brackens_tools::{
    general,
    glam::{Mat4, Quat, Vec3},
};
use shipyard::{Component, EntitiesViewMut, EntityId, Get, Remove, ViewMut};

//===============================================================

#[derive(Component, Default, Clone, Copy)]
#[track(All)]
pub struct Transform(pub(crate) general::Transform);
impl Transform {
    //--------------------------------------------------

    #[inline]
    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self(general::Transform {
            translation,
            rotation,
            scale,
        })
    }

    #[inline]
    pub fn from_translation(translation: Vec3) -> Self {
        Self(general::Transform::from_translation(translation))
    }

    #[inline]
    pub fn from_rotation(rotation: Quat) -> Self {
        Self(general::Transform::from_rotation(rotation))
    }

    #[inline]
    pub fn from_scale(scale: Vec3) -> Self {
        Self(general::Transform::from_scale(scale))
    }

    #[inline]
    pub fn from_translation_rotation(translation: Vec3, rotation: Quat) -> Self {
        Self(general::Transform::from_translation_rotatation(
            translation,
            rotation,
        ))
    }
    #[inline]
    pub fn from_translation_scale(translation: Vec3, scale: Vec3) -> Self {
        Self(general::Transform::from_translation_scale(
            translation,
            scale,
        ))
    }

    #[inline]
    pub fn from_rotation_scale(rotation: Quat, scale: Vec3) -> Self {
        Self(general::Transform::from_rotation_scale(rotation, scale))
    }

    #[inline]
    pub fn from_translation_rotation_scale(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self(general::Transform::from_translation_rotatation_scale(
            translation,
            rotation,
            scale,
        ))
    }

    //--------------------------------------------------

    #[inline]
    pub fn translation(&self) -> &Vec3 {
        &self.0.translation
    }
    #[inline]
    pub fn rotation(&self) -> &Quat {
        &self.0.rotation
    }
    #[inline]
    pub fn scale(&self) -> &Vec3 {
        &self.0.scale
    }

    #[inline]
    pub fn translation_mut(&mut self) -> &mut Vec3 {
        &mut self.0.translation
    }
    #[inline]
    pub fn rotation_mut(&mut self) -> &mut Quat {
        &mut self.0.rotation
    }
    #[inline]
    pub fn scale_mut(&mut self) -> &mut Vec3 {
        &mut self.0.scale
    }

    //--------------------------------------------------

    #[inline]
    pub fn forward(&self) -> Vec3 {
        self.0.forward()
    }

    #[inline]
    pub fn right(&self) -> Vec3 {
        self.0.right()
    }

    //--------------------------------------------------

    #[inline]
    pub fn lerp(&mut self, target: &Transform, s: f32) {
        self.0.lerp(&target.0, s);
    }

    //--------------------------------------------------

    #[inline]
    pub fn to_raw(&self) -> [f32; 16] {
        self.0.to_raw()
    }

    #[inline]
    pub fn to_mat4(&self) -> Mat4 {
        self.0.to_mat4()
    }

    //--------------------------------------------------
}
impl std::ops::Add for Transform {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Transform(self.0 + rhs.0)
    }
}
impl std::ops::Add<&Self> for Transform {
    type Output = Self;

    #[inline]
    fn add(self, rhs: &Self) -> Self::Output {
        Transform(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for Transform {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

#[derive(Component, Default, Clone, Copy)]
#[track(All)]
pub struct GlobalTransform(pub(crate) Transform);
impl GlobalTransform {
    //--------------------------------------------------

    #[inline]
    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self(Transform::new(translation, rotation, scale))
    }

    #[inline]
    pub fn from_translation(translation: Vec3) -> Self {
        Self(Transform::from_translation(translation))
    }

    #[inline]
    pub fn from_rotation(rotation: Quat) -> Self {
        Self(Transform::from_rotation(rotation))
    }

    #[inline]
    pub fn from_scale(scale: Vec3) -> Self {
        Self(Transform::from_scale(scale))
    }

    #[inline]
    pub fn from_translation_rotation(translation: Vec3, rotation: Quat) -> Self {
        Self(Transform::from_translation_rotation(translation, rotation))
    }

    #[inline]
    pub fn from_translation_scale(translation: Vec3, scale: Vec3) -> Self {
        Self(Transform::from_translation_scale(translation, scale))
    }

    #[inline]
    pub fn from_rotation_scale(rotation: Quat, scale: Vec3) -> Self {
        Self(Transform::from_rotation_scale(rotation, scale))
    }

    #[inline]
    pub fn from_translation_rotation_scale(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self(Transform::from_translation_rotation_scale(
            translation,
            rotation,
            scale,
        ))
    }

    //--------------------------------------------------

    #[inline]
    pub fn translation(&self) -> &Vec3 {
        self.0.translation()
    }
    #[inline]
    pub fn rotation(&self) -> &Quat {
        self.0.rotation()
    }
    #[inline]
    pub fn scale(&self) -> &Vec3 {
        self.0.scale()
    }

    #[inline]
    pub fn translation_mut(&mut self) -> &mut Vec3 {
        self.0.translation_mut()
    }
    #[inline]
    pub fn rotation_mut(&mut self) -> &mut Quat {
        self.0.rotation_mut()
    }
    #[inline]
    pub fn scale_mut(&mut self) -> &mut Vec3 {
        self.0.scale_mut()
    }

    //--------------------------------------------------

    #[inline]
    pub fn forward(&self) -> Vec3 {
        self.0.forward()
    }

    #[inline]
    pub fn right(&self) -> Vec3 {
        self.0.right()
    }

    //--------------------------------------------------

    #[inline]
    pub fn lerp(&mut self, target: &GlobalTransform, s: f32) {
        self.0.lerp(&target.0, s);
    }

    //--------------------------------------------------

    #[inline]
    pub fn to_raw(&self) -> [f32; 16] {
        self.0.to_raw()
    }

    #[inline]
    pub fn to_mat4(&self) -> Mat4 {
        self.0.to_mat4()
    }

    //--------------------------------------------------
}

impl std::ops::Add for GlobalTransform {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        GlobalTransform(self.0 + rhs.0)
    }
}
impl std::ops::Add<&Self> for GlobalTransform {
    type Output = Self;

    #[inline]
    fn add(self, rhs: &Self) -> Self::Output {
        GlobalTransform(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for GlobalTransform {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

//===============================================================

#[derive(Component)]
#[track(All)]
pub struct UseParentTransform;

#[derive(Component)]
pub struct UpdateGlobalTransform;

#[derive(Component)]
pub(crate) struct TransformModified(pub(crate) EntityId, pub(crate) Option<GlobalTransform>);

//===============================================================

const TRANSFORM_DIRTY_SET: u32 = 255;

#[derive(Component, PartialEq, Eq)]
pub struct TransformDirty(pub(crate) u8, pub(crate) u8, pub(crate) Option<EntityId>);
impl TransformDirty {
    #[inline]
    fn get_value(&self) -> u32 {
        (TRANSFORM_DIRTY_SET * self.1 as u32) + self.0 as u32
    }
}

impl PartialOrd for TransformDirty {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_value().partial_cmp(&other.get_value())
    }
}

impl Ord for TransformDirty {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_value().cmp(&other.get_value())
    }
}

//===============================================================
// Heirarchy stuff starts here

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
}
impl Child {
    #[inline]
    pub fn parent(&self) -> EntityId {
        self.parent
    }
}

#[derive(Component)]
pub struct ParentRoot;

//===============================================================

pub struct ChildrenIter<C> {
    get_child: C,
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
            let ret = self.cursor.0;
            self.cursor.0 = self.get_child.get(self.cursor.0).unwrap().next;
            Some(ret)
        } else {
            None
        }
    }
}

pub struct AncestorIter<C> {
    get_child: C,
    cursor: EntityId,
}
impl<'a, C> Iterator for AncestorIter<C>
where
    C: Get<Out = &'a Child> + Copy,
{
    type Item = EntityId;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_child.get(self.cursor).ok().map(|child| {
            self.cursor = child.parent;
            child.parent
        })
    }
}

pub struct DescendantsIter<P, C> {
    get_parent: P,
    get_child: C,
    cursors: Vec<(EntityId, usize, EntityId)>,
}
impl<'a, P, C> Iterator for DescendantsIter<P, C>
where
    P: Get<Out = &'a Parent> + Copy,
    C: Get<Out = &'a Child> + Copy,
{
    // First EntityId is the child id
    // Second EntityId is the childs parent id
    // usize is how far down the tree we are
    type Item = (EntityId, EntityId, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cursor) = self.cursors.last_mut() {
            if cursor.1 > 0 {
                cursor.1 -= 1;
                let ret = cursor.0;
                cursor.0 = self.get_child.get(cursor.0).unwrap().next;
                let parent_id = cursor.2;
                if let Ok(parent) = self.get_parent.get(ret) {
                    self.cursors
                        .push((parent.first_child, parent.child_count, ret));
                }
                Some((ret, parent_id, self.cursors.len()))
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
        let (_, children) = self;

        AncestorIter {
            get_child: *children,
            cursor: id,
        }
    }

    fn children(&self, id: EntityId) -> ChildrenIter<C> {
        let (parents, children) = self;

        ChildrenIter {
            get_child: *children,
            cursor: parents
                .get(id)
                .map_or((id, 0), |parent| (parent.first_child, parent.child_count)),
        }
    }

    fn descendants(&self, id: EntityId) -> DescendantsIter<P, C> {
        let (parents, children) = self;

        DescendantsIter {
            get_parent: *parents,
            get_child: *children,
            cursors: parents.get(id).map_or_else(
                |_| Vec::new(),
                |parent| vec![(parent.first_child, parent.child_count, id)],
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
    /// Attaches an entity as a child to a given parent entity.
    fn attach(&mut self, entities: &mut EntitiesViewMut, parent_id: EntityId, child_id: EntityId);

    /// Creates a new entity and attaches it to the given parent.
    fn attach_new(&mut self, entities: &mut EntitiesViewMut, parent_id: EntityId) -> EntityId;

    /// Removes the child status of an entity
    fn detach(&mut self, entities: &mut EntitiesViewMut, child_id: EntityId);

    /// Remove an entity from the hierarchy completely
    fn remove_all(&mut self, entities: &mut EntitiesViewMut, entity_id: EntityId);

    // Remove an entity and all its children from the hierarchy completely
    fn remove_all_children(&mut self, entities: &mut EntitiesViewMut, entity_id: EntityId);
}

impl<'a> HierarchyBundleTools for HierarchyBundle<'a> {
    fn attach(&mut self, entities: &mut EntitiesViewMut, parent_id: EntityId, child_id: EntityId) {
        // Make sure new child doesn't already have a parent
        self.detach(entities, child_id);

        let (parents, children, roots) = self;

        // If the child was a root before, it will no longer be now
        roots.remove(child_id);

        // If the parent is not a child, it will now be a root
        if !children.contains(parent_id) {
            entities.add_component(parent_id, roots, ParentRoot);
        }

        // Check to see if the parent we're attaching to is already a parent
        match parents.get(parent_id) {
            // The parent is already a parent and therefore has at least one
            // child already
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
                    },
                );
            }

            // The parent is not a parent already. We can add the respective
            // components to each entity without changing anything else.
            // The child will link to itself for now
            Err(_) => {
                entities.add_component(
                    child_id,
                    children,
                    Child {
                        parent: parent_id,
                        prev: child_id,
                        next: child_id,
                    },
                );
                entities.add_component(
                    parent_id,
                    parents,
                    Parent {
                        child_count: 1,
                        first_child: child_id,
                    },
                )
            }
        }
    }

    fn attach_new(&mut self, entities: &mut EntitiesViewMut, parent_id: EntityId) -> EntityId {
        let child_id = entities.add_entity((), ());
        self.attach(entities, child_id, parent_id);
        child_id
    }

    // Remove an entity from a parent. Remove its child component
    // and update the parent entity.
    fn detach(&mut self, entities: &mut EntitiesViewMut, child_id: EntityId) {
        let (parents, children, roots) = self;
        // Remove the child component if exists.
        if let Some(child) = children.remove(child_id) {
            // Retrieve and update parent component from ancestor
            let parent = &mut parents[child.parent];
            parent.child_count -= 1;

            // The parent has no children and is no longer a parent. Remove
            // its parent and potential root component
            if parent.child_count == 0 {
                parents.remove(child.parent);
                roots.remove(child.parent);
            }
            // The parent still has children and those children need
            // their links to be changed to no longer include the
            // removed entitiy
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
        }
    }

    fn remove_all(&mut self, entities: &mut EntitiesViewMut, entity_id: EntityId) {
        // Remove it's child component
        self.detach(entities, entity_id);

        // Get a vector of the entities children
        let children = (&self.0, &self.1).children(entity_id).collect::<Vec<_>>();
        for child_id in children {
            self.detach(entities, child_id);
        }
        // Remove it's parent component
        self.1.remove(entity_id);
    }

    fn remove_all_children(&mut self, entities: &mut EntitiesViewMut, entity_id: EntityId) {
        let (parents, children, _) = self;
        for child_id in (&*parents, &*children)
            .children(entity_id)
            .collect::<Vec<_>>()
        {
            self.remove_all_children(entities, child_id);
        }
        self.remove_all(entities, entity_id);
    }
}

//===============================================================
//===============================================================
