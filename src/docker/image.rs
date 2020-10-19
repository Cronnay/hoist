use bollard::image::ListImagesOptions;
use bollard::models::ImageSummary;
use bollard::Docker;

pub async fn get_all_images(
    docker: &Docker,
) -> Result<Vec<ImageSummary>, Box<dyn std::error::Error + 'static>> {
    let mut images: Vec<ImageSummary> = vec![];
    let docker_images = &docker
        .list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();

    for image in docker_images {
        images.push(image.clone());
    }
    Ok(images)
}
