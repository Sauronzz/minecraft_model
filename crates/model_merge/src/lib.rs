pub trait ModelMerge {
    fn merge(&mut self, other: &Self);
}

impl ModelMerge for String {
    fn merge(&mut self, other: &Self) {
        
    }
}

impl ModelMerge for bool {
    fn merge(&mut self, other: &Self) {
        
    }
}

impl ModelMerge for f32 {
    fn merge(&mut self, other: &Self) {
        
    }
}

impl<T> ModelMerge for Vec<T>
    where T: ModelMerge + Clone
{
    fn merge(&mut self, other: &Self) {
        
    }
}