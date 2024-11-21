use futures::{future::FusedFuture, FutureExt};

/// A test that frame can propagate FusedFuture.
mod util;

#[test]
fn consolidate() {
    util::model(|| util::run(fused_future()));
}

#[async_backtrace::framed]
fn fused_future() -> impl FusedFuture<Output = ()> {
    async_backtrace::location!().frame(ready().fuse())
}

#[async_backtrace::framed]
async fn ready() {
    let dump = async_backtrace::taskdump_tree(true);

    pretty_assertions::assert_str_eq!(
        util::strip(dump),
        "\
╼ fused::fused_future at backtrace/tests/fused.rs:LINE:COL
  └╼ fused::ready::{{closure}} at backtrace/tests/fused.rs:LINE:COL"
    );
}
