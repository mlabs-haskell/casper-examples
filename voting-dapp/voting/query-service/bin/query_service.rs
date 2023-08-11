use actix_cors::Cors;
use actix_web::{error, get, web, App, HttpResponse, HttpServer, Responder};
use contracts::{deployed_contracts::DeployedGovernor, governor::GovernorDeployer};
// use log::{info, trace, warn};

use query_service::dto::{ProposalDTO, ProposalsDTO, Status};

#[get("/proposal/{proposal_id}")]
async fn get_proposal(
    pid: web::Path<u64>,
    data: web::Data<ClientState>,
) -> actix_web::Result<impl Responder> {
    let proposal_id = pid.into_inner();
    let result = web::block(move || {
        let mut gov = GovernorDeployer::register(data.governor.get_package_hash_address());

        gov.get_proposal(proposal_id)
    })
    .await
    .map_err(|_| error::ErrorInternalServerError("Failed to query proposal from chain"))?;

    Ok(HttpResponse::Ok().json(ProposalDTO::from(result)))
}

// Will fail with "not implemented error" if no proposals were created,
// coz there will be no corresponding Named Key in Contract context created yet, I suppose
#[get("/proposals")]
async fn all_proposals(data: web::Data<ClientState>) -> actix_web::Result<impl Responder> {
    let result = web::block(move || {
        let mut gov = GovernorDeployer::register(data.governor.get_package_hash_address());

        let num_of_proposals = gov.last_proposal_id();
        let mut proposals = ProposalsDTO::empty();
        for n in 0..=num_of_proposals {
            proposals.add(ProposalDTO::from(gov.get_proposal(n)));
        }

        proposals
    })
    .await
    .map_err(|_| error::ErrorInternalServerError("Failed to query proposals from chain"))?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/call-data/{proposal_id}")]
async fn call_data(
    pid: web::Path<u64>,
    data: web::Data<ClientState>,
) -> actix_web::Result<impl Responder> {
    let proposal_id = pid.into_inner();
    let result = web::block(move || {
        let gov = GovernorDeployer::register(data.governor.get_package_hash_address());

        gov.get_call_data(proposal_id)
    })
    .await
    .map_err(|_| error::ErrorInternalServerError("Failed to query call_data from chain"))?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/governor")]
async fn get_governor(data: web::Data<ClientState>) -> actix_web::Result<impl Responder> {
    Ok(HttpResponse::Ok().json(&data.governor))
}

#[get("debug/proposals")]
async fn debug_proposals(_data: web::Data<ClientState>) -> actix_web::Result<impl Responder> {
    let mut proposals = ProposalsDTO::empty();
    for n in 0..=4 {
        proposals.add(ProposalDTO {
            id: n,
            statement: format!("Proposal #{}", n),
            yea: n.try_into().unwrap(),
            nay: 0,
            status: Status::Active,
        });
    }

    Ok(HttpResponse::Ok().json(proposals))
}

struct ClientState {
    governor: DeployedGovernor,
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();

        let governor = DeployedGovernor::load_from_file("./../governor.json");
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(ClientState { governor }))
            .service(get_proposal)
            .service(all_proposals)
            .service(get_governor)
            .service(debug_proposals)
            .service(call_data)
        // .service(get_number)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
