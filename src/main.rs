cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        use linux as system;
    }
    else {
        mod unknown;
        use unknows as system;
    }
}


fn main() {
    let _guard = sentry::init((
        "SENTRY_",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        }
    ));
}
