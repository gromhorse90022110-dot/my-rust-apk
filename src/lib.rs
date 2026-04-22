use ndk_glue;

#[no_mangle]
#[ndk_glue::main]
fn main() {
    loop {
        if let Some(_event) = ndk_glue::poll_events() {
            // Приложение работает
        }
    }
}

