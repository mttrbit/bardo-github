/// Used to identify a new type used in a query pipeline. The types are
/// consistent between each one in terms of transforming one to another.
/// This helps reduce boiler plate code and makes it easy to exapnd and
/// maintain code in the future by simply adding a new field here if needed
macro_rules! new_type {
    ($($i: ident)*) => (
        $(
            pub struct $i<'g> {
                pub(crate) request: Result<RefCell<Request<Body>>>,
                pub(crate) core: &'g Rc<RefCell<Core>>,
                pub(crate) client: &'g Rc<Client<HttpsConnector>>,
                pub(crate) parameter: Option<String>,
            }
        )*
    );
}
