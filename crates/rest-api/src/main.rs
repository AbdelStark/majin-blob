use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let api = filters::blob();
    let routes = api.with(warp::log("blob"));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

mod filters {
    use super::handlers;
    use warp::Filter;

    pub fn blob() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        blob_recover()
    }

    pub fn blob_recover(
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("blob")
            .and(warp::post())
            .and(warp::body::bytes())
            .and_then(handlers::blob_recover)
    }
}

pub mod handlers {
    use majin_blob_core::blob;
    use majin_blob_types::serde;
    use std::convert::Infallible;
    use warp::hyper::body::Bytes;

    pub async fn blob_recover(data: Bytes) -> Result<impl warp::Reply, Infallible> {
        let data = String::from_utf8(data.to_vec()).unwrap();
        let blob_data = serde::parse_str_to_blob_data(data.as_str());
        let original_data = blob::recover(blob_data);
        let state_diffs = serde::parse_state_diffs(original_data.as_slice());
        let state_diffs_json = serde::to_json(state_diffs);
        Ok(state_diffs_json)
    }
}
