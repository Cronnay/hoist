use bollard::image::{ListImagesOptions, CreateImageOptions};
use bollard::models::{Image, ImageSummary};
use bollard::Docker;
use futures_util::stream::TryStreamExt;


/// Display all images
pub async fn get_all_images(docker: &Docker) -> Result<Vec<ImageSummary>, bollard::errors::Error> {
    let options = ListImagesOptions::<&str> {
        all: true,
        ..Default::default()
    };
    let docker_images = &docker.list_images(Some(options)).await?;
    Ok(docker_images.clone())
}

/// Get Docker image by name
pub async fn get_image_by_name(
    name_of_image: String,
    docker: &Docker,
) -> Result<Image, bollard::errors::Error> {
    let img_info = docker.inspect_image(&name_of_image).await?;
    Ok(img_info)
}

pub async fn pull_image(name_of_image: String, tag: String, docker: &Docker) -> Result<(), bollard::errors::Error> {
    let options = Some(CreateImageOptions {
        from_image: format!("{}:{}", name_of_image, tag),
        ..Default::default()
    });
    docker.create_image(options,None,None).try_collect::<Vec<_>>().await?;
    Ok(())
}