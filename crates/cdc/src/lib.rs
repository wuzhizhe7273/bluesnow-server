pub struct CdcConfig{
    
}

pub trait Cdc{
    fn new(config:CdcConfig)->Self;
}