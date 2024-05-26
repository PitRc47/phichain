use crate::timing::{BpmList, BpmPoint};
use bevy::prelude::World;
use undo::Edit;

#[derive(Debug, Copy, Clone)]
pub struct CreateBpmPoint(BpmPoint);

impl CreateBpmPoint {
    pub fn new(point: BpmPoint) -> Self {
        Self(point)
    }
}

impl Edit for CreateBpmPoint {
    type Target = World;
    type Output = ();

    fn edit(&mut self, target: &mut Self::Target) -> Self::Output {
        let mut bpm_list = target.resource_mut::<BpmList>();
        bpm_list.0.push(self.0);
        bpm_list.compute();
    }

    fn undo(&mut self, target: &mut Self::Target) -> Self::Output {
        let mut bpm_list = target.resource_mut::<BpmList>();
        bpm_list.0.pop();
        bpm_list.compute();
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RemoveBpmPoint {
    index: usize,
    point: Option<BpmPoint>,
}

impl RemoveBpmPoint {
    pub fn new(index: usize) -> Self {
        Self { index, point: None }
    }
}

impl Edit for RemoveBpmPoint {
    type Target = World;
    type Output = ();

    fn edit(&mut self, target: &mut Self::Target) -> Self::Output {
        let mut bpm_list = target.resource_mut::<BpmList>();
        self.point = Some(bpm_list.0.remove(self.index));
        bpm_list.compute();
    }

    fn undo(&mut self, target: &mut Self::Target) -> Self::Output {
        let mut bpm_list = target.resource_mut::<BpmList>();
        bpm_list.0.insert(self.index, self.point.take().unwrap());
        bpm_list.compute();
    }
}

#[derive(Debug, Copy, Clone)]
pub struct EditBpmPoint {
    index: usize,
    from: BpmPoint,
    to: BpmPoint,
}

impl EditBpmPoint {
    pub fn new(index: usize, from: BpmPoint, to: BpmPoint) -> Self {
        Self { index, from, to }
    }
}

impl Edit for EditBpmPoint {
    type Target = World;
    type Output = ();

    fn edit(&mut self, target: &mut Self::Target) -> Self::Output {
        let mut bpm_list = target.resource_mut::<BpmList>();
        bpm_list.0[self.index] = self.to;
    }

    fn undo(&mut self, target: &mut Self::Target) -> Self::Output {
        let mut bpm_list = target.resource_mut::<BpmList>();
        bpm_list.0[self.index] = self.from;
    }
}