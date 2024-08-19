pub trait AppEntity{
    const RESOURCE:&'static str;
}
pub trait ToResult{
    type Output:AppEntity;
    fn to_result(self)->result::Result<Self::Output>;
    fn check_absents(self)->result::Result<Self::Output>;


}

impl<T> ToResult for Option<T>
where
    T:AppEntity
{
    type Output = T;
    fn to_result(self) -> result::Result<Self::Output> {
        self.ok_or_else(||{
            result::Error::NotFound(format!("{}",Self::Output::RESOURCE))
        })
    }
    fn check_absents(self) -> result::Result<Self::Output> {
        self.ok_or_else(||{
            result::Error::ResourceExist(format!("{} IS AlREADY EXISTS",Self::Output::RESOURCE))
        })
    }
}