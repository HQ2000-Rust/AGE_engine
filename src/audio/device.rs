pub struct Device<T> {
    pub(super) device: rodio::Device,
    _marker: std::marker::PhantomData<T>,
}
