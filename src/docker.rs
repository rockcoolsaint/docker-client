use std::env;

use bollard::{Docker, errors::Error, models::{ContainerSummary, ImageSummary}, query_parameters::{CreateImageOptionsBuilder, ListContainersOptionsBuilder, ListImagesOptionsBuilder, StartContainerOptionsBuilder, StopContainerOptionsBuilder}};
use futures_util::TryStreamExt;

pub struct DockerClient {
    docker: Docker,
}

impl DockerClient {
    pub fn new() -> Self {
        let docker = Docker::connect_with_unix(&env::var("UNIX_SOCKET_PATH").unwrap_or_default(),
        120, //Timeout in seconds
        bollard::API_DEFAULT_VERSION
        ).expect("Failed to connect to Docker");

        Self { docker: docker }
    }

    pub async fn list_containers(&self, all: bool) -> Result<Vec<ContainerSummary>, Error> {
        let options = Some(ListContainersOptionsBuilder::default()
            .all(all)
            .build()
        );

        let containers = self.docker.list_containers(options).await?;
        Ok(containers)
    }

    pub async fn list_images(&self) -> Result<Vec<ImageSummary>, Error> {
        let options = Some(ListImagesOptionsBuilder::default()
            .all(true)
            .build()
        );

        let images = self.docker.list_images(options).await?;
        Ok(images)
    }

    pub async fn start_container(&self, container_name: &str) -> Result<(), Error> {
        self.docker
            .start_container(container_name, Some(StartContainerOptionsBuilder::default()
            .build()))
            .await?;
        Ok(())
    }

    pub async fn stop_container(&self, container_name: &str) -> Result<(), Error> {
        let options = Some(StopContainerOptionsBuilder::default()
            .t(30)
            .build()
        );

        self.docker
            .stop_container(container_name, options)
            .await?;
        Ok(())
    }

    pub async fn pull_image(&self, image_name: &str) -> Result<(), Error> {
        let options = Some(CreateImageOptionsBuilder::default()
            .from_image(image_name)
            .build()
        );

        let mut stream = self.docker
            .create_image(options, None, None);

        while let Some(msg) = stream.try_next().await? {
            if let Some(status) = msg.status {
                println!("{:?}", status);
            }
        }
        Ok(())
    }
}
