#[macro_export]
macro_rules! user_api {
    () => {
        warp::path("root")
            .and(routes::user::get_by_cookie())
            .and_then(handlers::user::profile)
            .or(
                // login, give cookie and direct to profile
                routes::user::login()
                .and_then(handlers::user::profile_with_cookie)
                .recover(handlers::user::login_error)
            )
            .or(
                // signup, give cookie and direct to profile
                routes::user::signup()
                .and_then(handlers::user::profile_with_cookie)
                .recover(handlers::user::signup_error)
            )
            .or(
                // logout, send to landing page
                routes::user::logout()
                .and_then(handlers::user::logout)
                // .recover(handlers::user::handle_logout_errors)
            )
            .or(
                // redirect signup page to profile if signed in
                routes::user::signup_form()
                .and(routes::user::authenticate_cookie())
                .and_then(routes::workspace::with_root_workspace)
                .untuple_one()
                .and_then(handlers::user::profile))
            .or(
                // signup form
                routes::user::signup_form()
                .and_then(handlers::user::signup_form))
            .or(
                // style form
                routes::user::update_prelude()
                .and_then(handlers::user::edit_prelude))
            .or(
                // prelude form
                routes::user::update_style()
                .and_then(handlers::user::edit_style))
            .or(
                // redirect login page to profile if signed in
                routes::user::login_form()
                .and(routes::user::authenticate_cookie())
                .and_then(routes::workspace::with_root_workspace)
                .untuple_one()
                .and_then(handlers::user::profile))
            .or(
                // login form
                routes::user::login_form()
                .and_then(handlers::user::login_form)
            )
        .with(warp::trace::named("user"))
    };
}
