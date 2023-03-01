pub fn initialize() -> Result<(), ()> {
    super::exceptions::initialize_exceptions()?;
    super::paging::initialize_paging()?;

    Ok(())
}
