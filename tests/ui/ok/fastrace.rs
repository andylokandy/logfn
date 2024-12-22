#[logcall::logcall]
#[fastrace::trace]
fn f() {}

#[logcall::logcall]
#[fastrace::trace]
async fn g() {
    std::future::ready(1).await;
}

fn main() {
    f();
    pollster::block_on(g());
}
