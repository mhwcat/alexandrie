use diesel::prelude::*;
use json::json;
use rocket::State;
use rocket_contrib::templates::Template;

use crate::db::models::CrateRegistration;
use crate::db::schema::*;
use crate::db::DbConn;
use crate::error::Error;
use crate::frontend::config::Config;

#[get("/")]
pub(crate) fn route(config: State<Config>, conn: DbConn) -> Result<Template, Error> {
    let crate_count = crates::table
        .select(diesel::dsl::count(crates::id))
        .first::<i64>(&conn.0)?;
    let total_downloads = crates::table
        .select(diesel::dsl::count(crates::downloads))
        .first::<i64>(&conn.0)?;
    let most_downloaded = crates::table
        .order_by(crates::downloads.desc())
        .limit(10)
        .load::<CrateRegistration>(&conn.0)?;
    let last_updated = crates::table
        .order_by(crates::updated_at.desc())
        .limit(10)
        .load::<CrateRegistration>(&conn.0)?;
    // let (total_downloads, crate_count) = (1_234_423_423, 414_324);
    Ok(Template::render(
        "index",
        json!({
            "instance": config.inner(),
            "total_downloads": total_downloads,
            "crate_count": crate_count,
            "most_downloaded": most_downloaded,
            "last_updated": last_updated,
        }),
    ))
}
