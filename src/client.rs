#[allow(unused_imports)]
use crate::{
    BASE_URL, DSLAM, Departement, Error, NRA, NRAStatus, Response, TargetType, parser,
    request::Request,
};
use crate::{DSLAMStatus, DepartementStatus};

/// API Client. Can be also build from a [`reqwest::Client`]
#[derive(Debug, Default, Clone)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    /// Create a new API Client
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub(crate) async fn _get_status<S: std::fmt::Display>(
        &self,
        target: &S,
        ty: TargetType,
    ) -> Result<bool, Error> {
        let response = self
            .client
            .get(format!("{BASE_URL}/{}", target))
            .send()
            .await?;

        let responsetext = response.text().await?;

        parser::extract_status_from_xml(&responsetext)
            .ok_or(Error::NonExistent(ty, target.to_string()))
    }

    /// Take a [`Departement`] and return a [`DepartementStatus`] or [`enum@Error`]
    /// ```
    /// use api_free_reseau_fr::{Client, Departement, Error, Request};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///
    ///     let target = Departement::new(75);
    ///     let status = client.get_departement_status(&target).await?;
    ///     println!("{target} is {status}");
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_departement_status(
        &self,
        request: &Departement,
    ) -> Result<DepartementStatus, Error> {
        let status = self
            ._get_status(&request.0, TargetType::DEPARTEMENT)
            .await?;
        Ok(status.into())
    }
    /// Take a [`DSLAM`] and return a [`DSLAMStatus`] or [`enum@Error`]
    /// ```
    /// use api_free_reseau_fr::{Client, Request, DSLAM, Error};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///
    ///     let target = DSLAM::new("mon", 75, 2);
    ///     let status = client.get_dslam_status(&target).await?;
    ///     println!("{target} is {status}");
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_dslam_status(&self, request: &DSLAM) -> Result<DSLAMStatus, Error> {
        let status = self._get_status(&request.0, TargetType::DSLAM).await?;
        Ok(status.into())
    }
    /// Take a [`NRA`] and return a [`NRAStatus`] or [`enum@Error`]
    /// ```
    /// use api_free_reseau_fr::{Client, Request, NRA, Error};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///
    ///     let target = NRA::new("mon", 75);
    ///     let status = client.get_nra_status(&target).await?;
    ///     println!("{target} is {status}");
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_nra_status(&self, request: &NRA) -> Result<NRAStatus, Error> {
        let status = self._get_status(&request.0, TargetType::NRA).await?;
        Ok(status.into())
    }

    /// Take a [`Request`] or equivalent and return a [`bool`] or [`enum@Error`]
    /// ```
    /// use api_free_reseau_fr::{Client, Request, NRA, DSLAM, Error};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///
    ///     let target = NRA::new("mon", 75);
    ///     let status = client.get_status(&Request::from(&target)).await?;
    ///     println!("{target} is {status}");
    ///
    ///     let target_2 = Request::from(DSLAM::new("mon", 75, 1));
    ///     let status = client.get_status(&target_2).await?;
    ///     println!("{target} is {status}");
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_status(&self, request: &Request) -> Result<bool, Error> {
        let status = self
            ._get_status(&request.target, request.target_type)
            .await?;
        Ok(status)
    }

    /// Take a [`Request`] or equivalent and return a [`Response`]. Can be use to implement a API endpoint in a http server for example.
    /// ```
    /// use api_free_reseau_fr::{Client, Error, NRA, Request, Response};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///
    ///     let target = Request::from(NRA::new("mon",75));
    ///
    ///     match client.get(&target).await {
    ///         Response::NRA {
    ///             target,
    ///             target_status,
    ///         } => {
    ///             println!("NRA {target} is {target_status}");
    ///         }
    ///         Response::DSLAM {
    ///             target,
    ///             target_status,
    ///         } => {
    ///             println!("DSLAM {target} is {target_status}");
    ///         }
    ///         Response::DEPARTEMENT {
    ///             target,
    ///             target_status,
    ///         } => {
    ///             println!("DEPARTEMENT {target} is {target_status}");
    ///         }
    ///         Response::Err { error_message, .. } => {
    ///             eprintln!("ERROR: {error_message}");
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(&self, request: impl Into<&Request>) -> Response {
        let request = request.into();

        let response = self._get_status(&request.target, request.target_type).await;

        match response {
            Ok(status) => {
                //
                match request.target_type {
                    TargetType::NRA => Response::NRA {
                        target: request.target.to_string(),
                        target_status: status.into(),
                    },
                    TargetType::DSLAM => Response::DSLAM {
                        target: request.target.to_string(),
                        target_status: status.into(),
                    },
                    TargetType::DEPARTEMENT => Response::DEPARTEMENT {
                        target: request.target.to_string(),
                        target_status: status.into(),
                    },
                }
                //
            }
            Err(err) => Response::Err {
                target: request.target.to_string(),
                target_type: request.target_type,
                error_message: err.to_string(),
            },
        }
    }
}

impl From<reqwest::Client> for Client {
    fn from(client: reqwest::Client) -> Self {
        Self { client }
    }
}
