pub(super) trait Scenario {
    fn execute(&self) -> Option<()>;
    fn is_playing(&self) -> bool;
    fn name(&self) -> &'static str;
}
