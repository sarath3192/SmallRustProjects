trait PackageState {
    fn audit(&self) -> String;
}

macro_rules! impl_package_state{
    ($($state:ident),+) => {
        $(
            impl PackageState for $state {
                fn audit(&self) -> String{
                    format!("{self:?}")
                }
            }
        )+

    }

}

type Name =&'static str;
type Address =&'static str;
type Date = &'static str;

#[derive(Debug)]
struct Queued;

#[derive(Debug)]
struct Picking
{
    picker: Name,
}

#[derive(Debug)]
struct Loading{

    truck_id: u32,
}

#[derive(Debug)]
struct OutForDelivery{
    driver: Name,
    address: Address,
    truck_id: u32,
}

#[derive(Debug)]
struct Delivered{
    date: Date,
}

#[derive(Debug)]
struct Finalized;

impl impl_package_state {
    Queued,
    Picking,
    Loading,
    OutForDelivery,
    Delivered,
    Finalized,
}