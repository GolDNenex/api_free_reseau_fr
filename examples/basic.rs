use api_free_reseau_fr::*;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let client = Client::new();

    let _target = DSLAM::from("mon75-1");
    let target = DSLAM::new("mon", 75, 1);

    let status = client.get_dslam_status(&target).await?;
    println!("{target} is {status}");

    let target_2 = Request::from(NRA::from("mon75"));
    let status = client.get_status(&target_2).await?;
    println!("{target} is {status}");

    Ok(())
}
