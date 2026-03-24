use utoipa::OpenApi;

use crate::models::creator::{CreateCreatorRequest, CreatorResponse};
use crate::models::tip::{RecordTipRequest, TipResponse};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Stellar Tipjar API",
        version = "0.1.0",
        description = "Backend API for the Stellar Tipjar — create creator profiles and record on-chain tips verified against the Stellar network."
    ),
    paths(
        crate::routes::creators::create_creator,
        crate::routes::creators::get_creator,
        crate::routes::creators::get_creator_tips,
        crate::routes::tips::record_tip,
    ),
    components(
        schemas(
            CreateCreatorRequest,
            CreatorResponse,
            RecordTipRequest,
            TipResponse,
        )
    ),
    tags(
        (name = "creators", description = "Creator profile management"),
        (name = "tips", description = "Tip recording and retrieval")
    )
)]
pub struct ApiDoc;
