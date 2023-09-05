use tide::Request;

use crate::utils;
use crate::State;

pub(crate) async fn get(req: Request<State>) -> tide::Result {
    Ok(utils::response::redirect(req.state(), "account/manage"))
}
