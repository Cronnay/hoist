use bollard::image::ListImagesOptions;
use bollard::models::{Image, ImageSummary};
use bollard::Docker;

/// Display all images
pub async fn get_all_images(docker: &Docker) -> Result<Vec<ImageSummary>, bollard::errors::Error> {
    let mut images: Vec<ImageSummary> = vec![];
    let options = ListImagesOptions::<&str> {
        all: true,
        ..Default::default()
    };
    let docker_images = &docker.list_images(Some(options)).await?;
    for image in docker_images {
        images.push(image.clone());
    }
    Ok(images)
}

/// Get Docker image by name
pub async fn get_image_by_name(
    name_of_image: &str,
    docker: &Docker,
) -> Result<Image, bollard::errors::Error> {
    let img_info = docker.inspect_image(name_of_image).await?;
    Ok(img_info)
}
