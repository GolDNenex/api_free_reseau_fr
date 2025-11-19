use api_free_reseau_fr::{Client, DSLAM, Departement, Error, NRA, Request, Response};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    //
    let client = Client::new();

    let dslams = [
        DSLAM::from("mon75-1"),
        DSLAM::new("mon", 75, 2),
        DSLAM::new("mon", 75, 3),
        DSLAM::new("xyz", 75, 1),
    ];

    let nras = [
        NRA::from("mon75"),
        NRA::new("mar", 75),
        NRA::new("men", 75),
        NRA::new("xyz", 75),
    ];

    let departements = [
        Departement::new(75),
        Departement::new(76),
        Departement::new(77),
        Departement::new(1337),
    ];

    let requests: Vec<Request> = dslams
        .iter()
        .map(Request::from)
        .chain(nras.iter().map(Request::from))
        .chain(departements.iter().map(Request::from))
        .collect();

    println!("\nChecking requests");
    for req in &requests {
        match client.get(req).await {
            Response::NRA {
                target,
                target_status,
            } => {
                println!("NRA {target} is {target_status}");
            }
            Response::DSLAM {
                target,
                target_status,
            } => {
                println!("DSLAM {target} is {target_status}");
            }
            Response::DEPARTEMENT {
                target,
                target_status,
            } => {
                println!("DEPARTEMENT {target} is {target_status}");
            }
            Response::Err { error_message, .. } => {
                eprintln!("ERROR: {error_message}");
            }
        }
    }

    //
    Ok(())
}
